// Echidna - Data

use {
    crate::*,
    tokio::{
        net,
        task,
        io::AsyncReadExt,
        io::AsyncWriteExt,
    },
    codec::Codec,
    std::{
        sync::Arc,
        net::SocketAddr,
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

        // connect to participant
        let mut stream = net::TcpStream::connect("0.0.0.0:7332").await.expect("cannot connect to participant");

        // open data socket
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create subscriber socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        // announce subscriber to participant
        let message = ToPart::InitSub(id,SubRef {
            port: address.port(),
            topic: topic.to_string(),
        });
        let mut send_buffer = Vec::<u8>::new();
        message.encode(&mut send_buffer);
        stream.write_all(&send_buffer).await.expect("cannot send InitSub");

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
            this.run_participant_receiver(stream).await;
        });

        subscriber
    }

    pub async fn run_participant_receiver(self: &Arc<Subscriber>,mut stream: net::TcpStream) {

        let mut recv_buffer = vec![0u8; 65536];

        // receive participant messages
        while let Ok(_) = stream.read(&mut recv_buffer).await {
            if let Some((_,message)) = PartToSub::decode(&recv_buffer) {
                match message {
                    PartToSub::Init => {
                        println!("participant accepts init");
                    },
                    _ => {
                        println!("TODO: some other message...");
                    },
                }
            }
        }
    }
}