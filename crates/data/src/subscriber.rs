// Echidna - Data

use crate::*;

pub struct Subscriber {
    pub topic: Topic,
}

impl Subscriber {

    fn new(topic: Topic) -> Option<Subscriber> {
        return Some(Subscriber {
            topic: topic,
        });
    }
}
