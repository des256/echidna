// Echidna - Data

use {
    crate::*,
    tokio::{
        net,
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
    pub stream: net::TcpStream,
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
            stream: stream,
            socket: socket,
            address: address,
        });

        subscriber
    }
}