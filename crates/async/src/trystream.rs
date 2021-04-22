use {
    crate::*,
    core::{
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
};

mod private_try_stream {

    use super::Stream;

    pub trait Sealed { }

    impl<S,T,E> Sealed for S where S: ?Sized + Stream<Item = Result<T,E>> { }
}

pub trait TryStream: Stream + private_try_stream::Sealed {

    type Ok;
    type Error;

    fn try_poll_next(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Result<Self::Ok, Self::Error>>>;
}

impl<S,T,E> TryStream for S where S: ?Sized + Stream<Item = Result<T,E>>,
{
    type Ok = T;
    type Error = E;

    fn try_poll_next(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Result<Self::Ok, Self::Error>>> {
        self.poll_next(cx)
    }
}
