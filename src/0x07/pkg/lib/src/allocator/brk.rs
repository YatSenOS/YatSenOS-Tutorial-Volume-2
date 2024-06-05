use linked_list_allocator::LockedHeap;

use crate::*;

const HEAP_SIZE: usize = 8 * 1024 - 8; // 8 KiB

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    let heap_start = sys_brk(None).unwrap();
    let heap_end = heap_start + HEAP_SIZE;

    let ret = sys_brk(Some(heap_end)).expect("Failed to allocate heap");

    assert!(ret == heap_end, "Failed to allocate heap");

    unsafe { ALLOCATOR.lock().init(heap_start as *mut u8, HEAP_SIZE) };
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}
