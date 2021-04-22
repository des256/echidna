use {
    crate::*,
    core::sync::atomic::AtomicUsize,
    std::sync::Arc,
};

// futures_core: Stream

pub(crate) struct Channel<T> {
    pub(crate) queue: ConcurrentQueue<T>,
    pub(crate) send_ops: Event,
    pub(crate) recv_ops: Event,
    pub(crate) stream_ops: Event,
    pub(crate) sender_count: AtomicUsize,
    pub(crate) receiver_count: AtomicUsize,
}

impl<T> Channel<T> {
    pub(crate) fn close(&self) -> bool {
        if self.queue.close() {
            self.send_ops.notify(usize::MAX);
            self.recv_ops.notify(usize::MAX);
            self.stream_ops.notify(usize::MAX);
            true
        }
        else {
            false
        }
    }
}

pub fn bounded<T>(cap: usize) -> (Sender<T>,Receiver<T>) {
    assert!(cap > 0,"capacity cannot be zero");
    let channel = Arc::new(Channel {
        queue: ConcurrentQueue::bounded(cap),
        send_ops: Event::new(),
        recv_ops: Event::new(),
        stream_ops: Event::new(),
        sender_count: AtomicUsize::new(1),
        receiver_count: AtomicUsize::new(1),
    });
    let s = Sender {
        channel: channel.clone(),
    };
    let r = Receiver {
        channel,
        listener: None,
    };
    (s,r)
}

pub fn unbounded<T>() -> (Sender<T>,Receiver<T>) {
    let channel = Arc::new(Channel {
        queue: ConcurrentQueue::unbounded(),
        send_ops: Event::new(),
        recv_ops: Event::new(),
        stream_ops: Event::new(),
        sender_count: AtomicUsize::new(1),
        receiver_count: AtomicUsize::new(1),
    });
    let s = Sender {
        channel: channel.clone(),
    };
    let r = Receiver {
        channel,
        listener: None,
    };
    (s,r)
}
