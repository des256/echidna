use {
    data::*,
    tokio::{
        runtime,
        time,
    },
    std::{
        time::Duration,
        io::prelude::*,
        fs,
    },
};

fn on_message(buffer: &[u8]) {
    let mut file = fs::File::create("result.jpg").expect("cannot create file");
    file.write_all(&buffer).expect("cannot write to file");
}

async fn async_main() {

    // create and register hello subscriber
    let _subscriber = Subscriber::new(7332,"office_test","/hello",on_message).await;

    // wait forever
    loop {
        time::sleep(Duration::from_secs(10)).await;
    }
}

fn main() {
    let runtime = runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}
