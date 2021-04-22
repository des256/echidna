use {
    crate::*,
    core::{
        ops::DerefMut,
        pin::Pin,
    },
};

pub trait FusedStream: Stream {
    fn is_terminated(&self) -> bool;
}

impl<F: ?Sized + FusedStream + Unpin> FusedStream for &mut F {
    fn is_terminated(&self) -> bool {
        <F as FusedStream>::is_terminated(&**self)
    }
}

impl<P> FusedStream for Pin<P> where P: DerefMut + Unpin,P::Target: FusedStream {
    fn is_terminated(&self) -> bool {
        <P::Target as FusedStream>::is_terminated(&**self)
    }
}
