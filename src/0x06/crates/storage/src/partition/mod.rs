use core::marker::PhantomData;

use crate::*;

pub mod mbr;

/// Partition table trait
pub trait PartitionTable<T, B>
where
    T: BlockDevice<B>,
    B: BlockTrait,
    Self: Sized,
{
    /// Parse the partition table
    fn parse(inner: T) -> FsResult<Self>;

    /// Returns the partitions
    fn partitions(&self) -> FsResult<Vec<Partition<T, B>>>;
}

/// Identifies a partition on the disk.
#[derive(Clone, Copy)]
pub struct Partition<T, B>
where
    T: BlockDevice<B>,
    B: BlockTrait,
{
    inner: T,
    offset: usize,
    size: usize,
    _block: PhantomData<B>,
}

impl<T, B> Partition<T, B>
where
    T: BlockDevice<B>,
    B: BlockTrait,
{
    pub fn new(inner: T, offset: usize, size: usize) -> Self {
        Self {
            inner,
            offset,
            size,
            _block: PhantomData,
        }
    }
}

impl<T, B> core::fmt::Debug for Partition<T, B>
where
    T: BlockDevice<B>,
    B: BlockTrait,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Partition")
            .field("offset", &self.offset)
            .field("size", &self.size)
            .finish()
    }
}

impl<T, B> BlockDevice<B> for Partition<T, B>
where
    T: BlockDevice<B>,
    B: BlockTrait,
{
    fn block_count(&self) -> FsResult<usize> {
        self.inner.block_count()
    }

    fn read_block(&self, offset: usize, block: &mut B) -> FsResult {
        if offset >= self.size {
            return Err(FsError::InvalidOffset);
        }

        // FIXME: calculate the block offset for inner device
        // FIXME: read from the inner device
    }

    fn write_block(&self, offset: usize, block: &B) -> FsResult {
        if offset >= self.size {
            return Err(FsError::InvalidOffset);
        }

        // FIXME: calculate the block offset for inner device
        // FIXME: write to the inner device
    }
}
