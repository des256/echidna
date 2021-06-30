// Echidna - Data

use {
    crate::*,
    tokio::{
        task,
        io,
        net,
        time,
        sync::Mutex,
        io::AsyncReadExt,
    },
    codec::Codec,
    std::{
        collections::HashMap,
        sync::Arc,
        time::Duration,
        net::{
            IpAddr,
            Ipv4Addr,
            SocketAddr,
        },
    },
};

pub struct PeerRef {
    pub ip: IpAddr,
    pub stream: io::WriteHalf<net::TcpStream>,
    pub pubs: HashMap<PublisherId,PublisherRef>,
    pub subs: HashMap<SubscriberId,SubscriberRef>,
}

pub struct LocalPublisherRef {
    pub stream: io::WriteHalf<net::TcpStream>,
    pub topic: String,
}

pub struct LocalSubscriberRef {
    pub stream: io::WriteHalf<net::TcpStream>,
    pub topic: String,
    pub address: SocketAddr,
}

pub struct Participant {
    pub id: ParticipantId,
    pub domain: String,
    pub part_port: u16,
    pub pubsub_port: u16,
    pub beacon_port: u16,
    pub listener: net::TcpListener,
    pub peers: Mutex<HashMap<ParticipantId,PeerRef>>,
    pub pubs: Mutex<HashMap<PublisherId,LocalPublisherRef>>,
    pub subs: Mutex<HashMap<SubscriberId,LocalSubscriberRef>>,
}

impl Participant {

    pub async fn new(pubsub_port: u16,beacon_port: u16,domain: &str) -> Arc<Participant> {

        // new ID
        let id = rand::random::<u64>();

        // create participant listener
        let part_listener = net::TcpListener::bind("0.0.0.0:0").await.expect("cannot bind participant listener socket");
        let part_port = part_listener.local_addr().expect("cannot obtain local address of participant listener socket").port();

        // create pub/sub listener
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}",pubsub_port)).await.expect("cannot bind local listener socket");

        // new participant
        let participant = Arc::new(Participant {
            id: id,
            domain: domain.to_string(),
            part_port: part_port,
            pubsub_port: pubsub_port,
            beacon_port: beacon_port,
            listener: listener,
            peers: Mutex::new(HashMap::new()),
            pubs: Mutex::new(HashMap::new()),
            subs: Mutex::new(HashMap::new()),
        });

        // spawn beacon broadcaster
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_beacon_broadcaster(beacon_port).await;
        });

        // spawn beacon receiver
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_beacon_receiver(beacon_port).await;
        });

        // spawn peer listener
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_participant_listener(part_listener).await;
        });

        // spawn local listener
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_local_listener().await;
        });

        participant
    }

    async fn run_beacon_broadcaster(self: &Arc<Participant>,beacon_port: u16) {

        // This task sends periodic beacon messages to anyone listening.
    
        // create UDP socket at any port
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create beacon transmit socket");

        loop {

            // broadcast beacon
            let beacon = Beacon {
                id: self.id,
                domain: self.domain.clone(),
                port: self.part_port,
            };
            let mut buffer: Vec<u8> = Vec::new();
            beacon.encode(&mut buffer);
            socket.send_to(&buffer,("239.255.0.1",beacon_port)).await.expect("cannot send beacon");

            // sleep until next tick
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn run_beacon_receiver(self: &Arc<Participant>,beacon_port: u16) {
    
        // This task receives beacons from peers, and in certain cases, establishes a connection.

        let mut buffer = vec![0u8; 65536];

        // create beacon receiver socket
        let socket = net::UdpSocket::bind(format!("0.0.0.0:{}",beacon_port)).await.expect("cannot create beacon receiver socket");
        socket.join_multicast_v4(Ipv4Addr::new(239,255,0,1),Ipv4Addr::new(0,0,0,0)).expect("cannot join multicast group");            

        loop {

            // receive beacon
            let (_,address) = socket.recv_from(&mut buffer).await.expect("cannot receive beacon");

            // decode beacon
            if let Some((_,beacon)) = Beacon::decode(&buffer) {

                // if this is not a local echo, and it's the correct domain
                if (beacon.id != self.id) && (beacon.domain == self.domain) {

                    // if peer not already known, and port number strict higher
                    if {
                        let state_peers = self.peers.lock().await;
                        if let None = state_peers.get(&beacon.id) {
                            beacon.port < self.part_port
                        }
                        else {
                            false
                        }
                    } {
                        // connect to this peer
                        let address = SocketAddr::new(address.ip(),beacon.port);
                        let stream = net::TcpStream::connect(address).await.expect("cannot connect to remote participant");

                        // spawn active peer connection
                        let this = Arc::clone(&self);
                        task::spawn(async move {
                            this.run_active_peer(stream,beacon.id).await;
                        });
                    }
                }
            }
        }
    }

    async fn run_participant_listener(self: &Arc<Participant>,listener: net::TcpListener) {

        // This task services incoming peer connections.

        loop {

            // accept connection request
            let (stream,_) = listener.accept().await.expect("cannot accept connection from remote participant");

            // spawn passive peer connection
            let this = Arc::clone(&self);
            task::spawn(async move {
                this.run_passive_peer(stream).await;
            });
        }
    }

    async fn run_local_listener(self: &Arc<Participant>) {

        // This task services incoming subscriber and publisher connections from other local processes.

        loop {

            // accept the connection
            let (mut stream,_) = self.listener.accept().await.expect("cannot accept connection from local endpoint");

            // spawn local 
            let this = Arc::clone(&self);
            task::spawn(async move {

                let mut buffer = vec![0u8; 65536];

                // read first message, should be ToPart::InitPub or ToPart::InitSub
                if let Ok(_) = stream.read(&mut buffer).await {
                    if let Some((_,message)) = ToParticipant::decode(&buffer) {
                        match message {

                            ToParticipant::InitPub(id,domain,publisher) => {
                                this.run_publisher(stream,id,domain,publisher).await;
                            },

                            ToParticipant::InitSub(id,domain,subscriber) => {
                                this.run_subscriber(stream,id,domain,subscriber).await;
                            },
                        }
                    }    
                }
            });
        }
    }

    async fn run_publisher(self: &Arc<Participant>,stream: net::TcpStream,id: PublisherId,domain: String,publisher: PublisherRef) {

        // This task runs communication with the local publisher (currently no traffic).

        // make sure it's the same domain
        if domain == self.domain {

            // split stream read and write ends
            let (mut stream_read,stream_write) = io::split(stream);

            // create local publisher reference
            {
                let mut state_pubs = self.pubs.lock().await;
                state_pubs.insert(id,LocalPublisherRef {
                    stream: stream_write,
                    topic: publisher.topic.clone(),
                });
            }

            // initialize local publisher
            let mut subs = HashMap::<SubscriberId,SubscriberRef>::new();
            {
                let state_subs = self.subs.lock().await;
                for (id,s) in state_subs.iter() {
                    if s.topic == publisher.topic {
                        subs.insert(*id,SubscriberRef {
                            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),s.address.port()),
                            topic: s.topic.clone(),
                        });
                    }
                }
            }
            {
                let state_peers = self.peers.lock().await;
                for (_,peer) in state_peers.iter() {
                    for (id,s) in &peer.subs {
                        subs.insert(*id,SubscriberRef {
                            address: SocketAddr::new(peer.ip,s.address.port()),
                            topic: s.topic.clone(),
                        });
                    }
                }
            }
            {
                let mut state_pubs = self.pubs.lock().await;
                let p = state_pubs.get_mut(&id).unwrap();
                send_message(&mut p.stream,ParticipantToPublisher::Init(subs)).await;
            }

            // inform all peers of new publisher
            {
                let mut state_peers = self.peers.lock().await;
                for (_,peer) in state_peers.iter_mut() {
                    send_message(&mut peer.stream,ParticipantToParticipant::NewPub(id,publisher.clone())).await;
                }
            }

            // wait for connection to break
            let mut buffer = vec![0u8; 65536];
            while let Ok(length) = stream_read.read(&mut buffer).await {
                if length == 0 {
                    break;
                }
            }

            // inform all peers that publisher is lost
            {
                let mut state_peers = self.peers.lock().await;
                for (_,peer) in state_peers.iter_mut() {
                    send_message(&mut peer.stream,ParticipantToParticipant::DropPub(id)).await;
                }
            }

            // destroy local publisher reference
            {
                let mut state_pubs = self.pubs.lock().await;
                state_pubs.remove(&id);
            }
        }
        else {
            // initialization failed
            let mut state_pubs = self.pubs.lock().await;
            let p = state_pubs.get_mut(&id).unwrap();
            send_message(&mut p.stream,ParticipantToPublisher::InitFailed(PubInitFailed::DomainMismatch)).await;
        }
    }

    async fn run_subscriber(self: &Arc<Participant>,stream: net::TcpStream,id: SubscriberId,domain: String,subscriber: SubscriberRef) {

        // This task runs communication with the local subscriber.

        // make sure it's the same domain
        if domain == self.domain {

            // split stream read and write ends
            let (mut stream_read,stream_write) = io::split(stream);

            // create local subscriber reference
            {
                let mut state_subs = self.subs.lock().await;
                state_subs.insert(id,LocalSubscriberRef {
                    stream: stream_write,
                    address: subscriber.address,
                    topic: subscriber.topic.clone(),
                });
            }

            // initialize local subscriber
            {
                let mut state_subs = self.subs.lock().await;
                let p = state_subs.get_mut(&id).unwrap();
                send_message(&mut p.stream,ParticipantToSubscriber::Init).await;
            }

            // inform relevant local publishers of new subscriber
            {
                let mut state_pubs = self.pubs.lock().await;
                for (_,p) in state_pubs.iter_mut() {
                    if p.topic == subscriber.topic {
                        send_message(&mut p.stream,ParticipantToPublisher::NewSub(id,SubscriberRef {
                            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),subscriber.address.port()),
                            topic: subscriber.topic.clone(),
                        })).await;
                    }
                }
            }

            // inform all peers of new subscriber
            {
                let mut state_peers = self.peers.lock().await;
                for (_,peer) in state_peers.iter_mut() {
                    send_message(&mut peer.stream,ParticipantToParticipant::NewSub(id,subscriber.clone())).await;
                }
            }

            // wait for connection to break
            let mut buffer = vec![0u8; 65536];
            while let Ok(length) = stream_read.read(&mut buffer).await {
                if length == 0 {
                    break;
                }
            }

            // destroy local subscriber reference
            {
                let mut state_subs = self.subs.lock().await;
                state_subs.remove(&id);
            }

            // inform all peers that subscriber is lost
            {
                let mut state_peers = self.peers.lock().await;
                for (_,peer) in state_peers.iter_mut() {
                    send_message(&mut peer.stream,ParticipantToParticipant::DropSub(id)).await;
                }
            }

            // inform relevant local publishers that subscriber is lost
            {
                let mut state_pubs = self.pubs.lock().await;
                for (_,p) in state_pubs.iter_mut() {
                    if p.topic == subscriber.topic {
                        send_message(&mut p.stream,ParticipantToPublisher::DropSub(id)).await;
                    }
                }
            }

        }
        else {
            let mut state_subs = self.subs.lock().await;
            let p = state_subs.get_mut(&id).unwrap();
            send_message(&mut p.stream,ParticipantToSubscriber::InitFailed(SubInitFailed::DomainMismatch)).await;
        }
    }

    async fn run_active_peer(self: &Arc<Participant>,stream: net::TcpStream,peer_id: ParticipantId) {

        // This task handles communication with a peer from the active side.

        let address = stream.peer_addr().unwrap();

        // split stream read and write ends
        let (mut stream_read,stream_write) = io::split(stream);

        // create peer
        let mut peer = PeerRef {
            ip: address.ip(),
            stream: stream_write,
            pubs: HashMap::new(),
            subs: HashMap::new(),
        };

        // send announcement to passive side
        let message = {
            let pubs = {
                let state_pubs = self.pubs.lock().await;
                let mut pubs = HashMap::<PublisherId,PublisherRef>::new();
                for (id,p) in state_pubs.iter() {
                    pubs.insert(*id,PublisherRef {
                        topic: p.topic.clone(),
                    });
                }
                pubs
            };
            let subs = {
                let state_subs = self.subs.lock().await;
                let mut subs = HashMap::<SubscriberId,SubscriberRef>::new();
                for (id,s) in state_subs.iter() {
                    subs.insert(*id,SubscriberRef {
                        address: s.address,
                        topic: s.topic.clone(),
                    });
                }
                subs
            };
            ParticipantAnnounce {
                id: self.id,
                pubs: pubs,
                subs: subs,
            }
        };
        send_message(&mut peer.stream,message).await;

        // get counter announcement from passive side
        let mut recv_buffer = vec![0u8; 65536];
        if let Ok(_) = stream_read.read(&mut recv_buffer).await {
            if let Some((_,message)) = ParticipantAnnounce::decode(&recv_buffer) {

                peer.pubs = message.pubs;
                for (id,s) in message.subs.iter() {
                    peer.subs.insert(*id,SubscriberRef {
                        address: SocketAddr::new(address.ip(),s.address.port()),
                        topic: s.topic.clone(),
                    });
                }

                // and make peer reference live
                {
                    let mut state_peers = self.peers.lock().await;
                    state_peers.insert(peer_id,peer);
                }

                // notify relevant local publishers of the new subscribers
                {
                    let state_peers = self.peers.lock().await;
                    let mut state_pubs = self.pubs.lock().await;
                    let peer = state_peers.get(&peer_id).unwrap();
                    for (_,p) in state_pubs.iter_mut() {
                        for (sid,s) in peer.subs.iter() {
                            if p.topic == s.topic {
                                send_message(&mut p.stream,ParticipantToPublisher::NewSub(*sid,SubscriberRef {
                                    address: SocketAddr::new(peer.ip,s.address.port()),
                                    topic: s.topic.clone(),
                                })).await;
                            }
                        }
                    }
                }

                // handle rest of the messages
                self.run_peer(stream_read,peer_id).await;

                // notify relevant local publishers of lost subscribers
                {
                    let state_peers = self.peers.lock().await;
                    let mut state_pubs = self.pubs.lock().await;
                    let peer = state_peers.get(&peer_id).unwrap();
                    for (_,p) in state_pubs.iter_mut() {
                        for (sid,s) in peer.subs.iter() {
                            if p.topic == s.topic {
                                send_message(&mut p.stream,ParticipantToPublisher::DropSub(*sid)).await;
                            }
                        }
                    }
                }

                // remove peer reference
                {
                    let mut state_peers = self.peers.lock().await;
                    state_peers.remove(&peer_id);
                }
            }
        }
    }

    async fn run_passive_peer(self: &Arc<Participant>,stream: net::TcpStream) {

        // This task handles communication with a peer from the passive side.

        let address = stream.peer_addr().unwrap();

        // split stream read and write ends
        let (mut stream_read,stream_write) = io::split(stream);

        // get announcement from active side
        let mut recv_buffer = vec![0u8; 65536];
        if let Ok(_) = stream_read.read(&mut recv_buffer).await {
            if let Some((_,message)) = ParticipantAnnounce::decode(&recv_buffer) {

                // store new peer ID
                let peer_id = message.id;

                // create peer
                let mut peer = PeerRef {
                    ip: address.ip(),
                    stream: stream_write,
                    pubs: message.pubs,
                    subs: HashMap::new(),
                };
                for (id,s) in message.subs.iter() {
                    peer.subs.insert(*id,SubscriberRef {
                        address: SocketAddr::new(address.ip(),s.address.port()),
                        topic: s.topic.clone(),
                    });
                }

                // send response to active side
                let message = {
                    let pubs = {
                        let state_pubs = self.pubs.lock().await;
                        let mut pubs = HashMap::<PublisherId,PublisherRef>::new();
                        for (id,p) in state_pubs.iter() {
                            pubs.insert(*id,PublisherRef {
                                topic: p.topic.clone(),
                            });
                        }
                        pubs
                    };
                    let subs = {
                        let state_subs = self.subs.lock().await;
                        let mut subs = HashMap::<SubscriberId,SubscriberRef>::new();
                        for (id,s) in state_subs.iter() {
                            subs.insert(*id,SubscriberRef {
                                address: s.address,
                                topic: s.topic.clone(),
                            });
                        }
                        subs
                    };
                    ParticipantAnnounce {
                        id: self.id,
                        pubs: pubs,
                        subs: subs,
                    }
                };
                send_message(&mut peer.stream,message).await;

                // and make peer reference live
                {
                    let mut state_peers = self.peers.lock().await;
                    state_peers.insert(peer_id,peer);
                }

                // notify relevant local publishers of the new subscribers
                {
                    let state_peers = self.peers.lock().await;
                    let mut state_pubs = self.pubs.lock().await;
                    let peer = state_peers.get(&peer_id).unwrap();
                    for (_,p) in state_pubs.iter_mut() {
                        for (sid,s) in peer.subs.iter() {
                            if p.topic == s.topic {
                                send_message(&mut p.stream,ParticipantToPublisher::NewSub(*sid,SubscriberRef {
                                    address: SocketAddr::new(peer.ip,s.address.port()),
                                    topic: s.topic.clone(),
                                })).await;
                            }
                        }
                    }
                }

                // handle rest of the messages
                self.run_peer(stream_read,peer_id).await;

                // notify relevant local publishers of lost subscribers
                {
                    let state_peers = self.peers.lock().await;
                    let mut state_pubs = self.pubs.lock().await;
                    let peer = state_peers.get(&peer_id).unwrap();
                    for (_,p) in state_pubs.iter_mut() {
                        for (sid,s) in peer.subs.iter() {
                            if p.topic == s.topic {
                                send_message(&mut p.stream,ParticipantToPublisher::DropSub(*sid)).await;
                            }
                        }
                    }
                }

                // remove peer reference
                {
                    let mut state_peers = self.peers.lock().await;
                    state_peers.remove(&peer_id);
                }
            }
        }
    }

    async fn run_peer(self: &Arc<Participant>,mut stream: io::ReadHalf<net::TcpStream>,peer_id: ParticipantId) {

        let mut buffer = vec![0u8; 65536];

        while let Ok(length) = stream.read(&mut buffer).await {
            if length == 0 {
                break;
            }
            if let Some((_,message)) = ParticipantToParticipant::decode(&buffer) {
                match message {

                    // peer has new publisher
                    ParticipantToParticipant::NewPub(id,publisher) => {
                        let mut state_peers = self.peers.lock().await;
                        let peer = state_peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        peer.pubs.insert(id,publisher);
                    },

                    // peer lost publisher
                    ParticipantToParticipant::DropPub(id) => {
                        let mut state_peers = self.peers.lock().await;
                        let peer = state_peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        peer.pubs.remove(&id);
                    },

                    // peer has new subscriber
                    ParticipantToParticipant::NewSub(id,subscriber) => {
                        let mut state_peers = self.peers.lock().await;
                        let peer = state_peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        let mut state_pubs = self.pubs.lock().await;
                        for (_,p) in state_pubs.iter_mut() {
                            if p.topic == subscriber.topic {
                                send_message(&mut p.stream,ParticipantToPublisher::NewSub(id,SubscriberRef {
                                    address: SocketAddr::new(peer.ip,subscriber.address.port()),
                                    topic: subscriber.topic.clone(),
                                })).await;
                            }
                        }
                        peer.subs.insert(id,subscriber);
                    },

                    // peer lost subscriber
                    ParticipantToParticipant::DropSub(id) => {
                        let mut state_peers = self.peers.lock().await;
                        let peer = state_peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        let topic = peer.subs.get(&id).unwrap().topic.clone();
                        let mut state_pubs = self.pubs.lock().await;
                        for (_,p) in state_pubs.iter_mut() {
                            if p.topic == topic {
                                send_message(&mut p.stream,ParticipantToPublisher::DropSub(id)).await;
                            }
                        }
                        peer.subs.remove(&id);
                    },
                }
            }
        }
    }
}
