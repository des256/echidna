// Echidna - Data

use {
    tokio::{
        io,
        io::AsyncWriteExt,
    },
    codec::Codec,
    std::{
        collections::HashMap,
        net::SocketAddr,
    },
    libc::{
        c_int,
    },
};

pub type MessageId = u64;
pub type ParticipantId = u64;
pub type PublisherId = u64;
pub type SubscriberId = u64;

pub type ShmDescr = c_int;

pub struct SharedMemory {
    pub descr: ShmDescr,
    pub data: Vec<u8>,
}

#[derive(Codec)]
pub struct Chunk {
    pub ts: u64,
    pub id: MessageId,
    pub total_bytes: u64,
    pub chunk_size: u32,
    pub total: u32,
    pub index: u32,
    pub data: Vec<u8>,
}

#[derive(Codec)]
pub enum PublisherToSubscriber {
    Heartbeat(MessageId),
    Chunk(Chunk),
}

#[derive(Codec)]
pub enum SubscriberToPublisher {
    Ack(MessageId,u32),
    NAck(MessageId,u32,u32),
}

#[derive(Codec)]
pub struct Beacon {
    pub id: ParticipantId,
    pub domain: String,
    pub port: u16,
}

#[derive(Clone,Codec)]
pub struct PublisherRef {
    pub topic: String,
}

#[derive(Clone,Codec)]
pub struct SubscriberRef {
    pub address: SocketAddr,
    pub topic: String,
}

#[derive(Codec)]
pub struct ParticipantAnnounce {
    pub id: ParticipantId,
    pub pubs: HashMap<PublisherId,PublisherRef>,
    pub subs: HashMap<SubscriberId,SubscriberRef>,
}

#[derive(Codec)]
pub enum ParticipantToParticipant {
    NewPub(PublisherId,PublisherRef),
    DropPub(PublisherId),
    NewSub(SubscriberId,SubscriberRef),
    DropSub(SubscriberId),
}

#[derive(Codec)]
pub enum ToParticipant {
    InitPub(PublisherId,String,PublisherRef),
    InitSub(SubscriberId,String,SubscriberRef),
}

#[derive(Codec)]
pub enum PubInitFailed {
    DomainMismatch,
}

#[derive(Codec)]
pub enum ParticipantToPublisher {
    Init(HashMap<SubscriberId,SubscriberRef>,HashMap<SubscriberId,SubscriberRef>),
    InitFailed(PubInitFailed),
    NewLocalSub(SubscriberId,SubscriberRef),
    NewPeerSub(SubscriberId,SubscriberRef),
    DropLocalSub(SubscriberId),
    DropPeerSub(SubscriberId),
}

#[derive(Codec)]
pub enum SubInitFailed {
    DomainMismatch,
}

#[derive(Codec)]
pub enum ParticipantToSubscriber {
    Init,
    InitFailed(SubInitFailed),
}

pub async fn send_message<S: io::AsyncWrite + Unpin,M: Codec>(stream: &mut S,message: M) {
    let mut send_buffer = Vec::new();
    message.encode(&mut send_buffer);
    stream.write_all(&send_buffer).await.expect("cannot send message");
}
