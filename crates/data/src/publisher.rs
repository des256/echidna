// Echidna - Data

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

pub struct Publisher {
    pub id: PubId,
    pub topic: String,
    pub stream: net::TcpStream,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
}

impl Publisher {
    pub async fn new(topic: &str) -> Arc<Publisher> {

        // new ID
        let id = rand::random::<u64>();

        println!("starting publisher {:016X}",id);

        // connect to participant
        let mut stream = net::TcpStream::connect("0.0.0.0:7332").await.expect("cannot connect to participant");

        // open data socket
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create publisher socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        // announce publisher to participant
        let mut send_buffer = Vec::new();
        let message = ToPart::InitPub(id,PubRef {
            topic: topic.to_string(),
        });
        message.encode(&mut send_buffer);
        stream.write_all(&send_buffer).await.expect("cannot send InitPub");    

        // create publisher
        let publisher = Arc::new(Publisher {
            id: id,
            topic: topic.to_string(),
            stream: stream,
            socket: socket,
            address: address,
        });

        publisher
    }

    pub async fn send(self: &Arc<Publisher>,_data: &[u8]) {

    }
}