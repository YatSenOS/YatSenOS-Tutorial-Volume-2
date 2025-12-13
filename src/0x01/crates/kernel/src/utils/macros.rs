use alloc::string::ToString;
use crate::drivers::serial::{SERIAL, get_serial};
use core::{arch::asm, fmt::*};
use x86_64::instructions::interrupts;

/// Use spin mutex to control variable access
#[macro_export]
macro_rules! guard_access_fn {
    ($(#[$meta:meta])* $v:vis $fn:ident ($mutex:path : $ty:ty)) => {
        paste::item! {

            $(#[$meta])*
            #[inline(never)]
            #[allow(non_snake_case, dead_code)]
            $v fn $fn<'a>() -> Option<spin::MutexGuard<'a, $ty>> {
                $mutex.get().and_then(spin::Mutex::try_lock)
            }

            $(#[$meta])*
            #[inline(never)]
            #[allow(non_snake_case, dead_code)]
            $v fn [< $fn _for_sure >]<'a>() -> spin::MutexGuard<'a, $ty> {
                $mutex.get().and_then(spin::Mutex::try_lock).expect(
                    stringify!($mutex has not been initialized or lockable)
                )
            }
        }
    };
}

#[macro_export]
macro_rules! once_mutex {
    ($i:vis $v:ident: $t:ty) => {
        $i static $v: spin::Once<spin::Mutex<$t>> = spin::Once::new();

        paste::item! {
            #[allow(non_snake_case)]
            $i fn [<init_ $v>]([<val_ $v>]: $t) {
                $v.call_once(|| spin::Mutex::new([<val_ $v>]));
            }
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::utils::print_internal(format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n\r"));
    ($($arg:tt)*) => ($crate::print!("{}\n\r", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn print_internal(args: Arguments) {
    interrupts::without_interrupts(|| {
        if let Some(mut serial) = get_serial() {
            serial.write_fmt(args).unwrap();
        }
    });
}

#[allow(dead_code)]
#[cfg_attr(target_os = "none", panic_handler)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let stack_trace = collect_stack_trace(MAX_STACK_FRAMES);

    // force unlock serial for panic output
    unsafe { SERIAL.get().unwrap().force_unlock() };

    let location = if let Some(location) = info.location() {
        alloc::format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        )
    } else {
        "Unknown location".to_string()
    };

    error!(
        "\n\n\rERROR: panicked at {}\n\n\r{}\n",
        location,
        info.message()
    );

    if !stack_trace.is_empty() {
        println!("Stack trace (most recent call last):");
        for (idx, addr) in stack_trace.iter().enumerate() {
            println!("  #{:02}: {:#018x}", idx, addr);
        }
    } else {
        println!("Stack trace unavailable (frame-pointer chain empty).");
    }
    loop {}
}

fn collect_stack_trace(max_frames: usize) -> StackTrace {
    let mut frames = StackTrace::new();
    let mut rbp = read_frame_pointer();

    for _ in 0..max_frames {
        if rbp < core::mem::size_of::<usize>() || !is_canonical(rbp) {
            break;
        }

        unsafe {
            let next_rbp = *(rbp as *const usize);
            let ret_addr = *((rbp as *const usize).add(1));

            if ret_addr == 0 || !is_canonical(ret_addr) {
                break;
            }

            frames.push(ret_addr);

            if next_rbp <= rbp {
                break;
            }

            rbp = next_rbp;
        }
    }

    frames
}

const MAX_STACK_FRAMES: usize = 32;

struct StackTrace {
    frames: [usize; MAX_STACK_FRAMES],
    len: usize,
}

impl StackTrace {
    const fn new() -> Self {
        Self {
            frames: [0; MAX_STACK_FRAMES],
            len: 0,
        }
    }

    fn push(&mut self, addr: usize) {
        if self.len < self.frames.len() {
            self.frames[self.len] = addr;
            self.len += 1;
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn iter(&self) -> core::slice::Iter<'_, usize> {
        self.frames[..self.len].iter()
    }
}

#[inline(always)]
fn read_frame_pointer() -> usize {
    let rbp: usize;
    unsafe {
        asm!("mov {}, rbp", out(reg) rbp, options(nomem, nostack, preserves_flags));
    }
    rbp
}

#[inline(always)]
fn is_canonical(addr: usize) -> bool {
    let upper = addr >> 47;
    upper == 0 || upper == 0x1ffff
}

