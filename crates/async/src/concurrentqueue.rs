use {
    crate::*,
    core::{
        fmt,
        marker::{
            Send,
            Sync,
        },
    },
    std::panic::{
        UnwindSafe,
        RefUnwindSafe,
    },
};

enum Inner<T> {
    Single(Single<T>),
    Bounded(Box<Bounded<T>>),
    Unbounded(Box<Unbounded<T>>),
}

pub struct ConcurrentQueue<T>(Inner<T>);

unsafe impl<T: Send> Send for ConcurrentQueue<T> { }
unsafe impl<T: Send> Sync for ConcurrentQueue<T> { }

impl<T> UnwindSafe for ConcurrentQueue<T> { }
impl<T> RefUnwindSafe for ConcurrentQueue<T> { }

impl<T> ConcurrentQueue<T> {
    pub fn bounded(cap: usize) -> ConcurrentQueue<T> {
        if cap == 1 {
            ConcurrentQueue(Inner::Single(Single::new()))
        }
        else {
            ConcurrentQueue(Inner::Bounded(Box::new(Bounded::new(cap))))
        }
    }

    pub fn unbounded() -> ConcurrentQueue<T> {
        ConcurrentQueue(Inner::Unbounded(Box::new(Unbounded::new())))
    }

    pub fn push(&self,value: T) -> Result<(),PushError<T>> {
        match &self.0 {
            Inner::Single(q) => q.push(value),
            Inner::Bounded(q) => q.push(value),
            Inner::Unbounded(q) => q.push(value),
        }
    }

    pub fn pop(&self) -> Result<T,PopError> {
        match &self.0 {
            Inner::Single(q) => q.pop(),
            Inner::Bounded(q) => q.pop(),
            Inner::Unbounded(q) => q.pop(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Inner::Single(q) => q.is_empty(),
            Inner::Bounded(q) => q.is_empty(),
            Inner::Unbounded(q) => q.is_empty(),
        }
    }

    pub fn is_full(&self) -> bool {
        match &self.0 {
            Inner::Single(q) => q.is_full(),
            Inner::Bounded(q) => q.is_full(),
            Inner::Unbounded(q) => q.is_full(),
        }
    }

    pub fn len(&self) -> usize {
        match &self.0 {
            Inner::Single(q) => q.len(),
            Inner::Bounded(q) => q.len(),
            Inner::Unbounded(q) => q.len(),
        }
    }

    pub fn capacity(&self) -> Option<usize> {
        match &self.0 {
            Inner::Single(_) => Some(1),
            Inner::Bounded(q) => Some(q.capacity()),
            Inner::Unbounded(_) => None,
        }
    }

    pub fn close(&self) -> bool {
        match &self.0 {
            Inner::Single(q) => q.close(),
            Inner::Bounded(q) => q.close(),
            Inner::Unbounded(q) => q.close(),
        }
    }

    pub fn is_closed(&self) -> bool {
        match &self.0 {
            Inner::Single(q) => q.is_closed(),
            Inner::Bounded(q) => q.is_closed(),
            Inner::Unbounded(q) => q.is_closed(),
        }
    }
}

impl<T> fmt::Debug for ConcurrentQueue<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConcurrentQueue")
            .field("len",&self.len())
            .field("capacity",&self.capacity())
            .field("is_closed",&self.is_closed())
            .finish()
    }
}
