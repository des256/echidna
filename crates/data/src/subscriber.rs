// Echidna - Data

use {
    crate::*,
    codec::Codec,
    r#async::{
        net::{
            UdpSocket,
            SocketAddr,
        },
    },
    std::{
        sync::{
            Arc,
            Mutex,
        },
    },
};

pub type SubscriberId = u64;

pub struct SubscriberState {
    pub message_id: MessageId,
    pub buffers: Vec<Option<Vec<u8>>>,
}

pub struct Subscriber {
    pub id: SubscriberId,
    pub socket: UdpSocket,
    pub address: SocketAddr,
    pub publisher_address: SocketAddr,
}

impl Subscriber {

    pub async fn new<T>(topic: String,on_message: dyn Fn(T)) -> Option<Arc<Subscriber>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.expect("cannot create subscriber socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        let subscriber = Arc::new(Subscriber {
            id: rand::random::<u64>(),
            socket: socket,
            address: address,
            publisher_address: SocketAddr::from("0.0.0.0:0"),
        });

        let recv_subscriber = Arc::clone(&subscriber);
        spawn(async move {
            let mut state = SubscriberState {
                message_id: 0,
                buffers: Vec::new(),
            };
            let mut buffer = vec![0u8; 65536];
            loop {
                let (_,address) = recv_subscriber.socket.recv_from(&mut buffer).await.expect("error receiving sample or heartbeat");
                if let Some((length,pts)) = PubToSub::decode(&buffer) {
                    match pts {
                        PubToSub::Heartbeat => {
                            let mut ranges = Vec::<Range>::new();
                            let mut range = Range { min: 0, max: 0, };
                            for buffer in &state.buffers {
                                range.max += 1;
                                if let None = buffer {
                                    ranges.push(range);
                                    range.min = range.max;
                                }
                            }
                            ranges.push(range);
                            SubToPub::Ack(Ack { message_id: state.message_id, ranges: ranges, }).encode(&mut buffer);
                            recv_subscriber.socket.send_to(&buffer,recv_subscriber.publisher_address).await.expect("unable to send acknowledgment to publisher");
                        },
                        PubToSub::Sample(sample) => {
                            let data = &buffer[length..];
                            if sample.message_id != state.message_id {
                                state.message_id = sample.message_id;
                                state.buffers = vec![None; sample.total as usize];
                            }
                            let mut data = Vec::<u8>::new();
                            data.extend_from_slice(&buffer[length..]);
                            state.buffers[sample.index as usize] = Some(data);
                            let mut complete = true;
                            for buffer in &state.buffers {
                                if let None = buffer {
                                    complete = false;
                                    break;
                                }
                            }
                            if complete {
                                // call on_message
                            }
                        },
                    }
                }
            }
        });
        
        return Some(subscriber);
    }
}
