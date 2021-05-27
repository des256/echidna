use {
    r#async::{
        block_on,
        Timer,
    },
    std::{
        time::{
            Duration,
            Instant,
        },
    },
};

fn main() -> std::io::Result<()> {
    block_on(async {
        let start = Instant::now();
        println!("Sleeping...");
        Timer::after(Duration::from_secs(1)).await;
        println!("Woke up after {:?}",start.elapsed());
        Ok(())
    })
}
