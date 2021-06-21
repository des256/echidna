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

pub struct SubscriberRef {
    pub alive: usize,
    pub address: SocketAddr,
}

pub struct PublisherState {
    pub subscribers: HashMap<SubscriberId,SubscriberRef>,
    pub message_id: Option<MessageId>,
    pub message: Arc<Vec<u8>>,
}

pub struct Publisher {
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
            id: rand::random::<u64>(),
            socket: socket,
            address: address,
            topic: topic,
            state: Mutex::new(PublisherState {
                subscribers: HashMap::new(),
                message_id: None,
                message: Arc::new(Vec::new()),
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
        
        // prepare new message
        let message_id = rand::random::<u64>();

        // copy subscriber list
        let mut subscribers = HashMap::<SubscriberId,SubscriberRef>::new();
        {
            let state = self.state.lock().expect("cannot lock publisher");
            for (id,subscriber) in &state.subscribers {
                subscribers.insert(*id,SubscriberRef {
                    alive: subscriber.alive,
                    address: subscriber.address,
                });
            }
        }

        // send message to all subscribers
        let mut buffer = Vec::<u8>::new();
        let mut index = 0u32;
        let mut offset = 0usize;
        while offset < message.len() {
            let header = SampleHeader {
                ts: 0,
                message_id: message_id,
                size: message.len() as u64,
                total: total as u32,
                index: index,
            };
            PubToSub::Sample(header).encode(&mut buffer);
            let size = {
                if (offset + SAMPLE_SIZE) > message.len() {
                    message.len() - offset
                }
                else {
                    SAMPLE_SIZE
                }
            };
            buffer.extend_from_slice(&message[offset..offset + size]);
            for (_,subscriber) in &subscribers {
                self.socket.send_to(&mut buffer,subscriber.address).await.expect("error sending data chunk");
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
