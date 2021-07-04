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
    pub domain: String,
    pub topic: String,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
    pub state: Mutex<SubscriberState>,
}

impl Subscriber {
    pub async fn new(pubsub_port: u16,domain: &str,topic: &str,on_data: impl Fn(&[u8]) + Send + 'static) -> Arc<Subscriber> {

        // new ID
        let id = rand::random::<u64>();

        // open data socket
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create subscriber socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        // create subscriber
        let subscriber = Arc::new(Subscriber {
            id: id,
            domain: domain.to_string(),
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
            this.run_participant_connection(pubsub_port).await;
        });

        // spawn socket receiver
        let this = Arc::clone(&subscriber);
        task::spawn(async move {
            this.run_socket_receiver(on_data).await;
        });

        println!("subscriber {:016X} of \"{}\" running at port {}",id,topic,address.port());

        subscriber
    }

    pub async fn run_participant_connection(self: &Arc<Subscriber>,pubsub_port: u16) {

        loop {

            // connect to participant
            if let Ok(mut stream) = net::TcpStream::connect(format!("0.0.0.0:{}",pubsub_port)).await {

                // announce subscriber to participant
                send_message(&mut stream,ToParticipant::InitSub(self.id,self.domain.clone(),SubscriberRef {
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
                            ParticipantToSubscriber::InitFailed(reason) => {
                                match reason {
                                    SubInitFailed::DomainMismatch => { println!("Subscriber initialization failed: domain mismatch."); },
                                }
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

        let mut start_time = time::Instant::now();

        let mut buffer = vec![0u8; 65536];

        let mut first_missing = 0u32;
        let mut last_missing: Option<u32> = None;

        let mut chunks_total = 0usize;
        let mut chunks_ignored = 0usize;

        loop {

            // receive heartbeat or chunk
            let (_,address) = self.socket.recv_from(&mut buffer).await.expect("error receiving");

            if let Some((_,pts)) = PublisherToSubscriber::decode(&buffer) {

                match pts {

                    // heartbeat, respond with Ack
                    PublisherToSubscriber::Heartbeat(id) => {

                        let state = self.state.lock().await;

                        let mut send_buffer = Vec::<u8>::new();

                        // only respond if this is for the current message
                        if id == state.id {
               
                            //println!("receive heartbeat");

                            if let Some(last) = last_missing {
                                if last > first_missing {
                                    //println!("send nack {}-{}",first_missing,last);
                                    SubscriberToPublisher::NAck(id,first_missing,last).encode(&mut send_buffer);
                                }
                                else {
                                    //println!("send ack {}",first_missing);
                                    SubscriberToPublisher::Ack(id,first_missing).encode(&mut send_buffer);
                                }
                            }
                            else {
                                //println!("send ack {}",first_missing);
                                SubscriberToPublisher::Ack(id,first_missing).encode(&mut send_buffer);
                            }
                            self.socket.send_to(&mut send_buffer,address).await.expect("error sending retransmit request");
                        }
                    },

                    // chunk
                    PublisherToSubscriber::Chunk(chunk) => {

                        let mut state = self.state.lock().await;

                        // if this is a new chunk, reset state
                        if chunk.id != state.id {

                            start_time = time::Instant::now();

                            state.id = chunk.id;
                            state.buffer = vec![0; chunk.total_bytes as usize];
                            state.received = vec![false; chunk.total as usize];

                            chunks_total = 0;
                            chunks_ignored = 0;
                        }

                        chunks_total += 1;
                
                        // if we don't already have this chunk
                        if !state.received[chunk.index as usize] {

                            //println!("receive {}",chunk.index);

                            // copy data into final message buffer
                            let start = (chunk.index * chunk.chunk_size) as usize;
                            let end = start + chunk.data.len();
                            state.buffer[start..end].copy_from_slice(&chunk.data);
                
                            // mark the chunk as received
                            state.received[chunk.index as usize] = true;

                            // find first missing chunk
                            first_missing = chunk.total;
                            for i in 0..state.received.len() {
                                if !state.received[i] {
                                    first_missing = i as u32;
                                    break;
                                }
                            }

                            if first_missing < chunk.total {

                                // find last missing chunk of this range
                                last_missing = None;
                                for i in first_missing..state.received.len() as u32 {
                                    if state.received[i as usize] {
                                        last_missing = Some(i);
                                        break;
                                    }
                                }
                            }
                            else {
                                let end_time = time::Instant::now();
                                let throughput = chunk.total_bytes / ((end_time - start_time).as_micros() as u64);
                                let waste = (chunks_ignored * 100) / chunks_total;
                                if waste == 0 {
                                    println!("==> {:3} MBps, {:2}% <==",throughput,waste);
                                }
                                else {
                                    println!("    {:3} MBps, {:2}%",throughput,waste);
                                }

                                on_data(&state.buffer);
                            }
                        }
                        else {
                            chunks_ignored += 1;
                            //println!("ignore {}",chunk.index);
                        }
                    },
                }
            }
            else {
                //println!("message error");
            }
        }
    }
}