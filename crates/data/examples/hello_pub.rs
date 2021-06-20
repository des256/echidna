use {
    r#async::block_on,
    codec::Codec,
    data::{
        Participant,
        Publisher,
    },
    std::sync::{
        Arc,
        Mutex,
    },
};

async fn async_main() {
    if let Some(mut participant) = Participant::new() {

        // create and register hello publisher
        let publisher = Publisher::new("/hello".to_string()).await.expect("cannot create publisher");
        participant.register_publisher(&publisher);

        // send message
        let mut message = Vec::<u8>::new();
        "Hello, World!".to_string().encode(&mut message);
        let message = Arc::new(message);
        publisher.send(message).await;

        // start participant
        let this = Arc::new(Mutex::new(participant));
        Participant::run(this).await;
    }
    else {
        println!("cannot create participant");
    }
}

fn main() {
    block_on(async_main());
}
