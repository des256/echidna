use core::{
    ops::DerefMut,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};

pub trait Stream {

    type Item;

    fn poll_next(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

    #[inline]
    fn size_hint(&self) -> (usize,Option<usize>) {
        (0,None)
    }
}

impl<S: ?Sized + Stream + Unpin> Stream for &mut S {

    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        S::poll_next(Pin::new(&mut **self),cx)
    }

    fn size_hint(&self) -> (usize,Option<usize>) {
        (**self).size_hint()
    }
}

impl<P> Stream for Pin<P> where P: DerefMut + Unpin,P::Target: Stream {

    type Item = <P::Target as Stream>::Item;

    fn poll_next(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().as_mut().poll_next(cx)
    }

    fn size_hint(&self) -> (usize,Option<usize>) {
        (**self).size_hint()
    }
}
