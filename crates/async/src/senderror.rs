use {
    core::fmt,
    std::error,
};

#[derive(PartialEq,Eq,Clone,Copy)]
pub struct SendError<T>(pub T);

impl<T> SendError<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> error::Error for SendError<T> {
}

impl<T> fmt::Debug for SendError<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"SendError(..)")
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"sending into a closed channel")
    }
}
