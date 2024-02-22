use crate::proc::PageTableContext;
use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

pub const USER_HEAP_START: usize = 0x4000_0000_0000;
pub const USER_HEAP_SIZE: usize = 1024 * 1024; // 1 MiB
const USER_HEAP_PAGE: usize = USER_HEAP_SIZE / crate::memory::PAGE_SIZE as usize;

pub static USER_ALLOCATOR: LockedHeap = LockedHeap::empty();

// NOTE: export mod user / call in the kernel init / after frame allocator
pub fn init() {
    init_user_heap().expect("User Heap Initialization Failed.");
    info!("User Heap Initialized.");
}

pub fn init_user_heap() -> Result<(), MapToError<Size4KiB>> {
    // Get current pagetable mapper
    let mapper = &mut PageTableContext::new().mapper();
    // Get global frame allocator
    let frame_allocator = &mut *super::get_frame_alloc_for_sure();

    // FIXME: use elf::map_range to allocate & map
    //        frames (R/W/User Access)

    unsafe {
        USER_ALLOCATOR
            .lock()
            .init(USER_HEAP_START as *mut u8, USER_HEAP_SIZE);
    }

    Ok(())
}
