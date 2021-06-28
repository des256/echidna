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
    id: MessageId,
    buffer: Vec<u8>,
    received: Vec<bool>,
}

pub struct Subscriber {
    pub id: PublisherId,
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
                send_message(&mut stream,ToParticipant::InitSub(self.id,SubscriberRef {
                    address: self.address,
                    topic: self.topic.clone(),
                })).await;

                // receive participant messages
                let mut recv_buffer = vec![0u8; 65536];
                while let Ok(length) = stream.read(&mut recv_buffer).await {
                    if length == 0 {
                        break;
                    }
                    if let Some((_,message)) = ParticipantToSubscriber::decode(&recv_buffer) {
                        match message {
                            ParticipantToSubscriber::Init => { },
                            ParticipantToSubscriber::InitFailed => {
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

        let mut ack_indices = Vec::<u32>::new();

        loop {

            // receive heartbeat or chunk
            let (_,address) = self.socket.recv_from(&mut buffer).await.expect("error receiving");

            if let Some((_,pts)) = PublisherToSubscriber::decode(&buffer) {

                match pts {

                    // Heartbeat, respond with Ack
                    PublisherToSubscriber::Heartbeat(id) => {

                        let state = self.state.lock().await;

                        // only respond if this is for the current message
                        if id == state.id {

                            println!("received heartbeat");

                            // send acknowledgements back
                            println!("sending acknowledgements:");
                            for index in ack_indices.iter() {
                                println!("    {}",index);
                            }
                            let mut send_buffer = Vec::<u8>::new();
                            SubscriberToPublisher::Ack(id,ack_indices).encode(&mut send_buffer);
                            self.socket.send_to(&mut send_buffer,address).await.expect("error sending retransmit request");
                            ack_indices = Vec::new();
                        }
                    },

                    // chunk
                    PublisherToSubscriber::Chunk(chunk) => {

                        let mut state = self.state.lock().await;

                        // if this is a new chunk, reset state
                        if chunk.id != state.id {
                            state.id = chunk.id;
                            state.buffer = vec![0; chunk.total_bytes as usize];
                            state.received = vec![false; chunk.total as usize];
                            ack_indices.clear();
                        }

                        println!("received chunk {}",chunk.index);

                        ack_indices.push(chunk.index);

                        // if we don't already have this chunk
                        if !state.received[chunk.index as usize] {

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