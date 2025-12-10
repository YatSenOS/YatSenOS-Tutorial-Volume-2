pub const PAGE_SIZE: u64 = 4096;
pub const FRAME_SIZE: u64 = PAGE_SIZE;

pub static PHYSICAL_OFFSET: spin::Once<u64> = spin::Once::new();

pub fn init(boot_info: &'static boot::BootInfo) {
    PHYSICAL_OFFSET.call_once(|| boot_info.physical_memory_offset);

    info!("Physical Offset  : {:#x}", PHYSICAL_OFFSET.get().unwrap());
}

/// Convert a virtual address to a physical address.
#[inline(always)]
pub fn physical_to_virtual(addr: u64) -> u64 {
    addr + PHYSICAL_OFFSET
        .get()
        .expect("PHYSICAL_OFFSET not initialized")
}
