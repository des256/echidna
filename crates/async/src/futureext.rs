use {
    crate::*,
    core::{
        future::Future,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
    std::panic::UnwindSafe,
};

pub trait FutureExt: Future {
    fn poll(&mut self,cx: &mut Context<'_>) -> Poll<Self::Output> where Self: Unpin {
        Future::poll(Pin::new(self),cx)
    }

    fn or<F>(self,other: F) -> Or<Self,F> where Self: Sized,F: Future<Output = Self::Output> {
        Or {
            future1: self,
            future2: other,
        }
    }

    fn race<F>(self,other: F) -> Race<Self,F> where Self: Sized,F: Future<Output = Self::Output> {
        Race {
            future1: self,
            future2: other,
        }
    }

    fn catch_unwind(self) -> CatchUnwind<Self> where Self: Sized + UnwindSafe {
        CatchUnwind {
            inner: self,
        }
    }
}

impl<F: Future + ?Sized> FutureExt for F { }
