// Echidna - Data

use crate::*;

pub struct Subscriber {
    pub topic: String,
}

impl Subscriber {

    fn new(topic: String) -> Option<Subscriber> {
        return Some(Subscriber {
            topic: topic,
        });
    }
}
