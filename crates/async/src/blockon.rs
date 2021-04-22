use {
    crate::*,
    core::{
        future::Future,
        task::{
            Context,
            Poll,
        },
    },
};

pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    use std::cell::RefCell;
    use std::task::Waker;

    crate::pin!(future);

    fn parker_and_waker() -> (Parker,Waker) {
        let parker = Parker::new();
        let unparker = parker.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        (parker,waker)
    }

    thread_local! {
        static CACHE: RefCell<(Parker,Waker)> = RefCell::new(parker_and_waker());
    }

    CACHE.with(|cache| {
        match cache.try_borrow_mut() {
            Ok(cache) => {
                let (parker,waker) = &*cache;
                let cx = &mut Context::from_waker(&waker);
                loop {
                    match future.as_mut().poll(cx) {
                        Poll::Ready(output) => return output,
                        Poll::Pending => parker.park(),
                    }
                }
            },
            Err(_) => {
                let (parker,waker) = parker_and_waker();
                let cx = &mut Context::from_waker(&waker);
                loop {
                    match future.as_mut().poll(cx) {
                        Poll::Ready(output) => return output,
                        Poll::Pending => parker.park(),
                    }
                }
            },
        }
    })
}
