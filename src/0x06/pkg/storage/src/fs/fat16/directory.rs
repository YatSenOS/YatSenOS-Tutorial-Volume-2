//! Directory
//!
//! reference: <https://wiki.osdev.org/FAT#Directories_on_FAT12.2F16.2F32>

use super::*;

#[derive(Debug)]
pub struct Directory {
    /// The starting point of the directory listing.
    pub cluster: Cluster,
    /// Dir Entry of this directory, None for the root directory
    pub entry: Option<DirEntry>,
}

impl Directory {
    /// Create a new directory from a cluster number.
    pub fn new(cluster: Cluster) -> Self {
        Directory {
            cluster,
            entry: None,
        }
    }

    pub const fn root() -> Self {
        Directory {
            cluster: Cluster::ROOT_DIR,
            entry: None,
        }
    }

    pub fn from_entry(entry: DirEntry) -> Self {
        Directory {
            cluster: entry.cluster,
            entry: Some(entry),
        }
    }
}

impl core::fmt::Display for Directory {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Directory(cluster: {}, entry: {:?})",
            self.cluster, self.entry
        )
    }
}
