// Echidna - Data

use crate::*;

pub struct Publisher {
    pub topic: Topic,
}

impl Publisher {

    fn new(topic: Topic) -> Option<Publisher> {
        return Some(Publisher {
            topic: topic,
        });
    }
}
