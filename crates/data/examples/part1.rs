use {
    r#async::{
        block_on,
        spawn,
    },
    data::Participant,
};

fn main() {
    if let Some(mut participant) = Participant::new() {
        block_on(participant.run());
    }
}
