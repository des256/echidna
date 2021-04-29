use {
    r#async::{
        io,
        Async,
        block_on,
    },
    std::{
        os::unix::io::AsRawFd,
        time::{
            Duration,
            Instant,
        },
    },
    timerfd::{
        SetTimeFlags,
        TimerFd,
        TimerState,
    },
};

fn io_error(error: nix::Error) -> io::Error {
    match error {
        nix::Error::Sys(code) => code.into(),
        other => io::Error::new(io::ErrorKind::Other,Box::new(other)),
    }
}

async fn sleep(duration: Duration) -> io::Result<()> {
    let mut timer = TimerFd::new()?;
    timer.set_state(TimerState::Oneshot(duration),SetTimeFlags::Default);
    Async::new(timer)?.read_with(|t| nix::unistd::read(t.as_raw_fd(),&mut [0u8; 8]).map_err(io_error)).await?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    block_on(async {
        let start = Instant::now();
        println!("Sleeping...");
        sleep(Duration::from_secs(1)).await?;
        println!("Woke up after {:?}",start.elapsed());
        Ok(())
    })
}
