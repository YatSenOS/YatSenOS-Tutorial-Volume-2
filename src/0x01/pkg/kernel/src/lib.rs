#![no_std]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;
extern crate libm;

#[macro_use]
pub mod utils;
pub use utils::*;

mod drivers;

use boot::BootInfo;

pub fn init(_boot_info: &'static BootInfo) {
    drivers::serial::init(); // init serial output
    logger::init(); // init logger system

    info!("YatSenOS initialized.");
}

pub fn shutdown(boot_info: &'static BootInfo) -> ! {
    info!("YatSenOS shutting down.");
    unsafe {
        boot_info.system_table.runtime_services().reset(
            boot::ResetType::SHUTDOWN,
            boot::UefiStatus::SUCCESS,
            None,
        );
    }
}
