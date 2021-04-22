use {
    crate::*,
    core::{
        future::Future,
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    },
};

#[derive(Debug)]
pub struct Send<'a,T> {
    pub(crate) sender: &'a Sender<T>,
    pub(crate) listener: Option<EventListener>,
    pub(crate) msg: Option<T>,
}

impl<'a,T> Unpin for Send<'a,T> { }

impl<'a,T> Future for Send<'a,T> {
    type Output = Result<(),SendError<T>>;

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = Pin::new(self);
        loop {
            let msg = this.msg.take().unwrap();
            match this.sender.try_send(msg) {
                Ok(()) => {
                    match this.sender.channel.queue.capacity() {
                        Some(1) => { },
                        Some(_) | None => this.sender.channel.send_ops.notify(1),
                    }
                    return Poll::Ready(Ok(()));
                },
                Err(TrySendError::Closed(msg)) => return Poll::Ready(Err(SendError(msg))),
                Err(TrySendError::Full(msg)) => this.msg = Some(msg),
            }
            match &mut this.listener {
                None => {
                    this.listener = Some(this.sender.channel.send_ops.listen());
                },
                Some(l) => {
                    match Pin::new(l).poll(cx) {
                        Poll::Ready(_) => {
                            this.listener = None;
                            continue;
                        },
                        Poll::Pending => return Poll::Pending,
                    }
                }
            }
        }
    }
}
