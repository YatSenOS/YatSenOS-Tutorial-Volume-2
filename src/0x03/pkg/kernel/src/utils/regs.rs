use core::fmt;

#[repr(align(8), C)]
#[derive(Clone, Default, Copy)]
pub struct RegistersValue {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rdi: usize,
    pub rsi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    pub rbp: usize,
}

impl fmt::Debug for RegistersValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Registers")?;
        write!(f, "r15: 0x{:016x}, ", self.r15)?;
        write!(f, "r14: 0x{:016x}, ", self.r14)?;
        writeln!(f, "r13: 0x{:016x},", self.r13)?;
        write!(f, "r12: 0x{:016x}, ", self.r12)?;
        write!(f, "r11: 0x{:016x}, ", self.r11)?;
        writeln!(f, "r10: 0x{:016x},", self.r10)?;
        write!(f, "r9 : 0x{:016x}, ", self.r9)?;
        write!(f, "r8 : 0x{:016x}, ", self.r8)?;
        writeln!(f, "rdi: 0x{:016x},", self.rdi)?;
        write!(f, "rsi: 0x{:016x}, ", self.rsi)?;
        write!(f, "rdx: 0x{:016x}, ", self.rdx)?;
        writeln!(f, "rcx: 0x{:016x},", self.rcx)?;
        write!(f, "rbx: 0x{:016x}, ", self.rbx)?;
        write!(f, "rax: 0x{:016x}, ", self.rax)?;
        write!(f, "rbp: 0x{:016x}", self.rbp)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! as_handler {
    ($fn: ident) => {
        paste::item! {
            #[naked]
            pub extern "x86-interrupt" fn [<$fn _handler>](_sf: InterruptStackFrame) {
                unsafe {
                    core::arch::naked_asm!("
                    push rbp
                    push rax
                    push rbx
                    push rcx
                    push rdx
                    push rsi
                    push rdi
                    push r8
                    push r9
                    push r10
                    push r11
                    push r12
                    push r13
                    push r14
                    push r15
                    call {}
                    pop r15
                    pop r14
                    pop r13
                    pop r12
                    pop r11
                    pop r10
                    pop r9
                    pop r8
                    pop rdi
                    pop rsi
                    pop rdx
                    pop rcx
                    pop rbx
                    pop rax
                    pop rbp
                    iretq",
                    sym $fn);
                }
            }
        }
    };
}
