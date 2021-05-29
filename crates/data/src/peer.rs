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

#[derive(Codec)]
pub struct PublisherDescr {
    pub topic: String,
}

#[derive(Codec)]
pub struct SubscriberDescr {
    pub topic: String,
}

#[derive(Codec)]
pub struct Peer {
    pub id: ParticipantId,
    pub publishers: Vec<PublisherDescr>,
    pub subscriber: Vec<SubscriberDescr>,
}
