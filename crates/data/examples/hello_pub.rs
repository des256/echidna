use {
    r#async::{
        block_on,
        Timer,
    },
    codec::Codec,
    data::{
        Participant,
        Publisher,
    },
    std::{
        sync::Arc,
        time::{
            Instant,
            Duration,
        },
    },
};

async fn async_main() {

    // create participant
    let participant = Participant::new();

    // create and register hello publisher
    let publisher = Publisher::new("/hello".to_string()).await.expect("cannot create publisher");
    {
        let mut p = participant.lock().expect("cannot lock participant");
        p.register_publisher(&publisher);
    }

    // prepare message
    let mut message = Vec::<u8>::new();
    "Hello, World!".to_string().encode(&mut message);
    let message = Arc::new(message);

    // publish message every 5 seconds
    let mut next_time = Instant::now();
    loop {
        publisher.send(Arc::clone(&message)).await;
        
        next_time += Duration::from_secs(5);
        Timer::at(next_time).await;
    }
}

fn main() {
    block_on(async_main());
}
