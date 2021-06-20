// Echidna - Data

use {
    crate::*,
    r#async::net::{
        UdpSocket,
        SocketAddr,
    },
    codec::Codec,
    std::{
        sync::{
            Arc,
            Mutex,
        },
        collections::HashMap,
    },
};

pub type PublisherId = u64;

pub struct PublisherState {
    pub message_id: Option<MessageId>,
    pub message: Arc<Vec<u8>>,
    pub arrived: HashMap<SocketAddr,Vec<bool>>,
}

pub struct Publisher {
    pub subscribers: HashMap<SubscriberId,Endpoint>,
    pub id: PublisherId,
    pub socket: UdpSocket,
    pub address: SocketAddr,
    pub topic: String,
    pub state: Mutex<PublisherState>,
}

impl Publisher {

    pub async fn new(topic: String) -> Option<Arc<Publisher>> {

        let socket = UdpSocket::bind("0.0.0.0:0").await.expect("cannot create publisher socket");
        // ====
        let address = socket.local_addr().expect("cannot get local address of socket");

        let publisher = Arc::new(Publisher {
            subscribers: HashMap::new(),
            id: rand::random::<u64>(),
            socket: socket,
            address: address,
            topic: topic,
            state: Mutex::new(PublisherState {
                message_id: None,
                message: Arc::new(Vec::new()),
                arrived: HashMap::new(),
            }),
        });

        // spawn loop that processes incoming acknowledgements from subscribers
        /*let this = Arc::clone(&publisher);
        spawn(async move {
            let mut buffer = vec![0u8; 65536];
            loop {
                let (_length,address) = this.socket.recv_from(&mut buffer).await.expect("error receiving acknowledgement");
                // ====
                if let Some((_,ack)) = Ack::decode(&buffer) {
                    let mut state = this.state.lock().unwrap();
                    if let Some(message_id) = state.message_id {
                        if ack.message_id == message_id {
                            if state.arrived.contains_key(&address) {
                                for range in &ack.ranges {
                                    for i in range.min..range.max {
                                        state.arrived.get_mut(&address).unwrap()[i as usize] = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }).detach();*/

        Some(publisher)
    }

    pub async fn send(&self,message: Arc<Vec<u8>>) {

        // send all samples to all subscribers
        let mut total = message.len() / SAMPLE_SIZE;
        if (message.len() % SAMPLE_SIZE) != 0 {
            total += 1;
        }
        println!("sending message of {} bytes, total chunks = {}",message.len(),total);
        
        // prepare new state for all subscribers
        let message_id = rand::random::<u64>();
        let mut arrived = HashMap::<SocketAddr,Vec<bool>>::new();
        for (id,endpoint) in &self.subscribers {
            arrived.insert(endpoint.address,vec![false; total]);
        }

        // send message to all subscribers
        let mut buffer = Vec::<u8>::new();
        let mut index = 0u32;
        let mut offset = 0usize;
        while offset < message.len() {
            println!("chunk {}:",index);
            let header = SampleHeader {
                ts: 0,
                message_id: message_id,
                total: total as u32,
                index: index,
            };
            println!("    header: {{ message_id: {}, total: {}, index: {}, }}",header.message_id,header.total,header.index);
            header.encode(&mut buffer);
            let size = {
                if (offset + SAMPLE_SIZE) > message.len() {
                    message.len() - offset
                }
                else {
                    SAMPLE_SIZE
                }
            };
            println!("    chunk size: {}",size);
            buffer.extend_from_slice(&message[offset..offset + size]);
            for (id,endpoint) in &self.subscribers {
                println!("    sending to subscriber at {}",endpoint.address);
                self.socket.send_to(&mut buffer,endpoint.address).await.expect("error sending data chunk");
                // ====
            }
            offset += size;
            index += 1;
        }

        // send heartbeat to all subscribers
        /*let mut buffer = Vec::<u8>::new();
        loop {
            
            // send heartbeat to all subscribers until they return
        }*/
    }
}
