// Echidna - Data

use {
    crate::*,
    r#async::{
        spawn,
        net::{
            UdpSocket,
            Ipv4Addr,
        },
    },
    codec::Codec,
    std::time::{
        Instant,
        Duration,
    },
};

/*
discovery:
1. at start, and at every N seconds: send announcement to 239.255.0.1, 127.0.0.1 and local shared memory manager
2. respond to incoming announcements by sending publishers, subscribers, topics, data types, and QoS settings
3. for all same topics and datatypes, establish destinations
*/

pub struct Participant {
    pub publishers: Vec<Publisher>,
    pub subscribers: Vec<Subscriber>,
    pub topics: Vec<Topic>,
}

impl Participant {

    pub fn new() -> Option<Participant> {
        Some(Participant {
            publishers: Vec::new(),
            subscribers: Vec::new(),
            topics: Vec::new(),
        })
    }

    pub async fn run(&mut self) {

        // spawn loop that just keeps on sending Hello to the multicast group
        spawn(async move {
            let mut next_time = Instant::now();
            let socket = UdpSocket::bind("0.0.0.0:0").await.expect("cannot create announcer socket");
            loop {
                let mut buffer: Vec<u8> = Vec::new();
                "It's time to wind down...".to_string().encode(&mut buffer);
                socket.send_to(&buffer,("239.255.0.1",7331)).await.expect("cannot send message");
                next_time += Duration::from_secs(1);
                Timer::at(next_time).await;
            }    
        }).detach();

        // listen on multicast group
        let socket = UdpSocket::bind("0.0.0.0:7331").await.expect("cannot create admin socket");
        socket.join_multicast_v4(Ipv4Addr::new(239,255,0,1),Ipv4Addr::new(0,0,0,0)).expect("cannot join multicast group");
        loop {
            let mut buffer = vec![0u8; 1024];
            let (n,addr) = socket.recv_from(&mut buffer).await.expect("receive error");
            if let Some((_,message)) = String::decode(&buffer) {
                println!("Admin: received \"{}\"",message);
            }
        }    
    }
}
