#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            core::task::Poll::Ready(t) => t,
            core::task::Poll::Pending => return core::task::Poll::Pending,
        }
    };
}

#[macro_export]
macro_rules! pin {
    ($($x:ident),* $(,)?) => {
        $(
            let mut $x = $x;
            #[allow(unused_mut)]
            let mut $x = unsafe {
                core::pin::Pin::new_unchecked(&mut $x)
            };
        )*
    }
}

mod rng;
pub use rng::*;

mod parking;
pub use parking::*;

mod wakerfn;
pub use wakerfn::*;

mod asyncread;
pub use asyncread::*;

mod asyncwrite;
pub use asyncwrite::*;

mod asyncseek;
pub use asyncseek::*;

mod asyncbufread;
pub use asyncbufread::*;

mod atomicwaker;
pub use atomicwaker::*;

mod pinproject;
pub use pinproject::*;

mod blockon;
pub use blockon::*;

mod pending;
pub use pending::*;

mod pollonce;
pub use pollonce::*;

mod pollfn;
pub use pollfn::*;

mod ready;
pub use ready::*;

mod yieldnow;
pub use yieldnow::*;

mod zip;
pub use zip::*;

mod tryzip;
pub use tryzip::*;

mod or;
pub use or::*;

mod race;
pub use race::*;

mod catchunwind;
pub use catchunwind::*;

mod futureext;
pub use futureext::*;

mod fusedfuture;
pub use fusedfuture::*;

mod tryfuture;
pub use tryfuture::*;

mod stream;
pub use stream::*;

mod fusedstream;
pub use fusedstream::*;

mod trystream;
pub use trystream::*;

mod pusherror;
pub use pusherror::*;

mod poperror;
pub use poperror::*;

mod cachepadded;
pub use cachepadded::*;

mod single;
pub use single::*;

mod bounded;
pub use bounded::*;

mod unbounded;
pub use unbounded::*;

mod concurrentqueue;
pub use concurrentqueue::*;

mod senderror;
pub use senderror::*;

mod trysenderror;
pub use trysenderror::*;

mod recverror;
pub use recverror::*;

mod tryrecverror;
pub use tryrecverror::*;

mod send;
pub use send::*;

mod recv;
pub use recv::*;

mod sender;
pub use sender::*;

mod receiver;
pub use receiver::*;

mod channel;
pub use channel::*;

mod eventlistener;
pub use eventlistener::*;

mod executor;
pub use executor::*;

mod header;
pub use header::*;

mod taskvtable;
pub use taskvtable::*;

mod tasklayout;
pub use tasklayout::*;

mod rawtask;
pub use rawtask::*;

mod runnable;
pub use runnable::*;

mod task;
pub use task::*;

mod oncecell;
pub use oncecell::*;