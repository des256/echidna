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
    pub struct PollOnce<F> {
        #[pin]
        f: F,
    }
}

pub fn poll_once<T,F>(f: F) -> PollOnce<F> where F: Future<Output = T> {
    PollOnce { f }
}

impl<F> fmt::Debug for PollOnce<F> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollOnce").finish()
    }
}

impl<T,F> Future for PollOnce<F> where F: Future<Output = T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().f.poll(cx) {
            Poll::Ready(t) => Poll::Ready(Some(t)),
            Poll::Pending => Poll::Ready(None),
        }
    }
}
