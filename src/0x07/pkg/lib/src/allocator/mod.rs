#[cfg(feature = "brk_alloc")]
mod brk;

#[cfg(feature = "brk_alloc")]
pub use brk::*;

#[cfg(feature = "kernel_alloc")]
mod kernel;

#[cfg(feature = "kernel_alloc")]
pub use kernel::*;
