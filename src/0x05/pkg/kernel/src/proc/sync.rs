use super::ProcessId;
use alloc::{collections::BTreeMap, vec::Vec};
use spin::Mutex;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SemaphoreId(u32);

impl SemaphoreId {
    pub fn new(key: u32) -> Self {
        Self(key)
    }
}

/// Mutex is required for Semaphore
#[derive(Debug, Clone)]
pub struct Semaphore {
    count: usize,
    wait_queue: Vec<ProcessId>,
}

/// Semaphore result
#[derive(Debug)]
pub enum SemaphoreResult {
    Ok,
    NotExist,
    Block(ProcessId),
    WakeUp(ProcessId),
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(value: usize) -> Self {
        Self {
            count: value,
            wait_queue: Vec::new(),
        }
    }

    /// Wait the semaphore (acquire/down/proberen)
    ///
    /// if the count is 0, then push the process into the wait queue
    /// else decrease the count and return Ok
    pub fn wait(&mut self, pid: ProcessId) -> SemaphoreResult {
        // FIXME: if the count is 0, then push pid into the wait queue
        //          return Block(pid)
        // FIXME: else decrease the count and return Ok
    }

    /// Signal the semaphore (release/up/verhogen)
    ///
    /// if the wait queue is not empty, then pop a process from the wait queue
    /// else increase the count
    pub fn signal(&mut self) -> SemaphoreResult {
        // FIXME: if the wait queue is not empty
        //          pop a process from the wait queue
        //          return WakeUp(pid)
        // FIXME: else increase the count and return Ok
    }
}

#[derive(Debug, Default)]
pub struct SemaphoreSet {
    sems: BTreeMap<SemaphoreId, Mutex<Semaphore>>,
}

impl SemaphoreSet {
    pub fn insert(&mut self, key: u32, value: usize) -> bool {
        trace!("Sem Insert: <{:#x}>{}", key, value);

        // FIXME: insert a new semaphore into the sems
        //          use `insert(/* ... */).is_none()`
    }

    pub fn remove(&mut self, key: u32) -> bool {
        trace!("Sem Remove: <{:#x}>", key);

        // FIXME: remove the semaphore from the sems
        //          use `remove(/* ... */).is_some()`
    }

    /// Wait the semaphore (acquire/down/proberen)
    pub fn wait(&self, key: u32, pid: ProcessId) -> SemaphoreResult {
        let sid = SemaphoreId::new(key);

        // FIXME: try get the semaphore from the sems
        //         then do it's operation
        // FIXME: return NotExist if the semaphore is not exist
    }

    /// Signal the semaphore (release/up/verhogen)
    pub fn signal(&self, key: u32) -> SemaphoreResult {
        let sid = SemaphoreId::new(key);

        // FIXME: try get the semaphore from the sems
        //         then do it's operation
        // FIXME: return NotExist if the semaphore is not exist
    }
}

impl core::fmt::Display for Semaphore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Semaphore({}) {:?}", self.count, self.wait_queue)
    }
}
