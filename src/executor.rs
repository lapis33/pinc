use crate::utils::mutex::Mutex;
use alloc::{boxed::Box, collections::VecDeque};
use core::{
    future::Future,
    pin::Pin,
    ptr::null,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

static TASKS: Mutex<VecDeque<Task>> = Mutex::new(VecDeque::new());

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl<T: Future<Output = ()> + Send + 'static> From<T> for Task {
    fn from(value: T) -> Self {
        Self {
            future: Box::pin(value),
        }
    }
}

fn waker() -> Waker {
    unsafe { Waker::from_raw(raw_waker()) }
}

fn raw_waker() -> RawWaker {
    fn skip(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, skip, skip, skip);
    RawWaker::new(null(), vtable)
}

pub fn push<T: Into<Task>>(task: T) {
    TASKS.lock().push_back(task.into());
}

pub fn run() {
    loop {
        let next = { TASKS.lock().pop_front() };

        if let Some(mut task) = next {
            let waker = waker();
            let mut context = Context::from_waker(&waker);

            if task.future.as_mut().poll(&mut context) == Poll::Pending {
                TASKS.lock().push_back(task);
            }
        }
    }
}
