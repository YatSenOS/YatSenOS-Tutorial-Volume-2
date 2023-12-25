#![no_std]

#[macro_use]
extern crate log;

use core::ptr::{copy_nonoverlapping, write_bytes};

use x86_64::structures::paging::page::PageRange;
use x86_64::structures::paging::{mapper::*, *};
use x86_64::{align_up, PhysAddr, VirtAddr};
use xmas_elf::{program, ElfFile};

/// Map physical memory
///
/// map [0, max_addr) to virtual space [offset, offset + max_addr)
pub fn map_physical_memory(
    offset: u64,
    max_addr: u64,
    page_table: &mut impl Mapper<Size2MiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    trace!("Mapping physical memory...");
    let start_frame = PhysFrame::containing_address(PhysAddr::new(0));
    let end_frame = PhysFrame::containing_address(PhysAddr::new(max_addr));

    for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
        let page = Page::containing_address(VirtAddr::new(frame.start_address().as_u64() + offset));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            page_table
                .map_to(page, frame, flags, frame_allocator)
                .expect("Failed to map physical memory")
                .flush();
        }
    }
}

/// Map a range of memory
///
/// allocate frames and map to specified address (R/W)
pub fn map_range(
    addr: u64,
    count: u64,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<PageRange, MapToError<Size4KiB>> {
    let range_start = Page::containing_address(VirtAddr::new(addr));
    let range_end = range_start + count;

    trace!(
        "Page Range: {:?}({})",
        Page::range(range_start, range_end),
        count
    );

    // default flags for stack
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    for page in Page::range(range_start, range_end) {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            page_table
                .map_to(page, frame, flags, frame_allocator)?
                .flush();
        }
    }

    trace!(
        "Map hint: {:#x} -> {:#x}",
        addr,
        page_table
            .translate_page(range_start)
            .unwrap()
            .start_address()
    );

    Ok(Page::range(range_start, range_end))
}

/// Load & Map ELF file
///
/// load segments in ELF file to new frames and set page table
pub fn load_elf(
    elf: &ElfFile,
    physical_offset: u64,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let file_buf = elf.input.as_ptr();

    info!("Loading ELF file... @ {:#x}", file_buf as u64);

    for segment in elf.program_iter() {
        if segment.get_type().unwrap() != program::Type::Load {
            continue;
        }

        load_segment(
            file_buf,
            physical_offset,
            &segment,
            page_table,
            frame_allocator,
        )?
    }

    Ok(())
}

/// Load & Map ELF segment
///
/// load segment to new frame and set page table
fn load_segment(
    file_buf: *const u8,
    physical_offset: u64,
    segment: &program::ProgramHeader,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    trace!("Loading & mapping segment: {:#x?}", segment);

    let mem_size = segment.mem_size();
    let file_size = segment.file_size();
    let file_offset = segment.offset() & !0xfff;
    let virt_start_addr = VirtAddr::new(segment.virtual_addr());

    let mut page_table_flags = PageTableFlags::PRESENT;

    // FIXME: handle page table flags with segment flags
    unimplemented!("Handle page table flags with segment flags!");

    trace!("Segment page table flag: {:?}", page_table_flags);

    let start_page = Page::containing_address(virt_start_addr);
    let end_page = Page::containing_address(virt_start_addr + file_size - 1u64);
    let pages = Page::range_inclusive(start_page, end_page);

    let data = unsafe { file_buf.add(file_offset as usize) };

    for (idx, page) in pages.enumerate() {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let offset = idx as u64 * page.size();
        let count = if file_size - offset < page.size() {
            file_size - offset
        } else {
            page.size()
        };

        unsafe {
            copy_nonoverlapping(
                data.add(idx * page.size() as usize),
                (frame.start_address().as_u64() + physical_offset) as *mut u8,
                count as usize,
            );

            page_table
                .map_to(page, frame, page_table_flags, frame_allocator)?
                .flush();

            if count < page.size() {
                // zero the rest of the page
                trace!(
                    "Zeroing rest of the page: {:#x}",
                    page.start_address().as_u64()
                );
                write_bytes(
                    (frame.start_address().as_u64() + physical_offset + count) as *mut u8,
                    0,
                    (page.size() - count) as usize,
                );
            }
        }
    }

    if mem_size > file_size {
        // .bss section (or similar), which needs to be zeroed
        let zero_start = virt_start_addr + file_size;
        let zero_end = virt_start_addr + mem_size;

        // Map additional frames.
        let start_address = VirtAddr::new(align_up(zero_start.as_u64(), Size4KiB::SIZE));
        let start_page: Page = Page::containing_address(start_address);
        let end_page = Page::containing_address(zero_end);

        for page in Page::range_inclusive(start_page, end_page) {
            let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;

            unsafe {
                page_table
                    .map_to(page, frame, page_table_flags, frame_allocator)?
                    .flush();

                // zero bss section
                write_bytes(
                    (frame.start_address().as_u64() + physical_offset) as *mut u8,
                    0,
                    page.size() as usize,
                );
            }
        }
    }

    Ok(())
}
