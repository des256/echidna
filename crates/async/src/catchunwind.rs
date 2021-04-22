use {
    crate::*,
    core::{
        future::Future,
        marker::Send,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
    std::{
        any::Any,
        panic::{
            catch_unwind,
            AssertUnwindSafe,
            UnwindSafe,
        },
    },
};

pin_project! {
    #[derive(Debug)]
    pub struct CatchUnwind<F> {
        #[pin]
        pub(crate) inner: F,
    }
}

impl<F: Future + UnwindSafe> Future for CatchUnwind<F> {
    type Output = Result<F::Output, Box<dyn Any + Send>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        catch_unwind(AssertUnwindSafe(|| this.inner.poll(cx)))?.map(Ok)
    }
}
