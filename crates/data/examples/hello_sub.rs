use {
    r#async::block_on,
    data::{
        Participant,
        Subscriber,
    },
    std::sync::{
        Arc,
        Mutex,
    },
};

async fn async_main() {
    if let Some(mut participant) = Participant::new() {

        // create and register hello subscriber
        let subscriber = Subscriber::new("/hello".to_string()).await.expect("cannot create publisher");
        participant.register_subscriber(&subscriber);

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
