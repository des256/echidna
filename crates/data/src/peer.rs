// Echidna - Data

use {
    crate::*,
    codec::Codec,
    r#async::net::SocketAddr,
};

#[derive(Codec,Clone)]
pub struct PublisherDescr {
    pub id: PublisherId,
    pub address: SocketAddr,
    pub topic: String,
}

#[derive(Codec,Clone)]
pub struct SubscriberDescr {
    pub id: SubscriberId,
    pub address: SocketAddr,
    pub topic: String,
}

#[derive(Codec)]
pub struct Peer {
    pub id: ParticipantId,
    pub publishers: Vec<PublisherDescr>,
    pub subscriber: Vec<SubscriberDescr>,
}