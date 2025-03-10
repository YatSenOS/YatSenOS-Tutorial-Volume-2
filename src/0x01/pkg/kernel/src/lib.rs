#![no_std]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate log;
extern crate libm;

#[macro_use]
pub mod utils;
pub use utils::*;

mod drivers;

use boot::BootInfo;
use uefi::{runtime::ResetType, Status};

pub fn init(boot_info: &'static BootInfo) {
    unsafe {
        // set uefi system table
        uefi::table::set_system_table(boot_info.system_table.cast().as_ptr());
    }

    drivers::serial::init(); // init serial output
    logger::init(); // init logger system

    info!("YatSenOS initialized.");
}

pub fn shutdown() -> ! {
    info!("YatSenOS shutting down.");
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
