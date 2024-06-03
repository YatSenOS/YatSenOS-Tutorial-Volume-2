use crate::memory::*;
use core::ptr::copy_nonoverlapping;

use alloc::sync::Arc;
use x86_64::{
    registers::control::{Cr3, Cr3Flags},
    structures::paging::*,
    VirtAddr,
};

pub struct Cr3RegValue {
    pub addr: PhysFrame,
    pub flags: Cr3Flags,
}

impl Cr3RegValue {
    pub fn new(addr: PhysFrame, flags: Cr3Flags) -> Self {
        Self { addr, flags }
    }
}

pub struct PageTableContext {
    pub reg: Arc<Cr3RegValue>,
}

impl PageTableContext {
    pub fn new() -> Self {
        let (frame, flags) = Cr3::read();
        Self {
            reg: Arc::new(Cr3RegValue::new(frame, flags)),
        }
    }

    /// Create a new page table object based on current page table.
    pub fn clone_level_4(&self) -> Self {
        // 1. alloc new page table
        let mut frame_alloc = crate::memory::get_frame_alloc_for_sure();
        let page_table_addr = frame_alloc
            .allocate_frame()
            .expect("Cannot alloc page table for new process.");

        // 2. copy current page table to new page table
        unsafe {
            copy_nonoverlapping::<PageTable>(
                physical_to_virtual(self.reg.addr.start_address().as_u64()) as *mut PageTable,
                physical_to_virtual(page_table_addr.start_address().as_u64()) as *mut PageTable,
                1,
            );
        }

        // 3. create page table object
        Self {
            reg: Arc::new(Cr3RegValue::new(page_table_addr, Cr3Flags::empty())),
        }
    }

    /// Load the page table to Cr3 register.
    pub fn load(&self) {
        unsafe { Cr3::write(self.reg.addr, self.reg.flags) }
    }

    /// Get the page table object by Cr3 register value.
    pub fn mapper(&self) -> OffsetPageTable<'static> {
        unsafe {
            OffsetPageTable::new(
                (physical_to_virtual(self.reg.addr.start_address().as_u64()) as *mut PageTable)
                    .as_mut()
                    .unwrap(),
                VirtAddr::new_truncate(*PHYSICAL_OFFSET.get().unwrap()),
            )
        }
    }
}

impl core::fmt::Debug for PageTableContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageTable")
            .field("addr", &self.reg.addr)
            .field("flags", &self.reg.flags)
            .finish()
    }
}
