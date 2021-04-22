use {
    core::fmt,
    std::error,
};

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub struct RecvError;

impl error::Error for RecvError { }

impl fmt::Display for RecvError {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"receiving from an empty and closed channel")
    }
}
