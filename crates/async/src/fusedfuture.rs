use core::{
    ops::DerefMut,
    pin::Pin,
    future::Future,
};

pub trait FusedFuture: Future {
    fn is_terminated(&self) -> bool;
}

impl<F: FusedFuture + ?Sized + Unpin> FusedFuture for &mut F {
    fn is_terminated(&self) -> bool {
        <F as FusedFuture>::is_terminated(&**self)
    }
}

impl<P> FusedFuture for Pin<P> where P: DerefMut + Unpin,P::Target: FusedFuture {
    fn is_terminated(&self) -> bool {
        <P::Target as FusedFuture>::is_terminated(&**self)
    }
}
