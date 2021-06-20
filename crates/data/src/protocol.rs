// Echidna - Data

use {
    crate::*,
    codec::Codec,
};

pub type MessageId = u64;

#[derive(Codec)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

#[derive(Codec)]
pub struct Beacon {
    pub id: ParticipantId,
    pub publishers: Vec<PublisherDescr>,
    pub subscribers: Vec<SubscriberDescr>,
}

#[derive(Codec)]
pub struct SampleHeader {
    pub ts: u64,
    pub message_id: MessageId,
    pub total: u32,
    pub index: u32,
}

#[derive(Codec)]
pub struct Ack {
    pub message_id: MessageId,
    pub ranges: Vec<Range>,  // These are the samples I received
}

#[derive(Codec)]
pub enum PubToSub {
    Heartbeat,
    Sample(SampleHeader),  // rest of the packet contains the data
}

#[derive(Codec)]
pub enum SubToPub {
    Ack(Ack),
}

pub const SAMPLE_SIZE: usize = 16384;
