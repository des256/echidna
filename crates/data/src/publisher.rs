// Echidna - Data

use crate::*;

pub struct Publisher {
    pub topic: String,
}

impl Publisher {

    fn new(topic: String) -> Option<Publisher> {
        return Some(Publisher {
            topic: topic,
        });
    }
}
