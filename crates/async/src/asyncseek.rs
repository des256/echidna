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

pub trait AsyncSeek {

    fn poll_seek(self: Pin<&mut Self>,cx: &mut Context<'_>,pos: io::SeekFrom) -> Poll<io::Result<u64>>;
}

macro_rules! deref_async_seek {
    () => {
        fn poll_seek(mut self: Pin<&mut Self>,cx: &mut Context<'_>,pos: io::SeekFrom) -> Poll<io::Result<u64>> {
            Pin::new(&mut **self).poll_seek(cx,pos)
        }
    };
}

impl<T: ?Sized + AsyncSeek + Unpin> AsyncSeek for Box<T> {
    deref_async_seek!();
}

impl<T: ?Sized + AsyncSeek + Unpin> AsyncSeek for &mut T {
    deref_async_seek!();
}

impl<P> AsyncSeek for Pin<P> where P: DerefMut + Unpin,P::Target: AsyncSeek {
    fn poll_seek(self: Pin<&mut Self>,cx: &mut Context<'_>,pos: io::SeekFrom) -> Poll<io::Result<u64>> {
        self.get_mut().as_mut().poll_seek(cx,pos)
    }
}
