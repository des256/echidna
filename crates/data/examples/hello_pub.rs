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
    participant.register_publisher(&publisher);

    // prepare message (string for now)
    let message = "This message is published across UDP.".to_string();
    let mut buffer = Vec::<u8>::new();
    message.encode(&mut buffer);
    let buffer = Arc::new(buffer);

    // publish message every 5 seconds
    let mut next_time = Instant::now();
    loop {
        println!("sending message: {}",message);
        publisher.send(Arc::clone(&buffer)).await;
        
        next_time += Duration::from_secs(5);
        Timer::at(next_time).await;
    }
}

fn main() {
    block_on(async_main());
}
