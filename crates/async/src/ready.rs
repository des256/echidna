use {
    core::{
        future::Future,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
};

#[derive(Debug)]
pub struct Ready<T>(Option<T>);

pub fn ready<T>(val: T) -> Ready<T> {
    Ready(Some(val))
}

impl<T> Unpin for Ready<T> { }

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>,_cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("`Ready` polled after completion"))
    }
}
