use volatile::{access::ReadOnly, VolatileRef};
use x86_64::{registers::rflags::RFlags, structures::idt::InterruptStackFrameValue, VirtAddr};

use crate::{memory::gdt::get_selector, RegistersValue};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProcessContextValue {
    pub regs: RegistersValue,
    pub stack_frame: InterruptStackFrameValue,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ProcessContext {
    value: ProcessContextValue,
}

impl ProcessContext {
    #[inline]
    pub fn as_mut(&mut self) -> VolatileRef<ProcessContextValue> {
        VolatileRef::from_mut_ref(&mut self.value)
    }

    #[inline]
    pub fn as_ref(&self) -> VolatileRef<'_, ProcessContextValue, ReadOnly> {
        VolatileRef::from_ref(&self.value)
    }

    #[inline]
    pub fn set_rax(&mut self, value: usize) {
        self.value.regs.rax = value;
    }

    #[inline]
    pub fn save(&mut self, context: &ProcessContext) {
        self.value = context.as_ref().as_ptr().read();
    }

    #[inline]
    pub fn restore(&self, context: &mut ProcessContext) {
        context.as_mut().as_mut_ptr().write(self.value);
    }

    pub fn init_stack_frame(&mut self, entry: VirtAddr, stack_top: VirtAddr) {
        self.value.stack_frame.stack_pointer = stack_top;
        self.value.stack_frame.instruction_pointer = entry;
        self.value.stack_frame.cpu_flags =
            RFlags::IOPL_HIGH | RFlags::IOPL_LOW | RFlags::INTERRUPT_FLAG;

        let selector = get_selector();
        self.value.stack_frame.code_segment = selector.code_selector;
        self.value.stack_frame.stack_segment = selector.data_selector;

        trace!("Init stack frame: {:#?}", &self.stack_frame);
    }
}

impl Default for ProcessContextValue {
    fn default() -> Self {
        Self {
            regs: RegistersValue::default(),
            stack_frame: InterruptStackFrameValue::new(
                VirtAddr::new(0x1000),
                SegmentSelector(0),
                RFlags::empty(),
                VirtAddr::new(0x2000),
                SegmentSelector(0),
            ),
        }
    }
}

impl core::ops::Deref for ProcessContext {
    type Target = ProcessContextValue;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl core::fmt::Debug for ProcessContext {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl core::fmt::Debug for ProcessContextValue {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut f = f.debug_struct("StackFrame");
        f.field("stack_top", &self.stack_frame.stack_pointer);
        f.field("cpu_flags", &self.stack_frame.cpu_flags);
        f.field("instruction_pointer", &self.stack_frame.instruction_pointer);
        f.field("regs", &self.regs);
        f.finish()
    }
}
