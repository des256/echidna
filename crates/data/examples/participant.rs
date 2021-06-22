use {
    tokio::{
        runtime,
        time,
    },
    data::Participant,
    std::time::Duration,
};

async fn async_main() {

    println!("starting participant...");
    
    let _participant = Participant::new().await;
    loop {
        time::sleep(Duration::from_secs(10)).await;
    }
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
