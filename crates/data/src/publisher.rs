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

pub struct PublisherState {
    pub id: DataId,
    pub chunks: Vec<Vec<u8>>,
    pub success: HashMap<SubId,Vec<bool>>,
}

pub struct Publisher {
    pub id: PubId,
    pub topic: String,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
    pub subs: Mutex<HashMap<SubId,SubRef>>,
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
            state: Mutex::new(PublisherState {
                id: 0,
                chunks: Vec::new(),
                success: HashMap::new(),
            }),
        });

        // spawn participant receiver
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_participant_connection().await;
        });

        // spawn resend request handler
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_resend_handler().await;
        });

        println!("publisher {:016X} of \"{}\" running at port {}",id,topic,address.port());
        
        publisher
    }

    pub async fn run_participant_connection(self: &Arc<Publisher>) {

        loop {

            // connect to participant
            if let Ok(mut stream) = net::TcpStream::connect("0.0.0.0:7332").await {

                // announce publisher to participant
                send_message(&mut stream,ToPart::InitPub(self.id,PubRef {
                    topic: self.topic.clone(),
                })).await;

                // receive participant messages
                let mut recv_buffer = vec![0u8; 65536];
                while let Ok(length) = stream.read(&mut recv_buffer).await {
                    if length == 0 {
                        break;
                    }
                    if let Some((_,message)) = PartToPub::decode(&recv_buffer) {
                        match message {
                            PartToPub::Init(subs) => {
                                let mut state_subs = self.subs.lock().await;
                                *state_subs = subs;
                                for (id,s) in state_subs.iter() {
                                    println!("subscriber {:016X} found at {}",id,s.address);
                                }
                            },
                            PartToPub::InitFailed => {
                                panic!("publisher initialization failed!");
                            },
                            PartToPub::NewSub(id,subscriber) => {
                                println!("subscriber {:016X} found at {}",id,subscriber.address);
                                let mut state_subs = self.subs.lock().await;
                                state_subs.insert(id,subscriber);
                            },
                            PartToPub::DropSub(id) => {
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

        // calculate number of chunks for this message
        let total_bytes = message.len();
        let mut total = (total_bytes / CHUNK_SIZE) as u32;
        if (total_bytes % CHUNK_SIZE) != 0 {
            total += 1;
        }
        
        // prepare new message
        let id = rand::random::<u64>();

        {
            let mut state = self.state.lock().await;

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
                PubToSub::Chunk(chunk).encode(&mut buffer);

                // store
                state.chunks.push(buffer);

                // next
                offset += size;
                index += 1;
            }

            // initialize success loggers
            state.success = HashMap::new();
            let state_subs = self.subs.lock().await;
            for (id,_) in state_subs.iter() {
                state.success.insert(*id,vec![false; total as usize]);
            }
        }

        // send all chunks to all subscribers once
        {
            let state = self.state.lock().await;
            let state_subs = self.subs.lock().await;
            for send_buffer in state.chunks.iter() {
                for (_,s) in state_subs.iter() {
                    self.socket.send_to(send_buffer,s.address).await.expect("error sending data chunk");
                }
            }
        }

        // send heartbeats until everything is transmitted successfully
        let mut done = true;
        while !done {

            let state = self.state.lock().await;
            let state_subs = self.subs.lock().await;

            let mut send_buffer = Vec::<u8>::new();
            PubToSub::Heartbeat(id).encode(&mut send_buffer);

            for (sid,success) in state.success.iter() {

                // figure out if everything was sent
                let mut complete = true;
                for s in success.iter() {
                    if !s {
                        complete = false;
                        break;
                    }
                }

                // if not, send heartbeat to this subscriber
                if !complete {
                    let s = state_subs.get(&sid).unwrap();
                    self.socket.send_to(&send_buffer,s.address).await.expect("error sending heartbeat");
                    done = false;
                }
            }

            // and wait a really tiny bit before trying again
            time::sleep(Duration::from_micros(1)).await;
        }
    }

    pub async fn run_resend_handler(self: &Arc<Publisher>) {

        let mut buffer = vec![0u8; 65536];

        loop {

            // receive resend request
            let (_,address) = self.socket.recv_from(&mut buffer).await.expect("error receiving");

            if let Some((_,stp)) = SubToPub::decode(&buffer) {
                match stp {

                    SubToPub::Resend(sid,id,indices) => {

                        let mut state = self.state.lock().await;

                        // make sure this is about the current message
                        if id == state.id {

                            // log which chunks were successfully transmitted
                            let total = state.chunks.len();
                            let success = state.success.get_mut(&sid).unwrap();
                            *success = vec![true; total];
                            for index in indices.iter() {
                                if *index < total as u32 {
                                    success[*index as usize] = false;
                                }
                            }

                            // retransmit requested chunks
                            for index in indices.iter() {
                                self.socket.send_to(&state.chunks[*index as usize],address).await.expect("error resending data chunk");
                            }
                        }
                    },
                }
            }
        }
    }
}