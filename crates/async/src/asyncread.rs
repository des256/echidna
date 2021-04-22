#![warn(missing_debug_implementations,unreachable_pub)]
#![warn(clippy::all)]

use {
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

pub trait AsyncRead {

    fn poll_read(self: Pin<&mut Self>,cx: &mut Context<'_>,buf: &mut [u8]) -> Poll<io::Result<usize>>;

    fn poll_read_vectored(self: Pin<&mut Self>,cx: &mut Context<'_>,bufs: &mut [io::IoSliceMut<'_>]) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_read(cx, b);
            }
        }
        self.poll_read(cx, &mut [])
    }
}

macro_rules! deref_async_read {
    () => {

        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut **self).poll_read(cx, buf)
        }

        fn poll_read_vectored(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &mut [io::IoSliceMut<'_>],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut **self).poll_read_vectored(cx, bufs)
        }
    };
}

impl<T: ?Sized + AsyncRead + Unpin> AsyncRead for Box<T> {
    deref_async_read!();
}

impl<T: ?Sized + AsyncRead + Unpin> AsyncRead for &mut T {
    deref_async_read!();
}

impl<P> AsyncRead for Pin<P> where P: DerefMut + Unpin,P::Target: AsyncRead {

    fn poll_read(self: Pin<&mut Self>,cx: &mut Context<'_>,buf: &mut [u8]) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_read(cx,buf)
    }

    fn poll_read_vectored(self: Pin<&mut Self>,cx: &mut Context<'_>,bufs: &mut [io::IoSliceMut<'_>]) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_read_vectored(cx,bufs)
    }
}

macro_rules! delegate_async_read_to_stdio {
    () => {

        fn poll_read(mut self: Pin<&mut Self>,_: &mut Context<'_>,buf: &mut [u8]) -> Poll<io::Result<usize>> {
            Poll::Ready(io::Read::read(&mut *self, buf))
        }

        fn poll_read_vectored(mut self: Pin<&mut Self>,_: &mut Context<'_>,bufs: &mut [io::IoSliceMut<'_>]) -> Poll<io::Result<usize>> {
            Poll::Ready(io::Read::read_vectored(&mut *self, bufs))
        }
    };
}

impl AsyncRead for &[u8] {
    delegate_async_read_to_stdio!();
}
