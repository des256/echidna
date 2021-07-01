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
        collections::HashMap,
        time::Duration,
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
    pub subs: Mutex<HashMap<SubscriberId,Arc<SubscriberControl>>>,
    pub tasks: Mutex<Vec<task::JoinHandle<()>>>,
}

impl Publisher {
    pub async fn new(pubsub_port: u16,domain: &str,topic: &str) -> Arc<Publisher> {

        // new ID
        let id = rand::random::<u64>();

        // create publisher
        let publisher = Arc::new(Publisher {
            id: id,
            domain: domain.to_string(),
            topic: topic.to_string(),
            subs: Mutex::new(HashMap::new()),  // subscribers as maintained by the participant
            tasks: Mutex::new(Vec::new()),
        });

        // spawn participant receiver
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_participant_connection(pubsub_port).await;
        });

        println!("publisher {:016X} of \"{}\" running",id,topic);
        
        publisher
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
            time::sleep(Duration::from_secs(5)).await;

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

        // cancel current tasks
        {
            let mut tasks = self.tasks.lock().await;
            for task in tasks.iter() {
                task.abort();
            }
            *tasks = Vec::new();
        }

        // calculate number of chunks for this message
        let total_bytes = message.len();
        let mut total = total_bytes / CHUNK_SIZE;
        if (total_bytes % CHUNK_SIZE) != 0 {
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
                if (offset + CHUNK_SIZE) > total_bytes {
                    total_bytes - offset
                }
                else {
                    CHUNK_SIZE
                }
            };
            let chunk = Chunk {
                ts: 0,
                id: id,
                total_bytes: total_bytes as u64,
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

        // spawn task for each subscriber
        let mut tasks = Vec::<task::JoinHandle<()>>::new();
        for (_,subscriber) in subs.iter() {
            let chunks = Arc::clone(&master_chunks);
            let subscriber = Arc::clone(&subscriber);
            tasks.push(task::spawn(async move {

                let start_time = time::Instant::now();

                let mut last = 0usize;
                let mut indices = Vec::<usize>::new();
                while last < total {

                    // top up transmit list
                    while (indices.len() < CHUNKS_PER_HEARTBEAT) && (last < total) {
                        indices.push(last);
                        last += 1;
                    }

                    // send chunks
                    for index in indices.iter() {
                        //println!("{:?}: send {}",time::Instant::now(),index);
                        subscriber.socket.send_to(&chunks[*index],subscriber.address).await.expect("error sending chunk");
                    }

                    // send heartbeat
                    //println!("{:?}: send heartbeat",time::Instant::now());
                    let mut send_buffer = Vec::<u8>::new();
                    PublisherToSubscriber::Heartbeat(id).encode(&mut send_buffer);
                    subscriber.socket.send_to(&send_buffer,subscriber.address).await.expect("error sending heartbeat");

                    // wait for acknowledgement or RTO timeout
                    let mut buffer = vec![0u8; 65536];
                    let rto = Duration::from_micros(RETRANSMIT_TIMEOUT_USEC);
                    if let Ok(_) = time::timeout(rto,subscriber.socket.recv_from(&mut buffer)).await {
                        if let Some((_,stp)) = SubscriberToPublisher::decode(&buffer) {
                            match stp {
                                SubscriberToPublisher::Ack(message_id,acks) => {
                                    if message_id == id {
                                        //println!("{:?}: received acknowledgements for:",time::Instant::now());
                                        for ack in acks {
                                            //println!("    {}",ack);
                                            for i in 0..indices.len() {
                                                if indices[i] == ack as usize {
                                                    indices.remove(i);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                },
                            }
                        }
                    }
                    else {
                        //println!("retransmit timeout");
                    }

                    // TODO: measure RTT and recalculate RTO
                }

                let end_time = time::Instant::now();
                let duration = end_time - start_time;

                let mbps = ((total_bytes * 1000) as u128) / duration.as_nanos();

                println!("transmitted in {:?}ns ({} MB/s) to {}",duration.as_nanos(),mbps,subscriber.address);
            }));
        }

        // keep the tasks
        *self.tasks.lock().await = tasks;
    }
}