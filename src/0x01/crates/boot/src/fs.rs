use core::ptr::NonNull;
use uefi::boot::*;
use uefi::proto::media::file::*;
use uefi::proto::media::fs::SimpleFileSystem;
use xmas_elf::ElfFile;

/// Open root directory
pub fn open_root() -> Directory {
    let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
        .expect("Failed to get handle for SimpleFileSystem");
    let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
        .expect("Failed to get FileSystem");

    fs.open_volume().expect("Failed to open volume")
}

/// Open file at `path`
pub fn open_file(path: &str) -> RegularFile {
    let mut buf = [0; 64];
    let cstr_path = uefi::CStr16::from_str_with_buf(path, &mut buf).unwrap();

    let handle = open_root()
        .open(cstr_path, FileMode::Read, FileAttribute::empty())
        .expect("Failed to open file");

    match handle.into_type().expect("Failed to into_type") {
        FileType::Regular(regular) => regular,
        _ => panic!("Invalid file type"),
    }
}

/// Load file to new allocated pages
pub fn load_file(file: &mut RegularFile) -> &'static mut [u8] {
    let mut info_buf = [0u8; 0x100];
    let info = file
        .get_info::<FileInfo>(&mut info_buf)
        .expect("Failed to get file info");

    let pages = info.file_size() as usize / 0x1000 + 1;

    let mem_start =
        uefi::boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages)
            .expect("Failed to allocate pages");

    let buf = unsafe { core::slice::from_raw_parts_mut(mem_start.as_ptr(), pages * 0x1000) };
    let len = file.read(buf).expect("Failed to read file");

    info!(
        "Load file \"{}\" to memory, size = {}",
        info.file_name(),
        len
    );

    &mut buf[..len]
}

/// Free ELF files for which the buffer was created using 'load_file'
pub fn free_elf(elf: ElfFile) {
    let buffer = elf.input;
    let pages = buffer.len() / 0x1000 + 1;
    let mem_start = NonNull::new(buffer.as_ptr() as *mut u8).expect("Invalid pointer");

    info!("Free ELF file, pages = {}, addr = {:#x?}", pages, mem_start);

    unsafe {
        uefi::boot::free_pages(mem_start, pages).expect("Failed to free pages");
    }
}
