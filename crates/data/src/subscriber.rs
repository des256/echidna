// Echidna - Data

use {
    crate::*,
    tokio::{
        net,
        task,
        io::AsyncReadExt,
        sync::Mutex,
        time,
    },
    codec::Codec,
    std::{
        sync::Arc,
        net::SocketAddr,
        time::Duration,
    },
};

pub struct SubscriberState {
    id: DataId,
    buffer: Vec<u8>,
    received: Vec<bool>,
}

pub struct Subscriber {
    pub id: PubId,
    pub topic: String,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
    pub state: Mutex<SubscriberState>,
}

impl Subscriber {
    pub async fn new(topic: &str,on_data: impl Fn(&[u8]) + Send + 'static) -> Arc<Subscriber> {

        // new ID
        let id = rand::random::<u64>();

        // open data socket
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create subscriber socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        // create subscriber
        let subscriber = Arc::new(Subscriber {
            id: id,
            topic: topic.to_string(),
            socket: socket,
            address: address,
            state: Mutex::new(SubscriberState {
                id: 0,
                buffer: Vec::new(),
                received: Vec::new(),
            }),
        });

        // spawn participant receiver
        let this = Arc::clone(&subscriber);
        task::spawn(async move {
            this.run_participant_connection().await;
        });

        // spawn socket receiver
        let this = Arc::clone(&subscriber);
        task::spawn(async move {
            this.run_socket_receiver(on_data).await;
        });

        println!("subscriber {:016X} of \"{}\" running at port {}",id,topic,address.port());

        subscriber
    }

    pub async fn run_participant_connection(self: &Arc<Subscriber>) {

        loop {

            // connect to participant
            if let Ok(mut stream) = net::TcpStream::connect("0.0.0.0:7332").await {

                // announce subscriber to participant
                send_message(&mut stream,ToPart::InitSub(self.id,SubRef {
                    address: self.address,
                    topic: self.topic.clone(),
                })).await;

                // receive participant messages
                let mut recv_buffer = vec![0u8; 65536];
                while let Ok(length) = stream.read(&mut recv_buffer).await {
                    if length == 0 {
                        break;
                    }
                    if let Some((_,message)) = PartToSub::decode(&recv_buffer) {
                        match message {
                            PartToSub::Init => { },
                            PartToSub::InitFailed => {
                                panic!("publisher initialization failed!");
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

    pub async fn run_socket_receiver(self: &Arc<Subscriber>,on_data: impl Fn(&[u8]) + Send + 'static) {

        let mut buffer = vec![0u8; 65536];

        loop {

            // receive heartbeat or chunk
            let (_,address) = self.socket.recv_from(&mut buffer).await.expect("error receiving");

            if let Some((_,pts)) = PubToSub::decode(&buffer) {

                match pts {

                    // Heartbeat
                    PubToSub::Heartbeat(id) => {

                        let state = self.state.lock().await;

                        // only respond if this is for the current message
                        if id == state.id {

                            println!("received heartbeat");

                            // collect missing indices
                            let mut indices = Vec::<u32>::new();
                            for i in 0..state.received.len() {
                                if !state.received[i] {
                                    indices.push(i as u32);
                                }
                            }

                            // request resend
                            if indices.len() != 0 {
                                println!("requesting resends for:");
                                for index in indices.iter() {
                                    println!("    {}",index);
                                }
                                let mut send_buffer = Vec::<u8>::new();
                                SubToPub::Resend(self.id,id,indices).encode(&mut send_buffer);
                                self.socket.send_to(&mut send_buffer,address).await.expect("error sending acknowledgement");
                            }
                        }
                    },

                    // chunk
                    PubToSub::Chunk(chunk) => {

                        let mut state = self.state.lock().await;

                        println!("received chunk {} for message {:016X}",chunk.index,chunk.id);

                        // if this is a new chunk, reset state
                        if chunk.id != state.id {
                            state.id = chunk.id;
                            state.buffer = vec![0; chunk.total_bytes as usize];
                            state.received = vec![false; chunk.total as usize];
                        }

                        // copy data into final message buffer
                        let start = chunk.index as usize * CHUNK_SIZE;
                        let end = start + chunk.data.len();
                        state.buffer[start..end].copy_from_slice(&chunk.data);

                        // mark the chunk as received
                        state.received[chunk.index as usize] = true;

                        // verify if all chunks are received
                        let mut complete = true;
                        for received in state.received.iter() {
                            if !received {
                                complete = false;
                                break;
                            }
                        }

                        // if all chunks received, pass to callback
                        if complete {
                            println!("all chunks received");
                            on_data(&state.buffer);
                        }
                    },
                }
            }
            else {
                println!("message error");
            }
        }
    }
}