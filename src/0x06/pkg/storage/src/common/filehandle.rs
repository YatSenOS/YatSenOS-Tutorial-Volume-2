use super::*;
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};

pub struct FileHandle {
    pub meta: Metadata,
    file: Box<dyn FileIO + Send>,
}

impl FileHandle {
    pub fn new(meta: Metadata, file: Box<dyn FileIO + Send>) -> Self {
        Self { meta, file }
    }
}

impl Deref for FileHandle {
    type Target = Box<dyn FileIO + Send>;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for FileHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

impl Debug for FileHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FileHandle")
            .field("meta", &self.meta)
            .finish()
    }
}
