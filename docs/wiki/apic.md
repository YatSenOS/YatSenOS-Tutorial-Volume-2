# APIC 可编程中断控制器

## 什么是 APIC

在 x86/x64 计算机体系结构中，高级可编程中断控制器（APIC）是一种关键的硬件组件，旨在管理和协调系统内的中断请求。

中断是计算机系统中的一种异步通信机制，用于响应外部事件，如设备状态变化、错误条件或其他重要的系统通知。随着计算机系统的发展和性能需求的提高，早期的中断控制器（如 8259 PIC）逐渐显露出限制，特别是在多处理器系统的环境中。为了克服这些限制，高级可编程中断控制器（APIC）被引入，为计算机系统提供了更灵活、高效的中断处理机制。

APIC 不仅简单地分配中断向量，还提供了更为复杂的功能，如中断优先级、中断屏蔽、中断向量分发等。这使得它成为多处理器系统中协调中断处理的理想选择，并在大型、高性能的计算机系统中发挥关键作用。APIC 的作用不仅仅局限于中断处理，它还协助处理器间通信、同步和系统管理。通过提供多处理器系统中的高级中断控制和协同工作机制，APIC 极大地推动了操作系统和应用程序在复杂环境下的性能表现。

在本实验中，我们不会涉及到深入的 APIC 编程和使用，只需要专注于利用它实现基本的时钟中断和 I/O 设备中断。

## APIC 的初始化与编程

在基于 APIC 的系统中，每个 CPU 都由一个本地 APIC（LAPIC）控制。LAPIC 通过 MMIO（Memory Mapped I/O）方式映射到物理内存中的某个地址空间，这个地址空间称为 LAPIC 寄存器空间。同时，系统中还有一个 I/O APIC（IOAPIC），它是一个独立的芯片，负责管理系统中所有 I/O 设备的中断请求。I/O APIC 也通过 MMIO 方式映射到物理内存中的某个地址空间。

x2APIC 是 xAPIC 的变体和扩展，主要改进解决了支持的 CPU 数量和接口性能问题，他们都属于 LAPIC 的实现。在本实验中，我们将使用 xAPIC 来实现 LAPIC 的初始化和编程，在之后的描述中，出现的 APIC 均代指 xAPIC。

对于一个寄存器的读写操作可以由下列参考代码实现：

```rust
use core::ptr::{read_volatile, write_volatile};

pub struct XApic {
    addr: u64,
}

impl XApic {
    pub unsafe fn new(addr: u64) -> Self {
        XApic { addr }
    }

    unsafe fn read(&self, reg: u32) -> u32 {
        read_volatile((self.addr + reg as u64) as *const u32)
    }

    unsafe fn write(&mut self, reg: u32, value: u32) {
        write_volatile((self.addr + reg as u64) as *mut u32, value);
        self.read(0x20);
    }
}
```

APIC 的简单初始化过程包括以下几个步骤：

- 禁用 8259 PIC，使得系统只使用 APIC 进行中断处理。

     这一步被 UEFI BIOS 自动完成，我们无需关心。

- 检测系统中是否存在 APIC，在 `x86_64` 中可以通过如下代码获知：

    ```rust
    CpuId::new().get_feature_info().map(
        |f| f.has_apic()
    ).unwrap_or(false)
    ```

- 确定 APIC 的地址空间，即 LAPIC 和 IOAPIC 的 MMIO 地址空间。

    由于我们采用了虚拟地址空间，所以这里需要将物理地址映射到虚拟地址空间中，之后再进行 APIC 的相关操作。

- 操作 SPIV（Spurious Interrupt Vector Register, 0xF0）寄存器，启用 APIC 并设置 Spurious IRQ Vector。

- 设置计时器相关寄存器：

    - TDCR(0x3E0): Divide Configuration Register，设置分频系数。
    - TICR(0x380): Initial Count Register，设置初始计数值。
    - LVT Timer(0x320): Local Vector Table Timer，设置中断向量号和触发模式。

- 禁用 LVT LINT0, LVT LINT1，向对应寄存器写入 0x10000。

- 如果设备支持了性能计数器 LVT PC 溢出中断，则禁用它。

- 设置错误中断 LVT Error 到对应的中断向量号。

- 连续写入两次 0 以清除错误状态寄存器。

- 向 EOI 寄存器写入 0 以确认任何挂起的中断。

- 设置 ICR 寄存器：

    中断命令寄存器由两个 32 位寄存器组成，一个在 0x300，另一个在 0x310。它用于向不同的处理器发送中断。

    在写入 0x300 时发出中断，但在写入 0x310 时不发出中断。因此，要发送中断命令，应首先写入 0x310，然后写入 0x300。

    - Destination type(bit 18-19): 设置为 2，始终将中断发送给所有 APIC
    - Delivery mode(bit 8-10): 设置为 5，INIT De-assert 模式所需
    - Level(bit 14): 设置为 0，INIT De-assert 所需
    - Trigger Mode(bit 15): 设置为 1，INIT De-assert 所需

    设置完成后等待 Delivery Status(bit 12) 为 0。

- 设置 TPR 寄存器，允许接收中断。

以上过程会在实验任务文档中进行详细描述，具体细节和设置原因涉及对称多处理 SMP 等内容，不做理解要求，如有兴趣可以自行查阅参考资料了解。

## 参考资料

- [APIC - OSDev](https://wiki.osdev.org/APIC)
- [/arch/x86/kernel/apic/apic.c - Linux](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/x86/kernel/apic/apic.c?h=v6.7#n1525)
- [Symmetric Multiprocessing - OSDev](https://wiki.osdev.org/Symmetric_Multiprocessing)
- [Multiprocessing Support for Hobby OSes Explained](http://www.osdever.net/tutorials/view/multiprocessing-support-for-hobby-oses-explained)
