#[macro_use]
mod macros;
#[macro_use]
mod regs;

pub mod clock;
pub mod func;
pub mod logger;

pub use macros::*;
pub use regs::*;

use crate::proc::*;

pub const fn get_ascii_header() -> &'static str {
    concat!(
        r"
__  __      __  _____            ____  _____
\ \/ /___ _/ /_/ ___/___  ____  / __ \/ ___/
 \  / __ `/ __/\__ \/ _ \/ __ \/ / / /\__ \
 / / /_/ / /_ ___/ /  __/ / / / /_/ /___/ /
/_/\__,_/\__//____/\___/_/ /_/\____//____/

                                       v",
        env!("CARGO_PKG_VERSION")
    )
}

pub fn stack_thread_test() {
    let pid = spawn_kernel_thread(
        utils::func::stack_test,
        alloc::string::String::from("stack"),
        None,
    );

    // wait for progress exit
    wait(pid);
}

pub fn new_test_thread(id: &str) -> ProcessId {
    let proc_data = ProcessData::new();
    proc_data.set_env("id", id);

    spawn_kernel_thread(
        utils::func::test,
        format!("#{}_test", id),
        Some(proc_data),
    )

    // no wait
}

fn wait(pid: ProcessId) {
    loop {
        // FIXME: try to get the status of the process

        // HINT: it's better to use the exit code

        if /* FIXME: is the process exited? */ {
            x86_64::instructions::hlt();
        } else {
            break;
        }
    }
}
