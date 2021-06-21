use {
    tokio::{
        runtime,
        time,
    },
    codec::Codec,
    data::{
        Participant,
        //Subscriber,
    },
    std::time::Duration,
};

fn on_message(buffer: &[u8]) {
    if let Some((_,message)) = String::decode(&buffer) {
        println!("message received: {}",message);
    }
    else {
        println!("cannot decode message");
    }
}

async fn async_main() {

    // create participant
    let participant = Participant::new();

    /*// create and register hello subscriber
    let subscriber = Subscriber::new("/hello".to_string(),on_message).await;
    participant.register_subscriber(&subscriber);

    // wait forever
    loop {
        time::sleep(Duration::from_secs(10)).await;
    }*/
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
