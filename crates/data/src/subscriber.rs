// Echidna - Data

use {
    crate::*,
    tokio::{
        net,
        task,
        io::AsyncReadExt,
        time,
    },
    codec::Codec,
    std::{
        sync::Arc,
        net::SocketAddr,
        time::Duration,
    },
};

pub struct Subscriber {
    pub id: PubId,
    pub topic: String,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
}

impl Subscriber {
    pub async fn new(topic: &str,_on_data: impl Fn(&[u8]) + Send + 'static) -> Arc<Subscriber> {

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
        });

        // spawn participant receiver
        let this = Arc::clone(&subscriber);
        task::spawn(async move {
            this.run_participant_connection().await;
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
}