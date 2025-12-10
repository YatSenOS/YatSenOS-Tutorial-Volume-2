use super::*;

/// A block device
pub trait BlockDevice<B>: Send + Sync + 'static
where
    B: BlockTrait,
{
    /// Returns the number of blocks in the device
    fn block_count(&self) -> FsResult<usize>;

    /// Reads a block from the device into the provided buffer
    fn read_block(&self, offset: usize, block: &mut B) -> FsResult;

    /// Writes a block to the device from the provided buffer
    fn write_block(&self, offset: usize, block: &B) -> FsResult;

    /// Returns the block size of the device
    fn block_size(&self) -> usize {
        B::size()
    }
}
