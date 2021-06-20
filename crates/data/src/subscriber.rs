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
    pub buffer: Vec<u8>,
    pub received: Vec<bool>,
}

pub struct Subscriber {
    pub id: SubscriberId,
    pub socket: UdpSocket,
    pub address: SocketAddr,
    pub publisher_address: SocketAddr,
    pub topic: String,
}

impl Subscriber {

    pub async fn new(topic: String,on_message: impl Fn(&[u8]) + Send + 'static) -> Arc<Subscriber> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.expect("cannot create subscriber socket");
        let address = socket.local_addr().expect("cannot get local address of socket");
        let id = rand::random::<u64>();
        let subscriber = Arc::new(Subscriber {
            id: id,
            socket: socket,
            address: address,
            publisher_address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0),0)),
            topic: topic.clone(),
        });

        let this = Arc::clone(&subscriber);
        spawn(async move {
            println!("spawning subscriber {:016X} for topic \"{}\"",id,topic);
            let mut state = SubscriberState {
                message_id: 0,
                buffer: Vec::new(),
                received: Vec::new(),
            };
            let mut buffer = vec![0u8; 65536];
            loop {
                let (full_length,address) = this.socket.recv_from(&mut buffer).await.expect("error receiving sample or heartbeat");
                println!("received something from {}",address);
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
                            println!("receiving sample for message {:016X}, index {} of {}",sample.message_id,sample.index,sample.total);
                            let data = &buffer[length..full_length];
                            if sample.message_id != state.message_id {
                                println!("this is a new message, so create new buffers");
                                state.message_id = sample.message_id;
                                state.buffer = vec![0; sample.total as usize * SAMPLE_SIZE];
                                state.received = vec![false; sample.total as usize];
                                println!("buffer at size {}",state.buffer.len());
                            }
                            println!("trying to copy from data ({} bytes) to buffer ({} bytes)",data.len(),state.buffer.len());
                            println!("would be at state.buffer[{}..]",sample.index as usize * SAMPLE_SIZE);
                            let start = sample.index as usize * SAMPLE_SIZE;
                            let end = start + data.len();
                            state.buffer[start..end].copy_from_slice(data);
                            state.received[sample.index as usize] = true;

                            let mut complete = true;
                            for received in &state.received {
                                if !received {
                                    complete = false;
                                    break;
                                }
                            }

                            if complete {
                                on_message(&state.buffer[0..sample.size as usize]);
                            }
                        },
                    }
                }
                else {
                    println!("message error");
                }
            }
        }).detach();
        
        return subscriber;
    }
}
