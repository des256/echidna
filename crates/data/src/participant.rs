// Echidna - Data

use {
    tokio::{
        task,
        io,
        net,
        time,
        io::AsyncReadExt,
        io::AsyncWriteExt,
    },
    codec::Codec,
    std::{
        collections::HashMap,
        sync::{
            Arc,
            Mutex,
        },
        time::Duration,
        net::{
            Ipv4Addr,
            SocketAddr,
        },
    },
};

type PeerId = u64;
type PubId = u64;
type SubId = u64;

#[derive(Codec)]
pub struct Beacon {
    pub id: PeerId,
    pub port: u16,
}

#[derive(Clone,Codec)]
pub struct PubRef {
    pub topic: String,
}

#[derive(Clone,Codec)]
pub struct SubRef {
    pub port: u16,
    pub topic: String,
}

#[derive(Codec)]
pub struct PeerAnnounce {
    pub id: PeerId,
    pub pubs: HashMap<PubId,PubRef>,
    pub subs: HashMap<SubId,SubRef>,
}

#[derive(Codec)]
pub enum PeerToPeer {
    NewPub(PubId,PubRef),
    DropPub(PubId),
    NewSub(SubId,SubRef),
    DropSub(SubId),
}

#[derive(Codec)]
pub enum ToPart {
    InitPub(PubId,PubRef),
    InitSub(SubId,SubRef),
}

#[derive(Codec)]
pub enum PartToPub {
    Init(HashMap<SubId,SubRef>),
    InitFailed,
    NewSub(SubId,SubRef),
    DropSub(SubId),
}

#[derive(Codec)]
pub enum PartToSub {
    Init,
    InitFailed,
}

pub struct PeerRef {
    pub stream: io::WriteHalf<net::TcpStream>,
    pub pubs: HashMap<PubId,PubRef>,
    pub subs: HashMap<SubId,SubRef>,
}

pub struct ParticipantState {
    pub peers: HashMap<PeerId,PeerRef>,
    pub pubs: HashMap<PubId,PubRef>,
    pub subs: HashMap<SubId,SubRef>,
}

pub struct Participant {
    pub id: PeerId,
    pub port: u16,
    pub state: Mutex<ParticipantState>,
}

impl Participant {
    pub async fn new() -> Arc<Participant> {

        println!("Participant::new");

        // new ID
        let id = rand::random::<u64>();

        // create participant listener
        let part_listener = net::TcpListener::bind("0.0.0.0:0").await.expect("cannot bind participant listener socket");
        let port = part_listener.local_addr().expect("cannot obtain local address of participant listener socket").port();

        // new participant
        let participant = Arc::new(Participant {
            id: id,
            port: port,
            state: Mutex::new(ParticipantState {
                peers: HashMap::new(),
                pubs: HashMap::new(),
                subs: HashMap::new(),
            }),
        });

        println!("created participant {:016X} at port {}",id,port);

        // spawn beacon broadcaster
        println!("spawning beacon broadcaster");
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_beacon_broadcaster().await;
        });

        // spawn beacon receiver
        println!("spawning beacon receiver");
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_beacon_receiver().await;
        });

        // spawn peer listener
        println!("spawning peer listener");
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_participant_listener(part_listener).await;
        });

        // spawn local listener
        println!("spawning local listener");
        let this = Arc::clone(&participant);
        task::spawn(async move {
            this.run_local_listener().await;
        });

        participant
    }

    async fn run_beacon_broadcaster(self: &Arc<Participant>) {

        // This task sends periodic beacon messages to anyone listening.
    
        // create UDP socket at any port
        let socket = net::UdpSocket::bind("0.0.0.0:0").await.expect("cannot create beacon transmit socket");

        loop {

            // broadcast beacon
            let beacon = Beacon {
                id: self.id,
                port: self.port,
            };
            let mut buffer: Vec<u8> = Vec::new();
            beacon.encode(&mut buffer);
            socket.send_to(&buffer,("239.255.0.1",7331)).await.expect("cannot send beacon");

            // sleep until next tick
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn run_beacon_receiver(self: &Arc<Participant>) {
    
        // This task receives beacons from peers, and in certain cases, establishes a connection.

        let mut buffer = vec![0u8; 65536];

        // create beacon receiver socket
        let socket = net::UdpSocket::bind("0.0.0.0:7331").await.expect("cannot create beacon receiver socket");
        socket.join_multicast_v4(Ipv4Addr::new(239,255,0,1),Ipv4Addr::new(0,0,0,0)).expect("cannot join multicast group");            

        loop {

            // receive beacon
            let (_,address) = socket.recv_from(&mut buffer).await.expect("cannot receive beacon");

            // decode beacon
            if let Some((_,beacon)) = Beacon::decode(&buffer) {

                // if this is not a local echo
                if beacon.id != self.id {

                    // if peer not already known, and port number strict higher
                    if {
                        let state = self.state.lock().expect("cannot lock participant");
                        if let None = state.peers.get(&beacon.id) {
                            beacon.port < self.port
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
            let (stream,address) = listener.accept().await.expect("cannot accept connection from remote participant");

            println!("incoming peer connection from {}",address);

            // spawn passive peer connection
            let this = Arc::clone(&self);
            task::spawn(async move {
                this.run_passive_peer(stream).await;
            });
        }
    }

    async fn run_local_listener(self: &Arc<Participant>) {

        // This task services incoming subscriber and publisher connections from other local processes.

        // create listener
        let listener = net::TcpListener::bind("0.0.0.0:7332").await.expect("cannot bind local listener socket");

        loop {

            // accept the connection
            let (mut stream,address) = listener.accept().await.expect("cannot accept connection from local endpoint");

            println!("incoming local connection from {}",address);

            // spawn local 
            let this = Arc::clone(&self);
            task::spawn(async move {

                let mut buffer = vec![0u8; 65536];

                // read first message, should be ToPart::InitPub or ToPart::InitSub
                if let Ok(_) = stream.read(&mut buffer).await {
                    if let Some((_,message)) = ToPart::decode(&buffer) {
                        match message {

                            ToPart::InitPub(id,publisher) => {
                                this.run_publisher(stream,id,publisher).await;
                            },

                            ToPart::InitSub(id,subscriber) => {
                                this.run_subscriber(stream,id,subscriber).await;
                            },
                        }
                    }    
                }
            });
        }
    }

    async fn run_publisher(self: &Arc<Participant>,mut stream: net::TcpStream,id: PubId,publisher: PubRef) {

        println!("service local publisher at {}",stream.peer_addr().unwrap());

        // This task runs communication with the local publisher (currently no traffic).

        let mut buffer = vec![0u8; 65536];

        // create local publisher reference
        {
            let mut state = self.state.lock().expect("cannot lock participant");
            println!("new local publisher {:016}",id);
            state.pubs.insert(id,publisher);
        }

        // TODO: PartToPub::Init

        // TODO: PeerToPeer::NewPub

        // wait for connection to break
        while let Ok(_) = stream.read(&mut buffer).await { }

        // TODO: PeerToPeer::DropPub

        // destroy local publisher reference
        {
            let mut state = self.state.lock().expect("cannot lock participant");
            println!("local publisher {:016X} died",id);
            state.pubs.remove(&id);
        }
    }

    async fn run_subscriber(self: &Arc<Participant>,mut stream: net::TcpStream,id: SubId,subscriber: SubRef) {

        println!("service local subscriber at {}",stream.peer_addr().unwrap());

        let mut buffer = vec![0u8; 65536];

        // create local subscriber reference
        {
            let mut state = self.state.lock().expect("cannot lock participant");
            println!("new local subscriber {:016}",id);
            state.subs.insert(id,subscriber);
        }

        // TODO: PartToSub::Init

        // TODO: PeerToPeer::NewSub

        // TODO: matching publishers: PartToPub::NewSub

        // wait for connection to break
        while let Ok(_) = stream.read(&mut buffer).await { }

        // TODO: PeerToPeer::DropSub

        // TODO: matching publishers: PartToPub::DropSub

        // destroy local subscriber reference
        {
            let mut state = self.state.lock().expect("cannot lock participant");
            println!("local subscriber {:016X} died",id);
            state.subs.remove(&id);
        }
    }

    async fn run_active_peer(self: &Arc<Participant>,stream: net::TcpStream,peer_id: PeerId) {

        println!("actively service peer at {}",stream.peer_addr().unwrap());

        // This task handles communication with a peer from the active side.

        let mut buffer = vec![0u8; 65536];

        // split stream read and write ends
        let (mut stream_read,stream_write) = io::split(stream);

        // create peer
        let mut peer = PeerRef {
            stream: stream_write,
            pubs: HashMap::new(),
            subs: HashMap::new(),
        };

        // send announcement to passive side
        println!("sending announcement to passive peer");
        let message = {
            let state = self.state.lock().expect("cannot lock participant");
            PeerAnnounce {
                id: self.id,
                pubs: state.pubs.clone(),
                subs: state.subs.clone(),
            }
        };
        message.encode(&mut buffer);
        peer.stream.write_all(&buffer).await.expect("cannot send Announce");    

        // get counter announcement from passive side
        if let Ok(_) = stream_read.read(&mut buffer).await {
            if let Some((_,message)) = PeerAnnounce::decode(&buffer) {

                println!("got response from passive peer");

                peer.pubs = message.pubs;
                peer.subs = message.subs;

                // and make peer reference live
                {
                    let mut state = self.state.lock().expect("cannot lock participant");
                    state.peers.insert(peer_id,peer);
                }

                // handle rest of the messages
                println!("servicing peer...");
                self.run_peer(stream_read,peer_id).await;

                // remove peer reference
                {
                    let mut state = self.state.lock().expect("cannot lock participant");
                    state.peers.remove(&peer_id);
                }
            }
        }
    }

    async fn run_passive_peer(self: &Arc<Participant>,stream: net::TcpStream) {

        println!("passively service peer at {}",stream.peer_addr().unwrap());

        // This task handles communication with a peer from the passive side.

        let mut buffer = vec![0u8; 65536];

        // split stream read and write ends
        let (mut stream_read,stream_write) = io::split(stream);

        // get announcement from active side
        if let Ok(_) = stream_read.read(&mut buffer).await {
            if let Some((_,message)) = PeerAnnounce::decode(&buffer) {

                println!("got announcement from active peer");

                // store new peer ID
                let peer_id = message.id;

                // create peer
                let mut peer = PeerRef {
                    stream: stream_write,
                    pubs: message.pubs,
                    subs: message.subs,
                };

                // send response to active side
                println!("sending response to active peer");
                let message = {
                    let state = self.state.lock().expect("cannot lock participant");
                    PeerAnnounce {
                        id: self.id,
                        pubs: state.pubs.clone(),
                        subs: state.subs.clone(),
                    }
                };
                message.encode(&mut buffer);
                peer.stream.write_all(&buffer).await.expect("cannot send Announce");    

                // and make peer reference live
                {
                    let mut state = self.state.lock().expect("cannot lock participant");
                    state.peers.insert(peer_id,peer);
                }

                // handle rest of the messages
                println!("servicing peer...");
                self.run_peer(stream_read,peer_id).await;

                // remove peer reference
                {
                    let mut state = self.state.lock().expect("cannot lock participant");
                    state.peers.remove(&peer_id);
                }
            }
        }
    }

    async fn run_peer(self: &Arc<Participant>,mut stream: io::ReadHalf<net::TcpStream>,peer_id: PeerId) {

        let mut buffer = vec![0u8; 65536];

        while let Ok(_) = stream.read(&mut buffer).await {
            if let Some((_,message)) = PeerToPeer::decode(&buffer) {
                match message {

                    // peer has new publisher
                    PeerToPeer::NewPub(id,publisher) => {
                        let mut state = self.state.lock().expect("cannot lock participant");
                        let peer = state.peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        peer.pubs.insert(id,publisher);
                    },

                    // peer lost publisher
                    PeerToPeer::DropPub(id) => {
                        let mut state = self.state.lock().expect("cannot lock participant");
                        let peer = state.peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        peer.pubs.remove(&id);
                    },

                    // peer has new subscriber
                    PeerToPeer::NewSub(id,subscriber) => {
                        let mut state = self.state.lock().expect("cannot lock participant");
                        let peer = state.peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        peer.subs.insert(id,subscriber);
                        // TODO: all local pubs with same topic: PartToPub::NewSub
                    },

                    // peer lost subscriber
                    PeerToPeer::DropSub(id) => {
                        let mut state = self.state.lock().expect("cannot lock participant");
                        let peer = state.peers.get_mut(&peer_id).expect(&format!("cannot find participant reference {:016X}",peer_id));
                        peer.subs.remove(&id);
                        // TODO: all local pubs with same topic: PartToPub::DropSub
                    },
                }
            }
        }
    }
}
