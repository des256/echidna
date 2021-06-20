use {
    r#async::{
        block_on,
        Timer,
    },
    data::Participant,
    std::time::Duration,
};

async fn async_main() {

    // create participant
    let _participant = Participant::new();

    // wait forever
    loop {
        Timer::after(Duration::from_secs(10)).await;
    }
}

fn main() {
    block_on(async_main());
}
