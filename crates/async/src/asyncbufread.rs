#![warn(missing_debug_implementations,unreachable_pub)]
#![warn(clippy::all)]

use {
    crate::*,
    core::{
        ops::DerefMut,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
    std::io,
};

pub trait AsyncBufRead: AsyncRead {

    fn poll_fill_buf(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>>;

    fn consume(self: Pin<&mut Self>, amt: usize);
}

macro_rules! deref_async_buf_read {
    () => {

        fn poll_fill_buf(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
            Pin::new(&mut **self.get_mut()).poll_fill_buf(cx)
        }

        fn consume(mut self: Pin<&mut Self>,amt: usize) {
            Pin::new(&mut **self).consume(amt)
        }
    };
}

impl<T: ?Sized + AsyncBufRead + Unpin> AsyncBufRead for Box<T> {
    deref_async_buf_read!();
}

impl<T: ?Sized + AsyncBufRead + Unpin> AsyncBufRead for &mut T {
    deref_async_buf_read!();
}

impl<P> AsyncBufRead for Pin<P> where P: DerefMut + Unpin,P::Target: AsyncBufRead {

    fn poll_fill_buf(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        self.get_mut().as_mut().poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>,amt: usize) {
        self.get_mut().as_mut().consume(amt)
    }
}

macro_rules! delegate_async_buf_read_to_stdio {
    () => {
        fn poll_fill_buf(self: Pin<&mut Self>,_: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
            Poll::Ready(io::BufRead::fill_buf(self.get_mut()))
        }

        fn consume(self: Pin<&mut Self>,amt: usize) {
            io::BufRead::consume(self.get_mut(),amt)
        }
    };
}

impl AsyncBufRead for &[u8] {
    delegate_async_buf_read_to_stdio!();
}
