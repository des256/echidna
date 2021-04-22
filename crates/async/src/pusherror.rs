use {
    core::fmt,
    std::error,
};

pub enum PushError<T> {
    Full(T),
    Closed(T),
}

impl<T> PushError<T> {
    pub fn into_inner(self) -> T {
        match self {
            PushError::Full(t) => t,
            PushError::Closed(t) => t,
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            PushError::Full(_) => true,
            PushError::Closed(_) => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            PushError::Full(_) => false,
            PushError::Closed(_) => true,
        }
    }
}

impl<T: fmt::Debug> error::Error for PushError<T> { }

impl<T: fmt::Debug> fmt::Debug for PushError<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PushError::Full(t) => f.debug_tuple("Full").field(t).finish(),
            PushError::Closed(t) => f.debug_tuple("Closed").field(t).finish(),
        }
    }
}

impl<T> fmt::Display for PushError<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PushError::Full(_) => write!(f,"Full"),
            PushError::Closed(_) => write!(f,"Closed"),
        }
    }
}
