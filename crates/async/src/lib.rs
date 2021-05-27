// Echidna - Async

//! Asynchronous processing.
//! 
//! This currently just re-exports `smol`.

pub use smol::{
    Executor,
    LocalExecutor,
    Task,
    block_on,
    Async,
    Timer,
    unblock,
    Unblock,
    future,
    io,
    pin,
    prelude,
    ready,
    stream,
    channel,
    fs,
    lock,
    net,
    process,
    spawn,
};
