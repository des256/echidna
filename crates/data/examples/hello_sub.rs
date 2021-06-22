use {
    data::*,
    tokio::{
        runtime,
        time,
    },
    codec::Codec,
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

    // create and register hello subscriber
    let _subscriber = Subscriber::new("/hello",on_message).await;

    // wait forever
    loop {
        time::sleep(Duration::from_secs(10)).await;
    }
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
