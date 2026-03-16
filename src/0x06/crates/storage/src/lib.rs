#![cfg_attr(not(test), no_std)]
#![allow(dead_code, unused_imports)]
#![feature(trait_alias)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;

#[macro_use]
pub mod common;
mod fs;
mod partition;

use alloc::{borrow::ToOwned, boxed::Box, string::String, sync::Arc, vec::Vec};

pub use common::*;
pub use fs::*;
pub use partition::*;
