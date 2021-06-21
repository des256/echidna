use {
    data::*,
    tokio::{
        runtime,
        time,
    },
    codec::Codec,
    data::{
        Participant,
        //Publisher,
    },
    std::{
        sync::Arc,
        time::Duration,
    },
};

async fn async_main() {

    // create participant (this can be another process, but for now start it here)
    let _participant = Participant::new();

    // create hello publisher
    let publisher = Publisher::new("/hello").await;

    // prepare message (string for now)
    let message = "This message is really awesome!".to_string();
    let mut buffer = Vec::<u8>::new();
    message.encode(&mut buffer);
    let buffer = Arc::new(buffer);

    // publish message every 5 seconds
    loop {
        println!("sending message: {}",message);
        publisher.send(&buffer).await;
        
        time::sleep(Duration::from_secs(5)).await;
    }
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
