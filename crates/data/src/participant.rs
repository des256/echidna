// Echidna - Data

use {
    crate::*,
    r#async::{
        spawn,
        net::{
            UdpSocket,
            SocketAddrV4,
            SocketAddrV6,
            SocketAddr,
            Ipv4Addr,
            Ipv6Addr,
            IpAddr,
        },
        Timer,
    },
    codec::Codec,
    std::{
        collections::HashMap,
        str::FromStr,
        sync::{
            Arc,
            Mutex,
        },
        time::{
            Instant,
            Duration,
        },
    },
};

pub struct ParticipantState {
    pub publishers: Vec<Arc<Publisher>>,
    pub subscribers: Vec<Arc<Subscriber>>,
}

pub struct Participant {
    pub id: ParticipantId,
    pub address: SocketAddr,
    pub state: Mutex<ParticipantState>,
}

impl Participant {

    pub fn new() -> Arc<Participant> {
        let participant = Arc::new(Participant {
            id: rand::random::<u64>(),
            address: SocketAddr::from_str("0.0.0.0:0").expect("cannot create socket address"),
            state: Mutex::new(ParticipantState {
                publishers: Vec::new(),
                subscribers: Vec::new(),
            }),
        });

        let this = Arc::clone(&participant);
        spawn(async move {

            // start now
            let mut next_time = Instant::now();

            // open socket to start transmitting beacons
            let socket = UdpSocket::bind(this.address).await.expect("cannot create announcer socket");

            loop {

                // create beacon
                let mut beacon = Beacon {
                    id: this.id,
                    subscribers: HashMap::new(),
                };
                {
                    let state = this.state.lock().expect("cannot lock participant");
                    for subscriber in &state.subscribers {
                        beacon.subscribers.insert(subscriber.id,Endpoint {
                            address: subscriber.address.clone(),
                            topic: subscriber.topic.clone(),
                        });
                    }
                }

                // encode it
                let mut buffer: Vec<u8> = Vec::new();
                beacon.encode(&mut buffer);

                // send it to the multicast group
                socket.send_to(&buffer,("239.255.0.1",7331)).await.expect("cannot send message");

                // kill subscriber references
                {
                    let mut state = this.state.lock().expect("cannot lock participant");
                    for publisher in &mut state.publishers {
                        let mut delete_ids = Vec::<SubscriberId>::new();
                        let mut pubstate = publisher.state.lock().expect("cannot lock publisher");
                        for (id,subscriber_ref) in &mut pubstate.subscribers {
                            subscriber_ref.alive -= 1;
                            if subscriber_ref.alive == 0 {
                                delete_ids.push(*id);
                            }
                        }
                        for id in delete_ids {
                            println!("subscriber reference {} of publisher {} died",id,publisher.id);
                            pubstate.subscribers.remove(&id);
                        }
                    }
                }

                // and wait until next entry
                next_time += Duration::from_secs(1);
                Timer::at(next_time).await;
            }
        }).detach();

        let this = Arc::clone(&participant);
        spawn(async move {

            // open local listener on 7331
            let socket = UdpSocket::bind("0.0.0.0:7331").await.expect("cannot create admin socket");

            // join the multicast group
            socket.join_multicast_v4(Ipv4Addr::new(239,255,0,1),Ipv4Addr::new(0,0,0,0)).expect("cannot join multicast group");

            loop {

                // receive beacon
                let mut buffer = vec![0u8; 65536];
                let (_,address) = socket.recv_from(&mut buffer).await.expect("receive error");
                let ip = match address {
                    SocketAddr::V4(socketaddrv4) => IpAddr::V4(*socketaddrv4.ip()),
                    SocketAddr::V6(socketaddrv6) => IpAddr::V6(*socketaddrv6.ip()),
                };
                if let Some((_,beacon)) = Beacon::decode(&buffer) {
                    if beacon.id != this.id {
                        println!("beacon from {:016X} at {:?}",beacon.id,address);
                        let state = this.state.lock().expect("cannot lock participant");
                        for (id,subscriber) in &beacon.subscribers {
                            println!("    subscriber {:016X} at {:?} for \"{}\"",id,subscriber.address,subscriber.topic);
                            for publisher in &state.publishers {
                                if publisher.topic == subscriber.topic {
                                    let mut pubstate = publisher.state.lock().expect("cannot lock publisher");
                                    if !pubstate.subscribers.contains_key(id) {
                                        println!("        new subscriber for publisher {:016X}",publisher.id);
                                        let port = match subscriber.address {
                                            SocketAddr::V4(socketaddrv4) => socketaddrv4.port(),
                                            SocketAddr::V6(socketaddrv6) => socketaddrv6.port(),
                                        };
                                        pubstate.subscribers.insert(*id,SubscriberRef {
                                            alive: MAX_ALIVE,
                                            address: match ip {
                                                IpAddr::V4(ipv4) => SocketAddr::V4(SocketAddrV4::new(ipv4,port)),
                                                IpAddr::V6(ipv6) => SocketAddr::V6(SocketAddrV6::new(ipv6,port,0,0)),
                                            },
                                        });
                                    }
                                    else {
                                        println!("        keeping subscriber reference {:016X} for publisher {:016X} alive",id,publisher.id);
                                        let mut subscriber_ref = pubstate.subscribers.get_mut(id).expect("cannot get publisher subscriber reference");
                                        subscriber_ref.alive = MAX_ALIVE;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }).detach();

        participant
    }

    pub fn register_publisher(&self,publisher: &Arc<Publisher>) {
        let mut state = self.state.lock().expect("cannot lock participant");
        state.publishers.push(Arc::clone(&publisher));
    }

    pub fn register_subscriber(&self,subscriber: &Arc<Subscriber>) {
        let mut state = self.state.lock().expect("cannot lock participant");
        state.subscribers.push(Arc::clone(&subscriber));
    }
}
