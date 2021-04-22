use {
    crate::*,
    core::{
        sync::atomic::Ordering,
        fmt,
    },
    std::{
        sync::Arc,
        process,
    }
};

pub struct Sender<T> {
    pub(crate) channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn try_send(&self,msg: T) -> Result<(),TrySendError<T>> {
        match self.channel.queue.push(msg) {
            Ok(()) => {
                self.channel.recv_ops.notify(1);
                self.channel.stream_ops.notify(usize::MAX);
                Ok(())
            },
            Err(PushError::Full(msg)) => Err(TrySendError::Full(msg)),
            Err(PushError::Closed(msg)) => Err(TrySendError::Closed(msg)),
        }
    }

    pub fn send(&self,msg: T) -> Send<'_,T> {
        Send {
            sender: self,
            listener: None,
            msg: Some(msg),
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

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if self.channel.sender_count.fetch_sub(1,Ordering::AcqRel) == 1 {
            self.channel.close();
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Sender {{ .. }}")
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        let count = self.channel.sender_count.fetch_add(1,Ordering::Relaxed);
        if count > usize::MAX / 2 {
            process::abort();
        }
        Sender {
            channel: self.channel.clone(),
        }
    }
}
