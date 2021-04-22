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

pub trait AsyncWrite {

    fn poll_write(self: Pin<&mut Self>,cx: &mut Context<'_>,buf: &[u8]) -> Poll<io::Result<usize>>;

    fn poll_write_vectored(self: Pin<&mut Self>,cx: &mut Context<'_>,bufs: &[io::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        for b in bufs {
            if !b.is_empty() {
                return self.poll_write(cx, b);
            }
        }

        self.poll_write(cx, &[])
    }

    fn poll_flush(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>>;

    fn poll_close(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>>;
}

macro_rules! deref_async_write {
    () => {
        fn poll_write(mut self: Pin<&mut Self>,cx: &mut Context<'_>,buf: &[u8]) -> Poll<io::Result<usize>> {
            Pin::new(&mut **self).poll_write(cx,buf)
        }

        fn poll_write_vectored(mut self: Pin<&mut Self>,cx: &mut Context<'_>,bufs: &[io::IoSlice<'_>]) -> Poll<io::Result<usize>> {
            Pin::new(&mut **self).poll_write_vectored(cx,bufs)
        }

        fn poll_flush(mut self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut **self).poll_flush(cx)
        }

        fn poll_close(mut self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut **self).poll_close(cx)
        }
    };
}

impl<T: ?Sized + AsyncWrite + Unpin> AsyncWrite for Box<T> {
    deref_async_write!();
}

impl<T: ?Sized + AsyncWrite + Unpin> AsyncWrite for &mut T {
    deref_async_write!();
}

impl<P> AsyncWrite for Pin<P> where P: DerefMut + Unpin,P::Target: AsyncWrite {
    fn poll_write(self: Pin<&mut Self>,cx: &mut Context<'_>,buf: &[u8]) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_write(cx, buf)
    }

    fn poll_write_vectored(self: Pin<&mut Self>,cx: &mut Context<'_>,bufs: &[io::IoSlice<'_>]) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_write_vectored(cx, bufs)
    }

    fn poll_flush(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_mut().as_mut().poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_mut().as_mut().poll_close(cx)
    }
}

macro_rules! delegate_async_write_to_stdio {
    () => {
        fn poll_write(mut self: Pin<&mut Self>,_: &mut Context<'_>,buf: &[u8]) -> Poll<io::Result<usize>> {
            Poll::Ready(io::Write::write(&mut *self,buf))
        }

        fn poll_write_vectored(mut self: Pin<&mut Self>,_: &mut Context<'_>,bufs: &[io::IoSlice<'_>]) -> Poll<io::Result<usize>> {
            Poll::Ready(io::Write::write_vectored(&mut *self, bufs))
        }

        fn poll_flush(mut self: Pin<&mut Self>,_: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(io::Write::flush(&mut *self))
        }

        fn poll_close(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            self.poll_flush(cx)
        }
    };
}

impl AsyncWrite for Vec<u8> {
    delegate_async_write_to_stdio!();
}
