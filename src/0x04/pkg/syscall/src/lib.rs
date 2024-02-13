#![no_std]

use num_enum::TryFromPrimitive;

pub mod macros;

#[repr(u16)]
#[derive(Clone, Debug, TryFromPrimitive)]
pub enum Syscall {
    Read = 0,
    Write = 1,

    Spawn = 59,
    Exit = 60,
    WaitPid = 61,

    ListApp = 65531,
    Stat = 65532,
    Allocate = 65533,
    Deallocate = 65534,

    #[num_enum(default)]
    None = 65535,
}
