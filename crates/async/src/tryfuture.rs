use core::{
    pin::Pin,
    task::{
        Context,
        Poll,
    },
    future::Future,
};

mod private_try_future {
    use super::Future;

    pub trait Sealed { }

    impl<F,T,E> Sealed for F where F: ?Sized + Future<Output = Result<T,E>> { }
}

pub trait TryFuture : Future + private_try_future::Sealed {
    type Ok;
    type Error;

    fn try_poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<Self::Ok,Self::Error>>;
}

impl<F,T,E> TryFuture for F where F: ?Sized + Future<Output = Result<T,E>>,
{
    type Ok = T;
    type Error = E;

    #[inline]
    fn try_poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll(cx)
    }
}
