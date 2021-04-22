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
    /// Future for the [`zip()`] function.
    #[derive(Debug)]
    pub struct Zip<F1,F2> where F1: Future,F2: Future {
        #[pin]
        future1: F1,
        output1: Option<F1::Output>,
        #[pin]
        future2: F2,
        output2: Option<F2::Output>,
    }
}

pub fn zip<F1,F2>(future1: F1,future2: F2) -> Zip<F1,F2> where F1: Future,F2: Future {
    Zip {
        future1: future1,
        output1: None,
        future2: future2,
        output2: None,
    }
}

impl<F1,F2> Future for Zip<F1,F2> where F1: Future,F2: Future {
    type Output = (F1::Output,F2::Output);

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.output1.is_none() {
            if let Poll::Ready(out) = this.future1.poll(cx) {
                *this.output1 = Some(out);
            }
        }

        if this.output2.is_none() {
            if let Poll::Ready(out) = this.future2.poll(cx) {
                *this.output2 = Some(out);
            }
        }

        if this.output1.is_some() && this.output2.is_some() {
            Poll::Ready((this.output1.take().unwrap(),this.output2.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}
