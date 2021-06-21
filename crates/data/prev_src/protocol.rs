// Echidna - Data

use {
    r#async::net::SocketAddr,
    codec::Codec,
    std::collections::HashMap,
};

pub type ParticipantId = u64;
pub type SubscriberId = u64;
pub type PublisherId = u64;
pub type MessageId = u64;

#[derive(Codec)]
pub struct Endpoint {
    pub address: SocketAddr,
    pub topic: String,
}

#[derive(Codec)]
pub struct Beacon {
    pub id: ParticipantId,
    pub subscribers: HashMap<SubscriberId,Endpoint>,
}

#[derive(Codec)]
pub struct SampleHeader {
    pub ts: u64,
    pub message_id: MessageId,
    pub size: u64,
    pub total: u32,
    pub index: u32,
}

#[derive(Codec)]
pub enum PubToSub {
    Heartbeat,
    Sample(SampleHeader),  // rest of the packet contains the data
}

#[derive(Codec)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

#[derive(Codec)]
pub struct Ack {
    pub message_id: MessageId,
    pub ranges: Vec<Range>,
}

#[derive(Codec)]
pub enum SubToPub {
    Ack(Ack),
}

pub const SAMPLE_SIZE: usize = 16384;

pub const MAX_ALIVE: usize = 10;