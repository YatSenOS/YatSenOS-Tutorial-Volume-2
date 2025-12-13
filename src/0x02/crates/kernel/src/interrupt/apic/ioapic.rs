/// The I/O APIC manages hardware interrupts for an SMP system.
///
/// [Intel Doc](http://www.intel.com/design/chipsets/datashts/29056601.pdf)
use bit_field::BitField;

/// Default physical address of IO APIC
pub const IOAPIC_ADDR: u64 = 0xFEC00000;

bitflags! {
    /// The redirection table starts at REG_TABLE and uses
    /// two registers to configure each interrupt.
    /// The first (low) register in a pair contains configuration bits.
    /// The second (high) register contains a bitmask telling which
    /// CPUs can serve that interrupt.
    struct RedirectionEntry: u32 {
        /// Interrupt disabled
        const DISABLED  = 0x00010000;
        /// Level-triggered (vs edge-)
        const LEVEL     = 0x00008000;
        /// Active low (vs high)
        const ACTIVELOW = 0x00002000;
        /// Destination is CPU id (vs APIC ID)
        const LOGICAL   = 0x00000800;
        /// None
        const NONE		= 0x00000000;
    }
}

pub struct IoApic {
    reg: *mut u32,
    data: *mut u32,
}

impl IoApic {
    pub unsafe fn new(addr: u64) -> Self {
        IoApic {
            reg: addr as *mut u32,
            data: (addr + 0x10) as *mut u32,
        }
    }

    pub fn disable_all(&mut self) {
        // Mark all interrupts edge-triggered, active high, disabled,
        // and not routed to any CPUs.
        for i in 0..=self.maxintr() {
            self.write_irq(i, RedirectionEntry::DISABLED, 0);
        }
    }

    fn read(&mut self, reg: u8) -> u32 {
        unsafe {
            self.reg.write_volatile(reg as u32);
            self.data.read_volatile()
        }
    }

    fn write(&mut self, reg: u8, data: u32) {
        unsafe {
            self.reg.write_volatile(reg as u32);
            self.data.write_volatile(data);
        }
    }

    fn write_irq(&mut self, irq: u8, flags: RedirectionEntry, dest: u8) {
        self.write(0x10 + 2 * irq, (32 + irq) as u32 | flags.bits());
        self.write(0x10 + 2 * irq + 1, (dest as u32) << 24);
    }

    pub fn enable(&mut self, irq: u8, cpuid: u8) {
        // Mark interrupt edge-triggered, active high,
        // enabled, and routed to the given cpuid,
        // which happens to be that cpu's APIC ID.
        self.write_irq(irq, RedirectionEntry::NONE, cpuid);
        trace!("Enable IOApic: IRQ={}, CPU={}", irq, cpuid);
    }

    pub fn disable(&mut self, irq: u8, cpuid: u8) {
        self.write_irq(irq, RedirectionEntry::DISABLED, cpuid);
    }

    pub fn id(&mut self) -> u8 {
        self.read(0x00).get_bits(24..28) as u8
    }

    pub fn version(&mut self) -> u8 {
        self.read(0x01).get_bits(0..8) as u8
    }

    pub fn maxintr(&mut self) -> u8 {
        self.read(0x01).get_bits(16..24) as u8
    }
}
