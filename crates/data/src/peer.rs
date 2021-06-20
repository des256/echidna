// Echidna - Data

use {
    crate::*,
    r#async::net::{
        SocketAddr,
    },
    codec::Codec,
    std::{
        collections::HashMap,
    },
};

#[derive(Codec)]
pub struct Endpoint {
    pub address: SocketAddr,
    topic: String,
}

#[derive(Codec)]
pub struct Peer {
    pub id: ParticipantId,
    pub publishers: HashMap<PublisherId,Endpoint>,
    pub subscriber: HashMap<SubscriberId,Endpoint>,
}
