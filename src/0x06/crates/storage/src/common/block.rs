use crate::*;
use core::ops::Deref;

pub trait BlockTrait =
    AsMut<[u8]> + AsRef<[u8]> + SizedBlock + Default + Send + Sync + Clone + 'static;

pub trait SizedBlock {
    const BLOCK_SIZE: usize;

    fn size() -> usize {
        Self::BLOCK_SIZE
    }
}

pub type Block512 = Block<512>;
pub type Block4096 = Block<4096>;

/// A block of data.
#[derive(Clone)]
pub struct Block<const SIZE: usize> {
    contents: [u8; SIZE],
}

impl<const SIZE: usize> Block<SIZE> {
    /// Create a new block with data
    pub fn new(data: &[u8; SIZE]) -> Self {
        Self {
            contents: data.to_owned(),
        }
    }
}

impl<const SIZE: usize> Deref for Block<SIZE> {
    type Target = [u8; SIZE];

    /// For `&block[x..y] -> &[u8]`
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl<const SIZE: usize> AsRef<[u8]> for Block<SIZE> {
    fn as_ref(&self) -> &[u8] {
        &self.contents
    }
}

impl<const SIZE: usize> AsMut<[u8]> for Block<SIZE> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.contents
    }
}

impl<const SIZE: usize> Default for Block<SIZE> {
    fn default() -> Self {
        Self {
            contents: [0u8; SIZE],
        }
    }
}

impl<const SIZE: usize> SizedBlock for Block<SIZE> {
    const BLOCK_SIZE: usize = SIZE;
}

impl<const SIZE: usize> core::fmt::Debug for Block<SIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "Block:")?;
        for chunk in self.contents.chunks(32) {
            writeln!(
                f,
                "    {:016x} {:016x} {:016x} {:016x}",
                u64::from_be_bytes(chunk[0..8].try_into().unwrap()),
                u64::from_be_bytes(chunk[8..16].try_into().unwrap()),
                u64::from_be_bytes(chunk[16..24].try_into().unwrap()),
                u64::from_be_bytes(chunk[24..32].try_into().unwrap()),
            )?;
        }
        Ok(())
    }
}
