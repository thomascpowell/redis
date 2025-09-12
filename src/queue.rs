use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct Queue<T> {
    inner: Mutex<VecDeque<T>>,
    cvar: Condvar,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            inner: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }
    }
    pub fn push(&self, item: T) {
        let mut queue = self.inner.lock().unwrap();
        queue.push_back(item);
        self.cvar.notify_one();
    }
    pub fn wait_pop(&self) -> T {
        let mut queue = self.inner.lock().unwrap();
        loop {
            if let Some(item) = queue.pop_front() {
                return item;
            }
            queue = self.cvar.wait(queue).unwrap();
        }
    }
}
