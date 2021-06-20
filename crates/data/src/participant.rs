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

pub struct Participant {
    pub id: ParticipantId,
    pub address: SocketAddr,
    pub publishers: Vec<Arc<Publisher>>,
    pub subscribers: Vec<Arc<Subscriber>>,
}

impl Participant {

    pub fn new() -> Option<Participant> {
        Some(Participant {
            id: rand::random::<u64>(),
            address: SocketAddr::from_str("0.0.0.0:0").expect("cannot create socket address"),
            publishers: Vec::new(),
            subscribers: Vec::new(),
        })
    }

    pub async fn run_beacon(this: Arc<Mutex<Self>>) {

        // start now
        let mut next_time = Instant::now();

        // open socket to start transmitting beacons
        let (id,address) = { let lock = this.lock().unwrap(); (lock.id,lock.address) };
        let socket = UdpSocket::bind(address).await.expect("cannot create announcer socket");

        loop {

            // create beacon
            let mut beacon = Beacon {
                id: id.clone(),
                publishers: HashMap::new(),
                subscribers: HashMap::new(),
            };
            {
                let participant = this.lock().unwrap();
                for publisher in &participant.publishers {
                    beacon.publishers.insert(publisher.id,Endpoint {
                        address: publisher.address.clone(),
                        topic: publisher.topic.clone(),
                    });
                }
                for subscriber in &participant.subscribers {
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
    }

    pub async fn run_admin(this: Arc<Mutex<Self>>) {

        // open local listener on 7331
        let socket = UdpSocket::bind("0.0.0.0:7331").await.expect("cannot create admin socket");

        // join the multicast group
        socket.join_multicast_v4(Ipv4Addr::new(239,255,0,1),Ipv4Addr::new(0,0,0,0)).expect("cannot join multicast group");

        loop {

            // receive beacon
            let mut buffer = vec![0u8; 65536];
            let (_,addr) = socket.recv_from(&mut buffer).await.expect("receive error");

            if let Some((_,beacon)) = Beacon::decode(&buffer) {
                println!("beacon from {:016X} at {:?}",beacon.id,addr);
                for (id,publisher) in &beacon.publishers {
                    println!("    publisher {:016X} at {:?} for \"{}\"",id,publisher.address,publisher.topic);
                }
                let th = this.lock().expect("cannot lock participant");
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

    pub async fn run(this: Arc<Mutex<Self>>) {

        let beacon_this = Arc::clone(&this);
        spawn(async move { Participant::run_beacon(beacon_this).await; }).detach();

        //let admin_this = Arc::clone(&this);
        //spawn(async move { Participant::run_admin(admin_this); }).detach();

        Participant::run_admin(this).await;
    }

    pub fn register_publisher(&mut self,publisher: &Arc<Publisher>) {
        self.publishers.push(Arc::clone(&publisher));
    }

    pub fn register_subscriber(&mut self,subscriber: &Arc<Subscriber>) {
        self.subscribers.push(Arc::clone(&subscriber));
    }
}


// beacon:
//     this is me, I am publishing ... and I am subscribing to ...
//
// pubnew:
//     I'm also publishing ...
//
// subnew:
//     I'm also subscribing to ...
//
// pubdrop:
//     I'm no longer publishing ...
//
// subdrop:
//     I'm no longer subscribing to ...

