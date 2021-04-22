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
pub struct Recv<'a,T> {
    pub(crate) receiver: &'a Receiver<T>,
    pub(crate) listener: Option<EventListener>,
}

impl<'a,T> Unpin for Recv<'a,T> { }

impl<'a,T> Future for Recv<'a,T> {
    type Output = Result<T,RecvError>;

    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = Pin::new(self);
        loop {
            match this.receiver.try_recv() {
                Ok(msg) => {
                    match this.receiver.channel.queue.capacity() {
                        Some(1) => { },
                        Some(_) | None => this.receiver.channel.recv_ops.notify(1),
                    }
                    return Poll::Ready(Ok(msg));
                },
                Err(TryRecvError::Closed) => return Poll::Ready(Err(RecvError)),
                Err(TryRecvError::Empty) => { },
            }
            match &mut this.listener {
                None => {
                    this.listener = Some(this.receiver.channel.recv_ops.listen());
                },
                Some(l) => {
                    match Pin::new(l).poll(cx) {
                        Poll::Ready(_) => {
                            this.listener = None;
                            continue;
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
            }
        }
    }
}
