use {
    core::fmt,
    std::error,
};

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum TrySendError<T> {
    Full(T),
    Closed(T),
}

impl<T> TrySendError<T> {
    pub fn into_inner(self) -> T {
        match self {
            TrySendError::Full(t) => t,
            TrySendError::Closed(t) => t,
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            TrySendError::Full(_) => true,
            TrySendError::Closed(_) => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            TrySendError::Full(_) => false,
            TrySendError::Closed(_) => true,
        }
    }
}

impl<T> error::Error for TrySendError<T> { }

impl<T> fmt::Debug for TrySendError<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TrySendError::Full(..) => write!(f,"Full(..)"),
            TrySendError::Closed(..) => write!(f,"Closed(..)"),
        }
    }
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TrySendError::Full(..) => write!(f,"sending into full channel"),
            TrySendError::Closed(..) => write!(f,"sending into closed channel"),
        }
    }
}
