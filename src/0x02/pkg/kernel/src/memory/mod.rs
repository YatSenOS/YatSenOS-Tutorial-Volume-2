pub mod address;
pub mod allocator;
mod frames;

pub mod gdt;

pub use address::*;
pub use frames::*;

pub fn init(boot_info: &'static boot::BootInfo) {
    let memory_map = &boot_info.memory_map;

    let mut mem_size = 0;
    let mut usable_mem_size = 0;

    for item in memory_map.iter() {
        mem_size += item.page_count;
        if item.ty == boot::MemoryType::CONVENTIONAL {
            usable_mem_size += item.page_count;
        }
    }

    let (size, unit) = humanized_size(mem_size * PAGE_SIZE);
    info!("Physical Memory    : {:>7.*} {}", 3, size, unit);

    let (size, unit) = humanized_size(usable_mem_size * PAGE_SIZE);
    info!("Free Usable Memory : {:>7.*} {}", 3, size, unit);

    unsafe {
        init_FRAME_ALLOCATOR(BootInfoFrameAllocator::init(
            memory_map,
            usable_mem_size as usize,
        ));
    }

    info!("Frame Allocator initialized.");
}

pub fn humanized_size(size: u64) -> (f64, &'static str) {
    let bytes = size as f64;

    // use 1000 to keep the max length of the number is 3 digits
    if bytes < 1000f64 {
        (bytes, "  B")
    } else if (bytes / (1 << 10) as f64) < 1000f64 {
        (bytes / (1 << 10) as f64, "KiB")
    } else if (bytes / (1 << 20) as f64) < 1000f64 {
        (bytes / (1 << 20) as f64, "MiB")
    } else {
        (bytes / (1 << 30) as f64, "GiB")
    }
}
