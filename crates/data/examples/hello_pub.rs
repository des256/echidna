use {
    data::*,
    tokio::{
        runtime,
        time,
        fs,
        io::AsyncReadExt,
    },
    std::time::Duration,
};

async fn async_main() {

    // prepare message
    let mut file = fs::File::open("test.jpg").await.expect("cannot open file");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).await.expect("cannot read file");

    // create hello publisher
    let publisher = Publisher::new_default(7332,"office_test","/hello").await;

    // publish message every 5 seconds
    loop {
        publisher.send(&buffer).await;
        
        time::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
