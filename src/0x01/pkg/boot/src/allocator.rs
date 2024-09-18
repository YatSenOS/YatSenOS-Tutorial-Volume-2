use uefi::boot::{AllocateType, MemoryType};
use x86_64::{structures::paging::*, PhysAddr};

/// Use `BootServices::allocate_pages()` as frame allocator
pub struct UEFIFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for UEFIFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let addr = uefi::boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1)
            .expect("Failed to allocate frame");
        let frame = PhysFrame::containing_address(PhysAddr::new(addr.as_ptr() as u64));
        Some(frame)
    }
}
