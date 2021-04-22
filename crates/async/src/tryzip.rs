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
    pub struct TryZip<F1, F2> where F1: Future,F2: Future {
        #[pin]
        future1: F1,
        output1: Option<F1::Output>,
        #[pin]
        future2: F2,
        output2: Option<F2::Output>,
    }
}

pub fn try_zip<T1,T2,E,F1,F2>(future1: F1, future2: F2) -> TryZip<F1,F2> where F1: Future<Output = Result<T1, E>>,F2: Future<Output = Result<T2, E>> {
    TryZip {
        future1: future1,
        output1: None,
        future2: future2,
        output2: None,
    }
}

impl<T1,T2,E,F1,F2> Future for TryZip<F1,F2> where F1: Future<Output = Result<T1,E>>,F2: Future<Output = Result<T2,E>> {
    type Output = Result<(T1,T2),E>;

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.output1.is_none() {
            if let Poll::Ready(out) = this.future1.poll(cx) {
                match out {
                    Ok(t) => *this.output1 = Some(Ok(t)),
                    Err(err) => return Poll::Ready(Err(err)),
                }
            }
        }

        if this.output2.is_none() {
            if let Poll::Ready(out) = this.future2.poll(cx) {
                match out {
                    Ok(t) => *this.output2 = Some(Ok(t)),
                    Err(err) => return Poll::Ready(Err(err)),
                }
            }
        }

        if this.output1.is_some() && this.output2.is_some() {
            let res1 = this.output1.take().unwrap();
            let res2 = this.output2.take().unwrap();
            let t1 = res1.map_err(|_| unreachable!()).unwrap();
            let t2 = res2.map_err(|_| unreachable!()).unwrap();
            Poll::Ready(Ok((t1,t2)))
        } else {
            Poll::Pending
        }
    }
}
