use {
    r#async::{
        block_on,
        Timer,
    },
    data::{
        Participant,
        Subscriber,
    },
    std::time::Duration,
};

async fn async_main() {

    // create participant
    let participant = Participant::new();

    // create and register hello subscriber
    let subscriber = Subscriber::new("/hello".to_string()).await.expect("cannot create publisher");
    {
        let mut p = participant.lock().expect("cannot lock participant");
        p.register_subscriber(&subscriber);
    }

    // wait forever
    loop {
        Timer::after(Duration::from_secs(10)).await;
    }
}

fn main() {
    block_on(async_main());
}
