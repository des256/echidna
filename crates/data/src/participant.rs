// Echidna - Data

use {
    crate::*,
    r#async::{
        spawn,
        net::{
            UdpSocket,
            SocketAddr,
            Ipv4Addr,
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
                    let state = this.state.lock().unwrap();
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

                if let Some((_,beacon)) = Beacon::decode(&buffer) {
                    if beacon.id != this.id {
                        println!("beacon from {:016X} at {:?}",beacon.id,address);
                        //let th = this.lock().expect("cannot lock participant");
                        for (id,subscriber) in &beacon.subscribers {
                            println!("    subscriber {:016X} at {:?} for \"{}\"",id,subscriber.address,subscriber.topic);
                            /*for publisher in &th.publishers {
                                if publisher.topic == subscriber.topic {
                                    if !publisher.subscribers.contains_key(id) {
                                        publisher.subscribers.insert(*id,subscriber.address);
                                    }
                                }
                            }*/
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
