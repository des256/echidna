use {
    core::{
        future::Future,
        fmt,
        marker::PhantomData,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
};

pub struct Pending<T> {
    _marker: PhantomData<T>,
}

pub fn pending<T>() -> Pending<T> {
    Pending {
        _marker: PhantomData,
    }
}

impl<T> Unpin for Pending<T> {}

impl<T> fmt::Debug for Pending<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pending").finish()
    }
}

impl<T> Future for Pending<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>,_: &mut Context<'_>) -> Poll<T> {
        Poll::Pending
    }
}
