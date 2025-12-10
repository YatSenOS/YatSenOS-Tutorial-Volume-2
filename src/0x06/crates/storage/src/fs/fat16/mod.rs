pub mod bpb;
pub mod directory;
pub mod direntry;
pub mod file;
pub mod impls;

use crate::*;
use directory::Directory;
use direntry::*;
use file::File;

use bpb::Fat16Bpb;

const BLOCK_SIZE: usize = 512;

/// Identifies a Fat16 filesystem on the disk.
pub struct Fat16 {
    handle: Fat16Handle,
}

impl Fat16 {
    pub fn new(inner: impl BlockDevice<Block512>) -> Self {
        Self {
            handle: Arc::new(Fat16Impl::new(inner)),
        }
    }
}

type Fat16Handle = Arc<Fat16Impl>;

/// The Fat16 filesystem.
///
/// The partition is a collection of clusters.
/// BPB (Boot Parameter Block) is the first sector of the partition.
/// The BPB contains information about the filesystem.
///
/// [ Fat16 BPB ] [ Data ]
pub struct Fat16Impl {
    pub(crate) inner: Box<dyn BlockDevice<Block512>>,
    pub bpb: Fat16Bpb,
    pub fat_start: usize,
    pub first_data_sector: usize,
    pub first_root_dir_sector: usize,
}

impl core::fmt::Debug for Fat16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Fat16")
            .field("bpb", &self.handle.bpb)
            .finish()
    }
}

impl core::fmt::Debug for Fat16Impl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Fat16Impl").field("bpb", &self.bpb).finish()
    }
}
