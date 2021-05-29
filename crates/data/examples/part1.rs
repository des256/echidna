use {
    r#async::{
        block_on,
        spawn,
    },
    data::Participant,
    std::sync::{
        Arc,
        Mutex,
    },
};

fn main() {
    if let Some(participant) = Participant::new() {
        let this = Arc::new(Mutex::new(participant));
        block_on(Participant::run(this));
    }
}
