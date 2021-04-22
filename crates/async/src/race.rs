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
};

pin_project! {
    #[derive(Debug)]
    pub struct Race<F1,F2> {
        #[pin]
        pub(crate) future1: F1,
        #[pin]
        pub(crate) future2: F2,
    }
}

pub fn race<T,F1,F2>(future1: F1,future2: F2) -> Race<F1,F2> where F1: Future<Output = T>,F2: Future<Output = T> {
    Race { future1, future2 }
}

impl<T,F1,F2> Future for Race<F1,F2> where F1: Future<Output = T>,F2: Future<Output = T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if rng::bool() {
            if let Poll::Ready(t) = this.future1.poll(cx) {
                return Poll::Ready(t);
            }
            if let Poll::Ready(t) = this.future2.poll(cx) {
                return Poll::Ready(t);
            }
        } else {
            if let Poll::Ready(t) = this.future2.poll(cx) {
                return Poll::Ready(t);
            }
            if let Poll::Ready(t) = this.future1.poll(cx) {
                return Poll::Ready(t);
            }
        }
        Poll::Pending
    }
}
