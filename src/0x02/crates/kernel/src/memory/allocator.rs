use linked_list_allocator::LockedHeap;
use x86_64::VirtAddr;
use core::ptr::addr_of_mut;

pub const HEAP_SIZE: usize = 8 * 1024 * 1024; // 8 MiB

/// Use linked_list_allocator for kernel heap
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    // static buffer for kernel heap
    // will be allocated on the bss section when the kernel is load
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    let heap_start = VirtAddr::from_ptr(addr_of_mut!(HEAP));
    let heap_end = heap_start + HEAP_SIZE as u64;

    unsafe {
        ALLOCATOR.lock().init(addr_of_mut!(HEAP) as *mut u8, HEAP_SIZE);
    }

    debug!(
        "Kernel Heap      : 0x{:016x}-0x{:016x}",
        heap_start.as_u64(),
        heap_end.as_u64()
    );

    let (size, unit) = crate::humanized_size(HEAP_SIZE as u64);
    info!("Kernel Heap Size : {:>7.*} {}", 3, size, unit);

    info!("Kernel Heap Initialized.");
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}
