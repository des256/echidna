use {
    crate::*,
    core::{
        pin::Pin,
        task::{
            Context,
            Poll,
        },
        sync::atomic::Ordering,
        future::Future,
        fmt,
    },
    std::sync::Arc,
    std::process,
};

pub struct Receiver<T> {
    pub(crate) channel: Arc<Channel<T>>,
    pub(crate) listener: Option<EventListener>,
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Result<T,TryRecvError> {
        match self.channel.queue.pop() {
            Ok(msg) => {
                self.channel.send_ops.notify(1);
                Ok(msg)
            },
            Err(PopError::Empty) => Err(TryRecvError::Empty),
            Err(PopError::Closed) => Err(TryRecvError::Closed),
        }
    }

    pub fn recv(&self) -> Recv<'_,T> {
        Recv {
            receiver: self,
            listener: None,
        }
    }

    pub fn close(&self) -> bool {
        self.channel.close()
    }

    pub fn is_closed(&self) -> bool {
        self.channel.queue.is_closed()
    }

    pub fn is_empty(&self) -> bool {
        self.channel.queue.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.channel.queue.is_full()
    }

    pub fn len(&self) -> usize {
        self.channel.queue.len()
    }

    pub fn capacity(&self) -> Option<usize> {
        self.channel.queue.capacity()
    }

    pub fn receiver_count(&self) -> usize {
        self.channel.receiver_count.load(Ordering::SeqCst)
    }

    pub fn sender_count(&self) -> usize {
        self.channel.sender_count.load(Ordering::SeqCst)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        if self.channel.receiver_count.fetch_sub(1,Ordering::AcqRel) == 1 {
            self.channel.close();
        }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Receiver {{ .. }}")
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Receiver<T> {
        let count = self.channel.receiver_count.fetch_add(1,Ordering::Relaxed);
        if count > usize::MAX / 2 {
            process::abort();
        }
        Receiver {
            channel: self.channel.clone(),
            listener: None,
        }
    }
}

impl<T> Stream for Receiver<T> {
    type Item = T;
    fn poll_next(mut self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(listener) = self.listener.as_mut() {
                ready!(Pin::new(listener).poll(cx));
                self.listener = None;
            }
            loop {
                match self.try_recv() {
                    Ok(msg) => {
                        self.listener = None;
                        return Poll::Ready(Some(msg));
                    },
                    Err(TryRecvError::Closed) => {
                        self.listener = None;
                        return Poll::Ready(None);
                    },
                    Err(TryRecvError::Empty) => { },
                }
                match self.listener.as_mut() {
                    None => {
                        self.listener = Some(self.channel.stream_ops.listen());
                    },
                    Some(_) => {
                        break;
                    },
                }
            }
        }
    }
}

impl<T> FusedStream for Receiver<T> {
    fn is_terminated(&self) -> bool {
        self.channel.queue.is_closed() && self.channel.queue.is_empty()
    }
}
