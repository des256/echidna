use {
    core::fmt,
    std::error,
};

#[derive(Clone,Copy,Eq,PartialEq)]
pub enum PopError {
    Empty,
    Closed,
}

impl PopError {
    pub fn is_empty(&self) -> bool {
        match self {
            PopError::Empty => true,
            PopError::Closed => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            PopError::Empty => false,
            PopError::Closed => true,
        }
    }
}

impl error::Error for PopError { }

impl fmt::Debug for PopError {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopError::Empty => write!(f,"Empty"),
            PopError::Closed => write!(f,"Closed"),
        }
    }
}

impl fmt::Display for PopError {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopError::Empty => write!(f,"Empty"),
            PopError::Closed => write!(f,"Closed"),
        }
    }
}
