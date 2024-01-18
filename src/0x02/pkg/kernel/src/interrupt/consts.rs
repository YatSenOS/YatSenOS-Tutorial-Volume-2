// This is from https://github.com/rcore-os/rCore/blob/master/kernel/src/arch/x86_64/interrupt/consts.rs
// Reference: https://wiki.osdev.org/Exceptions

#[repr(u8)]
pub enum Interrupts {
    DivideError = 0,
    Debug = 1,
    NonMaskableInterrupt = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRangeExceeded = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverrun = 9,
    InvalidTSS = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtectionFault = 13,
    PageFault = 14,
    FloatingPointException = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SIMDFloatingPointException = 19,
    VirtualizationException = 20,
    SecurityException = 30,

    IrqBase = 0x20,
    Syscall = 0x80,
}

/// https://www.computerhope.com/jargon/i/irq.htm
/// https://wiki.osdev.org/IRQ
/// https://github.com/qemu/qemu/blob/aab8cfd4c3614a049b60333a3747aedffbd04150/include/hw/i386/microvm.h#L30-L50
#[repr(u8)]
pub enum Irq {
    Timer = 0,
    Keyboard = 1,
    Serial1 = 3,
    Serial0 = 4,
    Floppy = 6,
    Parallel = 7,
    RealTimeClock = 8,
    Ide0 = 14,
    Ide1 = 15,
    Error = 19,
    Spurious = 31,
}
