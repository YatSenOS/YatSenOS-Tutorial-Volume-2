#![no_std]
#![no_main]

#[macro_use]
extern crate log;

use core::arch::asm;
use ysos_kernel as ysos;

boot::entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static boot::BootInfo) -> ! {
    ysos::init(boot_info);

    loop {
        info!("Hello World from YatSenOS v2!");

        for _ in 0..0x10000000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}
