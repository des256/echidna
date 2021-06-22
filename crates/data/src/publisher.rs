// Echidna - Data

use {
    crate::*,
    codec::Codec,
    tokio::{
        net,
        task,
        io::AsyncReadExt,
        sync::Mutex,
    },
    std::{
        sync::Arc,
        net::SocketAddr,
        collections::HashMap,
    },
};

pub struct PublisherState {
    pub subs: HashMap<SubId,SubRef>,
}

pub struct Publisher {
    pub id: PubId,
    pub topic: String,
    pub socket: net::UdpSocket,
    pub address: SocketAddr,
    pub state: Mutex<PublisherState>,
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
        send_message(&mut stream,ToPart::InitPub(id,PubRef {
            topic: topic.to_string(),
        })).await;

        // create publisher
        let publisher = Arc::new(Publisher {
            id: id,
            topic: topic.to_string(),
            socket: socket,
            address: address,
            state: Mutex::new(PublisherState {
                subs: HashMap::new(),
            }),
        });

        // spawn participant receiver
        let this = Arc::clone(&publisher);
        task::spawn(async move {
            this.run_participant_receiver(stream).await;
        });
        
        publisher
    }

    pub async fn run_participant_receiver(self: &Arc<Publisher>,mut stream: net::TcpStream) {

        let mut recv_buffer = vec![0u8; 65536];

        // receive participant messages
        while let Ok(_) = stream.read(&mut recv_buffer).await {
            if let Some((_,message)) = PartToPub::decode(&recv_buffer) {
                match message {
                    PartToPub::Init(subs) => {
                        let mut state = self.state.lock().await;
                        state.subs = subs;
                        println!("publisher initialized:");
                        for (id,s) in &state.subs {
                            println!("    subscriber {:016X} at {} for topic \"{}\"",id,s.address,s.topic);
                        }
                    },
                    PartToPub::InitFailed => {
                        panic!("publisher initialization failed!");
                    },
                    PartToPub::NewSub(id,subscriber) => {
                        println!("new subscriber {:016X} at {}",id,subscriber.address);
                    },
                    PartToPub::DropSub(id) => {
                        println!("subscriber {:016X} lost",id);
                    },
                }
            }
        }
    }

    pub async fn send(self: &Arc<Publisher>,_data: &[u8]) {

    }
}