//! The filesystem trait definitions needed to implement new virtual filesystems
use crate::*;

use core::fmt::Debug;

/// File system trait
pub trait FileSystem: Debug + Sync + Send {
    /// Iterates over all direct children of this directory path
    fn read_dir(&self, path: &str) -> FsResult<Box<dyn Iterator<Item = Metadata> + Send>>;

    /// Opens the file at this path for reading
    fn open_file(&self, path: &str) -> FsResult<FileHandle>;

    /// Returns the file metadata for the file at this path
    fn metadata(&self, path: &str) -> FsResult<Metadata>;

    /// Returns true if a file or directory at path exists, false otherwise
    fn exists(&self, path: &str) -> FsResult<bool>;

    // ----------------------------------------------------
    // NOTE: following functions are not implemented (optional)
    // ----------------------------------------------------

    /// Creates a file at this path for writing
    fn create_file(&self, _path: &str) -> FsResult<FileHandle> {
        Err(FsError::NotSupported)
    }

    /// Opens the file at this path for appending
    fn append_file(&self, _path: &str) -> FsResult<FileHandle> {
        Err(FsError::NotSupported)
    }

    /// Removes the file at this path
    fn remove_file(&self, _path: &str) -> FsResult<FileHandle> {
        Err(FsError::NotSupported)
    }

    /// Removes the directory at this path
    fn remove_dir(&self, _path: &str) -> FsResult<FileHandle> {
        Err(FsError::NotSupported)
    }

    /// Copies the src path to the destination path within the same filesystem
    fn copy_file(&self, _src: &str, _dst: &str) -> FsResult {
        Err(FsError::NotSupported)
    }

    /// Moves the src path to the destination path within the same filesystem
    fn move_file(&self, _src: &str, _dst: &str) -> FsResult {
        Err(FsError::NotSupported)
    }

    /// Moves the src directory to the destination path within the same filesystem
    fn move_dir(&self, _src: &str, _dst: &str) -> FsResult {
        Err(FsError::NotSupported)
    }
}
