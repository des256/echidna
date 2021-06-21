use {
    tokio::{
        runtime,
        time,
    },
    data::Participant,
    std::time::Duration,
};

async fn async_main() {

    println!("async_main");

    // create participant
    let participant = Participant::new().await;

    // wait forever
    loop {
        time::sleep(Duration::from_secs(10)).await;
    }
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
