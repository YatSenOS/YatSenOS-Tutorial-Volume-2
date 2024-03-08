use core::{
    hint::spin_loop,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::*;

pub struct SpinLock {
    bolt: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            bolt: AtomicBool::new(false),
        }
    }

    pub fn acquire(&self) {
        // FIXME: acquire the lock, spin if the lock is not available
    }

    pub fn release(&self) {
        // FIXME: release the lock
    }
}

unsafe impl Sync for SpinLock {} // Why? Check reflection question 5

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Semaphore {
    /* FIXME: record the sem key */
}

impl Semaphore {
    pub const fn new(key: u32) -> Self {
        Semaphore { key }
    }

    #[inline(always)]
    pub fn init(&self, value: usize) -> bool {
        sys_new_sem(self.key, value)
    }

    /* FIXME: other functions with syscall... */
}

unsafe impl Sync for Semaphore {}

#[macro_export]
macro_rules! semaphore_array {
    [$($x:expr),+ $(,)?] => {
        [ $($crate::Semaphore::new($x),)* ]
    }
}
