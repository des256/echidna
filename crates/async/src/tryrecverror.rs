use {
    core::fmt,
    std::error,
};

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub enum TryRecvError {
    Empty,
    Closed,
}

impl TryRecvError {
    pub fn is_emtpy(&self) -> bool {
        match self {
            TryRecvError::Empty => true,
            TryRecvError::Closed => false,
        }
    }
    
    pub fn is_closed(&self) -> bool {
        match self {
            TryRecvError::Empty => true,
            TryRecvError::Closed => false,
        }
    }
}

impl error::Error for TryRecvError { }

impl fmt::Display for TryRecvError {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TryRecvError::Empty => write!(f,"receiving from an empty channel"),
            TryRecvError::Closed => write!(f,"receiving from an empty and closed channel"),
        }
    }
}
