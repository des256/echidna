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
    pub done: bool,
}

pub struct PublisherState {
    pub id: MessageId,
    pub chunks: Vec<Vec<u8>>,
    pub subs: HashMap<SubscriberId,SubscriberRef>,
}

pub struct Publisher {
    pub id: PublisherId,
    pub topic: String,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
    pub subs: Mutex<HashMap<SubscriberId,SubscriberRef>>,
    pub success: Mutex<HashMap<SubscriberId,Vec<bool>>>,
    pub state: Mutex<PublisherState>,
}

impl Publisher {
    pub async fn new(topic: &str) -> Arc<Publisher> {

        // new ID
        let id = rand::random::<u64>();

        // open data socket
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create publisher socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        // create publisher
        let publisher = Arc::new(Publisher {
            id: id,
            topic: topic.to_string(),
            socket: socket,
            address: address,
            subs: Mutex::new(HashMap::new()),
            success: Mutex::new(HashMap::new()),
            state: Mutex::new(PublisherState {
                id: 0,
                chunks: Vec::new(),
                subs: HashMap::new(),
            }),
        });

        // spawn participant receiver
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_participant_connection().await;
        });

        // spawn acknowledgement handler
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_ack_handler().await;
        });

        println!("publisher {:016X} of \"{}\" running at port {}",id,topic,address.port());
        
        publisher
    }

    pub async fn run_participant_connection(self: &Arc<Publisher>) {

        loop {

            // connect to participant
            if let Ok(mut stream) = net::TcpStream::connect("0.0.0.0:7332").await {

                // announce publisher to participant
                send_message(&mut stream,ToParticipant::InitPub(self.id,PublisherRef {
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
                                *state_subs = subs;
                                for (id,s) in state_subs.iter() {
                                    println!("subscriber {:016X} found at {}",id,s.address);
                                }
                            },
                            ParticipantToPublisher::InitFailed => {
                                panic!("publisher initialization failed!");
                            },
                            ParticipantToPublisher::NewSub(id,subscriber) => {
                                println!("subscriber {:016X} found at {}",id,subscriber.address);
                                let mut state_subs = self.subs.lock().await;
                                state_subs.insert(id,subscriber);
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

        // TODO: cancel whatever send is currently happening

        // copy current subscriber state
        {
            let state_subs = self.subs.lock().await;
            let mut state = self.state.lock().await;
            state.subs = state_subs.clone();
        }

        // calculate number of chunks for this message
        let total_bytes = message.len();
        let mut total = (total_bytes / CHUNK_SIZE) as u32;
        if (total_bytes % CHUNK_SIZE) != 0 {
            total += 1;
        }
        println!("sending message of {} bytes in {} chunks",total_bytes,total);
        
        // prepare chunks
        let id = rand::random::<u64>();

        {
            let mut state = self.state.lock().await;
            let mut success = self.success.lock().await;

            // initialize
            state.id = id;
            state.chunks = Vec::new();

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
                    total: total,
                    index: index,
                    data: Vec::<u8>::from(&message[offset..offset + size]),
                };

                // encode
                let mut buffer = Vec::<u8>::new();
                PublisherToSubscriber::Chunk(chunk).encode(&mut buffer);

                // store
                state.chunks.push(buffer);

                // next
                offset += size;
                index += 1;
            }

            // initialize success loggers
            success.clear();
            for (id,_) in state.subs.iter() {
                success.insert(*id,vec![false; total as usize]);
            }
        }

        // send all chunks
        println!("sending a bunch of chunks to all subscribers:");
        {
            let state = self.state.lock().await;
            let mut index = 0usize;
            for send_buffer in state.chunks.iter() {
                println!("sending chunk {} to all subscribers",index);
                for (_,s) in state.subs.iter() {
                    self.socket.send_to(send_buffer,s.address).await.expect("error sending data chunk");
                }
                index += 1;
                if index >= CHUNKS_PER_INITIAL_BURST {
                    break;
                }
            }
        }

        // retransmit missing chunks
        println!("done, retransmiting now...");

        // make copy of subscriber list
        let mut subs = HashMap::<SubscriberId,SubscriberControl>::new();
        {
            let state = self.state.lock().await;
            for (sid,s) in state.subs.iter() {
                subs.insert(*sid,SubscriberControl {
                    address: s.address,
                    done: false,
                });
            }
        }
        let mut done = false;
        while !done {

            // wait a really tiny bit before trying again
            time::sleep(Duration::from_micros(RETRANSMIT_DELAY_USEC as u64)).await;

            // send heartbeat to all subscribers that still need one
            let mut send_buffer = Vec::<u8>::new();
            PublisherToSubscriber::Heartbeat(id).encode(&mut send_buffer);
            for (_,s) in subs.iter() {
                if !s.done {
                    self.socket.send_to(&send_buffer,s.address).await.expect("error sending heartbeat");
                }
            }

            // if everything was sent, call it a day
            done = true;
            let state_success = self.success.lock().await;
            for (_,success) in state_success.iter() {
                for s in success {
                    if !s {
                        done = false;
                        break;
                    }
                }
                if !done {
                    break;
                }
            }
        }

        println!("everything sent successfully");
    }

    pub async fn run_ack_handler(self: &Arc<Publisher>) {

        let mut buffer = vec![0u8; 65536];

        loop {

            let (_,address) = self.socket.recv_from(&mut buffer).await.expect("error receiving");

            if let Some((_,stp)) = SubscriberToPublisher::decode(&buffer) {
                match stp {

                    SubscriberToPublisher::Ack(sid,id,indices) => {

                        let state = self.state.lock().await;
                        if id == state.id {

                            let mut state_success = self.success.lock().await;
                            let success = state_success.get_mut(&sid).unwrap();
    
                            println!("{:016X} acknowledges chunks:",sid);
                            for index in indices.iter() {
                                println!("    {}",index);
                                success[*index as usize] = true;
                            }
                        }
                    },

                    SubscriberToPublisher::NAck(sid,id,indices) => {

                        let state = self.state.lock().await;
                        if id == state.id {

                            println!("{:016X} wants retransmit of chunks:",sid);
                            for index in indices.iter() {
                                println!("    {}",index);
                                println!("retransmitting chunk {} to {:016X}",index,sid);
                                self.socket.send_to(&state.chunks[*index as usize],address).await.expect("error retransmitting chunk");
                            }
                        }
                    },
                }
            }
        }
    }
}