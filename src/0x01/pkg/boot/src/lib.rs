#![no_std]
use core::arch::asm;

pub use uefi::data_types::chars::*;
pub use uefi::data_types::*;
pub use uefi::prelude::SystemTable;
pub use uefi::proto::console::gop::{GraphicsOutput, ModeInfo};
pub use uefi::table::boot::{MemoryAttribute, MemoryDescriptor, MemoryType};
pub use uefi::table::runtime::*;
pub use uefi::table::Runtime;
pub use uefi::Status as UefiStatus;

use arrayvec::ArrayVec;
use x86_64::VirtAddr;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable};

pub mod allocator;
pub mod config;
pub mod fs;

pub use allocator::*;
pub use fs::*;

#[macro_use]
extern crate log;

pub type MemoryMap = ArrayVec<MemoryDescriptor, 256>;

/// This structure represents the information that the bootloader passes to the kernel.
pub struct BootInfo {
    /// The memory map
    pub memory_map: MemoryMap,

    /// The offset into the virtual address space where the physical memory is mapped.
    pub physical_memory_offset: u64,

    /// UEFI SystemTable
    pub system_table: SystemTable<Runtime>,
}

/// Get current page table from CR3
pub fn current_page_table() -> OffsetPageTable<'static> {
    let p4_table_addr = Cr3::read().0.start_address().as_u64();
    let p4_table = unsafe { &mut *(p4_table_addr as *mut PageTable) };
    unsafe { OffsetPageTable::new(p4_table, VirtAddr::new(0)) }
}

/// The entry point of kernel, set by BSP.
static mut ENTRY: usize = 0;

/// Jump to ELF entry according to global variable `ENTRY`
///
/// # Safety
///
/// This function is unsafe because the caller must ensure that the kernel entry point is valid.
pub unsafe fn jump_to_entry(bootinfo: *const BootInfo, stacktop: u64) -> ! {
    assert!(ENTRY != 0, "ENTRY is not set");
    asm!("mov rsp, {}; call {}", in(reg) stacktop, in(reg) ENTRY, in("rdi") bootinfo);
    unreachable!()
}

/// Set the entry point of kernel
///
/// # Safety
///
/// This function is unsafe because the caller must ensure that the kernel entry point is valid.
#[inline(always)]
pub unsafe fn set_entry(entry: usize) {
    ENTRY = entry;
}

/// This is copied from https://docs.rs/bootloader/0.10.12/src/bootloader/lib.rs.html
/// Defines the entry point function.
///
/// The function must have the signature `fn(&'static BootInfo) -> !`.
///
/// This macro just creates a function named `_start`, which the linker will use as the entry
/// point. The advantage of using this macro instead of providing an own `_start` function is
/// that the macro ensures that the function and argument types are correct.
#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        #[export_name = "_start"]
        pub extern "C" fn __impl_start(boot_info: &'static $crate::BootInfo) -> ! {
            // validate the signature of the program entry point
            let f: fn(&'static $crate::BootInfo) -> ! = $path;

            f(boot_info)
        }
    };
}
