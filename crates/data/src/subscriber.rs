// Echidna - Data

use {
    crate::*,
    codec::Codec,
    r#async::{
        net::{
            UdpSocket,
            SocketAddr,
            SocketAddrV4,
            Ipv4Addr,
        },
    },
    std::{
        sync::{
            Arc,
        },
    },
};

pub struct SubscriberState {
    pub message_id: MessageId,
    pub received: Vec<bool>,
    pub buffer: Vec<u8>,
}

pub struct Subscriber {
    pub id: SubscriberId,
    pub socket: UdpSocket,
    pub address: SocketAddr,
    pub publisher_address: SocketAddr,
    pub topic: String,
}

impl Subscriber {

    pub async fn new(topic: String) -> Option<Arc<Subscriber>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.expect("cannot create subscriber socket");
        let address = socket.local_addr().expect("cannot get local address of socket");

        let subscriber = Arc::new(Subscriber {
            id: rand::random::<u64>(),
            socket: socket,
            address: address,
            publisher_address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0),0)),
            topic: topic,
        });

        let recv_subscriber = Arc::clone(&subscriber);
        spawn(async move {
            let mut state = SubscriberState {
                message_id: 0,
                received: Vec::new(),
                buffer: Vec::new(),
            };
            let mut buffer = vec![0u8; 65536];
            loop {
                let (_,address) = recv_subscriber.socket.recv_from(&mut buffer).await.expect("error receiving sample or heartbeat");
                if let Some((length,pts)) = PubToSub::decode(&buffer) {
                    match pts {
                        PubToSub::Heartbeat => {
                            /*let mut ranges = Vec::<Range>::new();
                            let mut range = Range { min: 0, max: 0, };
                            for buffer in &state.buffers {
                                range.max += 1;
                                if let None = buffer {
                                    ranges.push(Range { min: range.min,max: range.max, });
                                    range.min = range.max;
                                }
                            }
                            ranges.push(range);
                            SubToPub::Ack(Ack { message_id: state.message_id, ranges: ranges, }).encode(&mut buffer);
                            recv_subscriber.socket.send_to(&buffer,recv_subscriber.publisher_address).await.expect("unable to send acknowledgment to publisher");*/
                        },
                        PubToSub::Sample(sample) => {
                            let data = &buffer[length..];
                            if sample.message_id != state.message_id {
                                state.message_id = sample.message_id;
                                state.buffer = Vec::with_capacity(sample.total as usize * SAMPLE_SIZE);
                                state.received = vec![false; sample.total as usize];
                            }
                            state.buffer[(sample.index as usize * SAMPLE_SIZE)..].copy_from_slice(data);
                            state.received[sample.index as usize] = true;
                            let mut complete = true;
                            for received in &state.received {
                                if !received {
                                    complete = false;
                                    break;
                                }
                            }
                            if complete {
                                println!("received message of {} bytes from publisher at {}",state.buffer.len(),address);
                            }
                        },
                    }
                }
            }
        }).detach();
        
        return Some(subscriber);
    }
}
