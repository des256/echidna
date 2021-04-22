use {
    crate::*,
    core::{
        future::Future,
        fmt,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
};

pin_project! {
    pub struct PollFn<F> {
        f: F,
    }
}

pub fn poll_fn<T,F>(f: F) -> PollFn<F> where F: FnMut(&mut Context<'_>) -> Poll<T> {
    PollFn { f }
}

impl<F> fmt::Debug for PollFn<F> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollFn").finish()
    }
}

impl<T,F> Future for PollFn<F> where F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<T> {
        let this = self.project();
        (this.f)(cx)
    }
}
