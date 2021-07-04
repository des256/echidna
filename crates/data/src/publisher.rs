// Echidna - Data

use {
    crate::*,
    codec::Codec,
    tokio::{
        net,
        task,
        io::AsyncReadExt,
        sync::Mutex,
        time,
    },
    std::{
        sync::Arc,
        net::SocketAddr,
        collections::{
            HashMap,
            HashSet,
        },
    },
};

pub struct SubscriberControl {
    pub address: SocketAddr,
    pub socket: net::UdpSocket,
}

pub struct Publisher {
    pub id: PublisherId,
    pub domain: String,
    pub topic: String,
    pub chunk_size: usize,
    pub chunks_per_heartbeat: usize,
    pub transmit_interval_usec: u64,
    pub intervals_before_heartbeat: usize,
    pub dead_counter_intervals: usize,
    pub favor_incoming: bool,
    pub subs: Mutex<HashMap<SubscriberId,Arc<SubscriberControl>>>,
    pub tasks: Mutex<HashMap<SubscriberId,task::JoinHandle<()>>>,
    pub finished: Arc<Mutex<HashMap<SubscriberId,bool>>>,
}

impl Publisher {
    pub async fn new(
        pubsub_port: u16,
        domain: &str,
        topic: &str,
        chunk_size: usize,
        chunks_per_heartbeat: usize,
        transmit_interval_usec: u64,
        intervals_before_heartbeat: usize,
        dead_counter_intervals: usize,
        favor_incoming: bool
    ) -> Arc<Publisher> {

        // new ID
        let id = rand::random::<u64>();

        // create publisher
        let publisher = Arc::new(Publisher {
            id: id,
            domain: domain.to_string(),
            topic: topic.to_string(),
            chunk_size: chunk_size,
            chunks_per_heartbeat: chunks_per_heartbeat,
            transmit_interval_usec: transmit_interval_usec,
            intervals_before_heartbeat: intervals_before_heartbeat,
            dead_counter_intervals: dead_counter_intervals,
            favor_incoming: favor_incoming,
            subs: Mutex::new(HashMap::new()),  // subscribers as maintained by the participant
            tasks: Mutex::new(HashMap::new()),
            finished: Arc::new(Mutex::new(HashMap::new())),
        });

        // spawn participant receiver
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_participant_connection(pubsub_port).await;
        });

        println!("publisher {:016X} of \"{}\" running",id,topic);
        
        publisher
    }

    pub async fn new_default(
        pubsub_port: u16,
        domain: &str,
        topic: &str
    ) -> Arc<Publisher> {
        Publisher::new(pubsub_port,domain,topic,32768,4,300,0,100,false).await
    }

    pub async fn run_participant_connection(self: &Arc<Publisher>,pubsub_port: u16) {

        loop {

            // connect to participant
            if let Ok(mut stream) = net::TcpStream::connect(format!("0.0.0.0:{}",pubsub_port)).await {

                // announce publisher to participant
                send_message(&mut stream,ToParticipant::InitPub(self.id,self.domain.clone(),PublisherRef {
                    topic: self.topic.clone(),
                })).await;

                // receive participant messages
                let mut recv_buffer = vec![0u8; 65536];
                while let Ok(length) = stream.read(&mut recv_buffer).await {
                    if length == 0 {
                        break;
                    }
                    if let Some((_,message)) = ParticipantToPublisher::decode(&recv_buffer) {
                        match message {
                            ParticipantToPublisher::Init(subs) => {
                                let mut state_subs = self.subs.lock().await;
                                for(id,s) in subs.iter() {
                                    println!("subscriber {:016X} found at {}",id,s.address);
                                    state_subs.insert(*id,Arc::new(SubscriberControl {
                                        address: s.address,
                                        socket: net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create publisher socket"),
                                    }));
                                }
                            },
                            ParticipantToPublisher::InitFailed(reason) => {
                                match reason {
                                    PubInitFailed::DomainMismatch => { println!("Publisher initialization failed: domain mismatch."); },
                                }
                            },
                            ParticipantToPublisher::NewSub(id,subscriber) => {
                                println!("subscriber {:016X} found at {}",id,subscriber.address);
                                let mut state_subs = self.subs.lock().await;
                                state_subs.insert(id,Arc::new(SubscriberControl {
                                    address: subscriber.address,
                                    socket: net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create publisher socket"),
                                }));
                            },
                            ParticipantToPublisher::DropSub(id) => {
                                let mut state_subs = self.subs.lock().await;
                                state_subs.remove(&id);
                                println!("subscriber {:016X} lost",id);
                            },
                        }
                    }
                }
                println!("participant lost...");
            }
            else {
                println!("could not connect to participant...");
            }

            // wait for a few seconds before trying again
            time::sleep(time::Duration::from_secs(5)).await;

            println!("attempting connection again.");
        }
    }

    pub async fn send(self: &Arc<Publisher>,message: &[u8]) {

        // if there are no subscribers, ignore
        {
            let state_subs = self.subs.lock().await;
            if state_subs.len() == 0 {
                return;
            }
        }

        // if new incoming value is more important (for slow updates), cancel current message
        if self.favor_incoming {
            {
                let mut tasks = self.tasks.lock().await;
                for (_,task) in tasks.iter() {
                    task.abort();
                }
                *tasks = HashMap::new();
            }
        }

        // otherwise, if still transmitting a message, let that message be transmitted fully
        else {
            let mut all_done = true;
            let finished = self.finished.lock().await;
            for (_,done) in finished.iter() {
                if !done {
                    all_done = false;
                    break;
                }
            }
            if !all_done {
                return;
            }
        }

        // calculate number of chunks for this message
        let total_bytes = message.len();
        let mut total = total_bytes / self.chunk_size;
        if (total_bytes % self.chunk_size) != 0 {
            total += 1;
        }
        println!("sending message of {} bytes in {} chunks",total_bytes,total);
        
        // prepare chunks
        let id = rand::random::<u64>();
        let mut chunks = Vec::new();

        // build chunks
        let mut index = 0u32;
        let mut offset = 0usize;
        while offset < total_bytes {

            // create chunk
            let size = {
                if (offset + self.chunk_size) > total_bytes {
                    total_bytes - offset
                }
                else {
                    self.chunk_size
                }
            };
            let chunk = Chunk {
                ts: 0,
                id: id,
                total_bytes: total_bytes as u64,
                chunk_size: self.chunk_size as u32,
                total: total as u32,
                index: index,
                data: Vec::<u8>::from(&message[offset..offset + size]),
            };

            // encode
            let mut buffer = Vec::<u8>::new();
            PublisherToSubscriber::Chunk(chunk).encode(&mut buffer);

            // store
            chunks.push(buffer);

            // next
            offset += size;
            index += 1;
        }
        let master_chunks = Arc::new(chunks);

        // take snapshot of current subscriber set
        let subs = self.subs.lock().await.clone();

        // clear finished flags
        {
            let mut new_finished = HashMap::<SubscriberId,bool>::new();
            for (subscriber_id,_) in subs.iter() {
                new_finished.insert(*subscriber_id,false);
            }
            let mut finished = self.finished.lock().await;
            *finished = new_finished;
        }

        // spawn send and recv tasks for each subscriber
        let mut send_tasks = HashMap::<SubscriberId,task::JoinHandle<()>>::new();
        for (subscriber_id,control) in subs.iter() {

            let sub_id = *subscriber_id;
            let chunks = Arc::clone(&master_chunks);
            let control = Arc::clone(&control);
            let finished = Arc::clone(&self.finished);

            let chunks_per_heartbeat = self.chunks_per_heartbeat;
            let intervals_before_heartbeat = self.intervals_before_heartbeat;
            let transmit_interval_usec = self.transmit_interval_usec;
            let dead_counter_intervals = self.dead_counter_intervals;

            send_tasks.insert(sub_id,task::spawn(async move {

                let mut dones = vec![false; chunks.len()];
                let mut retransmits = HashSet::<u32>::new();
                let mut last = 0usize;
                let mut dead_counter = 0usize;
                let mut done = false;
                let mut interval = time::interval(time::Duration::from_micros(transmit_interval_usec));

                while !done {

                    let mut indices = Vec::<u32>::new();

                    // first the retransmits
                    {
                        for index in retransmits.iter() {
                            //println!("send retransmit {}",index);
                            indices.push(*index);
                            if indices.len() >= chunks_per_heartbeat {
                                break;
                            }
                        }
                        for index in indices.iter() {
                            retransmits.remove(index);
                        }
                    }

                    // fill up what's left with remaining chunks
                    while (indices.len() < chunks_per_heartbeat) && (last < total) {
                        //println!("send regular {}",last);
                        indices.push(last as u32);
                        last += 1;
                    }

                    // if no items in the buffer yet, fill up with chunks left undone
                    if indices.len() == 0 {
                        for i in 0..dones.len() {
                            if !dones[i] {
                                //println!("send leftover {}",i);
                                indices.push(i as u32);
                                if indices.len() >= chunks_per_heartbeat {
                                    break;
                                }
                            }
                        }
                    }

                    // send chunks
                    for index in indices.iter() {
                        control.socket.send_to(&chunks[*index as usize],control.address).await.expect("error sending chunk");
                        interval.tick().await;
                    }

                    // wait before sending heartbeat
                    for _ in 0..intervals_before_heartbeat {
                        interval.tick().await;
                    }

                    // send heartbeat
                    //println!("send heartbeat");
                    let mut send_buffer = Vec::<u8>::new();
                    PublisherToSubscriber::Heartbeat(id).encode(&mut send_buffer);
                    control.socket.send_to(&send_buffer,control.address).await.expect("error sending heartbeat");

                    // flush incoming acks and nacks
                    // TODO: it's currently not exactly flushing, but rather processing at most one message
                    let mut buffer = vec![0u8; 65536];

                    if let Err(_) = time::timeout(time::Duration::from_micros(transmit_interval_usec),control.socket.recv_from(&mut buffer)).await {
                        dead_counter += 1;
                    }
                    else {
        
                        if let Some((_,stp)) = SubscriberToPublisher::decode(&buffer) {
        
                            match stp {
        
                                SubscriberToPublisher::Ack(message_id,index) => {
        
                                    // subscriber has everything until index
        
                                    if message_id == id {
                                        //println!("received ack {}",index);
        
                                        // mark received chunks
                                        for i in 0..index {
                                            dones[i as usize] = true;
                                        }
                                    }
        
                                },
        
                                SubscriberToPublisher::NAck(message_id,first,last) => {
        
                                    // subscriber is missing first..last
        
                                    if message_id == id {
                                        //println!("received nack {}-{}",first,last);
        
                                        // mark received chunks
                                        for i in 0..first {
                                            dones[i as usize] = true;
                                        }
        
                                        // and notice the retransmits
                                        for index in first..last {
                                            retransmits.insert(index);
                                        }
                                    }
                                },
                            }
                        }
                    }

                    // if subscriber hasn't responded for a specific time, exit loop
                    if dead_counter >= dead_counter_intervals {
                        println!("subscriber died.");
                        break;
                    }

                    // verify we sent everything
                    {
                        done = true;
                        for i in 0..dones.len() {
                            if !dones[i] {
                                done = false;
                                break;
                            }
                        }
                    }
                }

                if done {
                    println!("done, dead counter {}.",dead_counter);
                }

                let mut finished = finished.lock().await;
                *finished.get_mut(&sub_id).unwrap() = true;

            }));
        }

        // keep the tasks
        *self.tasks.lock().await = send_tasks;
    }
}
