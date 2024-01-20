# 实验二：中断处理

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 合并实验代码

!!! tip "如何使用本次参考代码"

    本次给出的参考代码为**增量补充**，即在上一次实验的基础上进行修改和补充。因此，你需要将本次参考代码与上一次实验的代码进行合并。

    文件的目录与上一次实验相同，因此你可以直接将本次参考代码的 `src` 目录下的文件复制到上一次实验的目录结构下，覆盖同名文件。

    合并后的代码并不能直接运行，你需要基于合并后的代码、按照如下文档进行修改补充，才能逐步实现本次实验的功能。

在 `pkg/kernel/src/memory` 文件夹中，增量代码补充包含了如下的模块：

- `address.rs`：定义了物理地址到虚拟地址的转换函数，这一模块接受启动结构体提供的物理地址偏移，从而对物理地址进行转换。此部分内容在 lab 1 中已经有所涉及，你可以参考[完整的物理地址映射](https://os.phil-opp.com/paging-implementation/#map-the-complete-physical-memory)进行深入了解。
- `frames.rs`：利用 bootloader 传入的内存布局进行物理内存帧分配，实现 x86_64 的 `FrameAllocator` trait。**本次实验中不会涉及，后续实验中会用到。**
- `gdt.rs`：定义 TSS 和 GDT，为内核提供内存段描述符和任务状态段。
- `allocator.rs`：注册内核堆分配器，为内核堆分配提供能力。从而能够在内核中使用 `alloc` 提供的操作和数据结构，进行动态内存分配的操作，如 `Vec`、`String`、`Box` 等。

!!! note "动态内存分配算法在这里不做要求，本次实验直接使用现有的库赋予内核堆分配能力。"

在 `pkg/kernel/src/interrupt` 文件夹中，增量代码补充包含了如下的模块：

- `apic`：有关 XAPIC、IOAPIC 和 LAPIC 的定义和实现。
- `consts.rs`：有关于中断向量、IRQ 的常量定义。
- `exceptions.rs`：包含了 CPU 异常的处理函数，并暴露 `register_idt` 用于注册 IDT。
- `mod.rs`：定义了 `init` 函数，用于初始化中断系统，加载 IDT。

## GDT 与 TSS

!!! note "请阅读 [x64 数据结构概述](../../wiki/structures.md) 部分，了解 x64 架构中的 GDT、TSS 和 IDT。"

在我们的操作系统中，GDT、TSS 和 IDT 均属于全局静态的数据结构，因此我们将它们定义为 `static` 类型，并使用 `lazy_static` 宏来实现懒加载，其本质上也是通过 `Once` 来保护全局对象，但是它的初始化函数无需参数传递，因此可以直接声明，无需手动调用 `call_once` 函数来传递不同的初始化参数。

```rust
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // do something...

        tss
    };
}
```

你需要参考上下文，在 `src/memory/gdt.rs` 中补全 TSS 的中断栈表，为 Double Fault 和 Page Fault 准备独立的栈。

## 注册中断处理程序

!!! note "请阅读 [CPU 中断处理](../../wiki/interrupts.md) 部分，学习中断基本知识。"

在 `src/interrupt/mod.rs` 中，参考如下代码，将中断描述符表的注册委托给各个模块。

```rust
// mod clock;
// mod serial;
mod exceptions;

use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            exceptions::register_idt(&mut idt);
            // TODO: clock::register_idt(&mut idt);
            // TODO: serial::register_idt(&mut idt);
        }
        idt
    };
}
```

在预期的项目结构中，你需要在 `src/interrupt` 目录下创建 `exceptions.rs`、`clock.rs` 和 `serial.rs` 三个文件：

`exceptions.rs` 中描述了 CPU 异常的处理，这些异常由 CPU 在内部生成，用于提醒正在运行的内核需要其注意的事件或情况。`x86_64` 的 `InterruptDescriptorTable` 中为这些异常处理函数提供了定义，如 `divide_error`、`double_fault` 等。

对于中断请求（IRQ）和硬件中断，我们将在独立的文件中进行处理。`clock.rs` 中描述了时钟中断的处理，`serial.rs` 中描述了串口输入中断的处理。

对于软件中断，如在 `x86` 架构中的系统调用 `int 0x80`，我们将在 `syscall.rs` 中进行处理。从而统一地对中断进行代码组织。这部分内容将在后续实验中进行实现。

之后按照项目规范，为 `interrupt` 模块添加 `pub fn init()` 函数，将中断系统的初始化工作统一起来：

```rust
/// init interrupt system
pub fn init() {
    // Load the Interrupt Descriptor Table
    IDT.load();

    // FIXME: check and init APIC

    info!("Interrupts Initialized.");
}
```

在 `exception.rs` 中，参考如下代码，为各种 CPU 异常注册中断处理程序。

```rust
pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    /* more... */
}
```

由于中断处理函数需要遵循相应的调用约定（calling convention），因此需要使用 `extern "x86-interrupt"` 修饰符来声明函数，例如：

```rust
pub extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR\n\n{:#?}", stack_frame);
}
```

你可以参考此 [说明](https://os.phil-opp.com/cpu-exceptions/#an-idt-type) 来了解相关结构体的定义和调用约定的信息。

对于一些特殊的异常情况，如 double fault，为了避免可能的 triple fault 引发系统重置，需要为它准备一个独立的栈。这些特殊的栈在 IST 中进行描述，并将相关信息存储在 TSS 中，在 `src/memory/gdt.rs` 中进行管理。在注册处理程序时如下所示：

```rust
pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    /* more... */
    idt.double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    /* more... */
}
```

## 初始化 APIC

可编程中断控制器（PIC）是构成 x86 架构的重要组成部分之一。得益于这一类芯片的存在，x86 架构得以实现中断驱动的操作系统设计。中断是一种处理外部事件的机制，允许计算机在运行过程中响应异步的、不可预测的事件。PIC 的引入为处理中断提供了关键的硬件支持。

最初，x86 架构使用的是 8259 可编程中断控制器，它是一种级联的、基于中断请求线（IRQ）的硬件设备。随着计算机体系结构的发展和性能需求的提高，单一的 8259 PIC 逐渐显露出瓶颈，无法满足现代系统对更高级别中断处理的需求。

为了解决这个问题，高级可编程中断控制器（APIC）被引入到 x86 架构中。APIC 提供了更灵活的中断处理机制，支持更多的中断通道和更先进的中断处理功能。它采用了分布式的架构，允许多个处理器在系统中独立处理中断，从而提高了整个系统的并行性和性能。

!!! note "请阅读 [APIC 可编程中断控制器](../../wiki/apic.md) 部分，了解什么是 APIC 可编程中断控制器。"

你需要在 `src/interrupt/apic/xapic.rs` 中补全 APIC 的初始化代码，以便在后续实验中使用 APIC 实现时钟中断和 I/O 设备中断。

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

!!! tip "你应当使用 `src/memory/address.rs` 中提供的函数进行 MMIO 地址到虚拟地址的映射"

下面以部分操作为例讲解如何进行 APIC 的初始化。

- 检测系统中是否存在 APIC，在 `x86_64` 中可以通过如下代码获知：

    ```rust
    CpuId::new().get_feature_info().map(
        |f| f.has_apic()
    ).unwrap_or(false)
    ```

- 操作 SPIV 寄存器，启用 APIC 并设置 Spurious IRQ Vector。

    查询文档可知，SPIV 寄存器的偏移量为 0xF0。其位描述如下：

    <table class="inst">
    <tr>
        <td class="inst-numnodel">31</td>
        <td class="inst-numnode" colspan="16"></td>
        <td class="inst-numnoder">10</td>
        <td class="inst-numnoder">9</td>
        <td class="inst-numnoder">8</td>
        <td class="inst-numnode" colspan="3"></td>
        <td class="inst-numnoder">4</td>
        <td class="inst-numnoder">3</td>
        <td class="inst-numnoder">2</td>
        <td class="inst-numnoder">1</td>
        <td class="inst-numnoder">0</td>
    </tr>
    <tr>
        <td colspan="18" class="inst-node-little"></td>
        <td colspan="1" class="inst-node-little">FC</td>
        <td colspan="1" class="inst-node-little">EN</td>
        <td colspan="4" class="inst-node-little">Vector</td>
        <td colspan="1" class="inst-node-little">1</td>
        <td colspan="1" class="inst-node-little">1</td>
        <td colspan="1" class="inst-node-little">1</td>
        <td colspan="1" class="inst-node-little">1</td>
    </tr>
    </table>

    因此，我们需要在保持其他位不变的情况下，将 EN bit 设置为 1，并将 Vector 设置为 `Irq::Spurious`，但是请注意实际设置的中断向量号需要加上 `Interrupts::IrqBase`。同时，此寄存器的 0-3 bit 无法被修改，始终为 1。

    最终代码如下：

    ```rust
    let mut spiv = self.read(0xF0);
    spiv |= 1 << 8; // set EN bit
    // clear and set Vector
    spiv &= !(0xFF);
    spiv |= Interrupts::IrqBase as u32 + Irq::Spurious as u32;
    self.write(0xF0, spiv);
    ```

- 设置 LVT 寄存器。

    Local Vector Table 寄存器用于设置中断向量号和触发模式。它们的位描述如下：

    <table class="inst">
    <tr>
        <td class="inst-numnode" colspan="4"></td>
        <td class="inst-numnodel">31</td>
        <td class="inst-numnode" colspan="8"></td>
        <td class="inst-numnoder">18</td>
        <td class="inst-numnoder">17</td>
        <td class="inst-numnoder">16</td>
        <td class="inst-numnoder">15</td>
        <td class="inst-numnoder">14</td>
        <td class="inst-numnoder">13</td>
        <td class="inst-numnoder">12</td>
        <td class="inst-numnoder">11</td>
        <td class="inst-numnode" colspan="2"></td>
        <td class="inst-numnoder">8</td>
        <td class="inst-numnode" colspan="3"></td>
        <td class="inst-numnoder">0</td>
    </tr>
    <tr>
        <td colspan="4">Timer</td>
        <td colspan="10" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">TP</td>
        <td colspan="1" class="inst-node-little">M</td>
        <td colspan="3" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">DS</td>
        <td colspan="4" class="inst-node-little">-</td>
        <td colspan="4" class="inst-node-little">Vector</td>
    </tr>
    <tr>
        <td colspan="4">LINT0</td>
        <td colspan="11" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">M</td>
        <td colspan="1" class="inst-node-little">TM</td>
        <td colspan="1" class="inst-node-little">RI</td>
        <td colspan="1" class="inst-node-little">IP</td>
        <td colspan="1" class="inst-node-little">DS</td>
        <td colspan="1" class="inst-node-little">-</td>
        <td colspan="3" class="inst-node-little">DMode</td>
        <td colspan="4" class="inst-node-little">Vector</td>
    </tr>
    <tr>
        <td colspan="4">LINT1</td>
        <td colspan="11" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">M</td>
        <td colspan="1" class="inst-node-little">TM</td>
        <td colspan="1" class="inst-node-little">RI</td>
        <td colspan="1" class="inst-node-little">IP</td>
        <td colspan="1" class="inst-node-little">DS</td>
        <td colspan="1" class="inst-node-little">-</td>
        <td colspan="3" class="inst-node-little">DMode</td>
        <td colspan="4" class="inst-node-little">Vector</td>
    </tr>
    <tr>
        <td colspan="4">ERROR</td>
        <td colspan="11" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">M</td>
        <td colspan="3" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">DS</td>
        <td colspan="4" class="inst-node-little">-</td>
        <td colspan="4" class="inst-node-little">Vector</td>
    </tr>
    <tr>
        <td colspan="4">PCINT</td>
        <td colspan="11" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">M</td>
        <td colspan="3" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">DS</td>
        <td colspan="1" class="inst-node-little">-</td>
        <td colspan="3" class="inst-node-little">DMode</td>
        <td colspan="4" class="inst-node-little">Vector</td>
    </tr>
    </table>

    - Vector 为中断向量号，当中断发生时，CPU 会跳转到中断向量表中对应处理程序执行。
    - DMode（Delivery Mode）为中断传递模式，本实验中不做理解要求。
    - DS（Delivery Status）为中断传递状态，只读。
    - M（Mask）为中断屏蔽位，取值为 1 表示中断已屏蔽。
    - TP（Timer Periodic Mode）为定时器周期模式，决定定时器周期触发还是仅触发一次。

    其余的位暂时不需要关注，如有兴趣可以参考 APIC 文档下的参考资料。

    以时钟中断的配置为例，需要将 Timer 的 Vector 设置为 `Irq::Timer`，并将 M 设置为 0，而后设置 TP 为 1，表示定时器周期模式。参考代码如下：

    ```rs
    let mut lvt_timer = self.read(0x320);
    // clear and set Vector
    lvt_timer &= !(0xFF);
    lvt_timer |= Interrupts::IrqBase as u32 + Irq::Timer as u32;
    lvt_timer &= !(1 << 16); // clear Mask
    lvt_timer |= 1 << 17; // set Timer Periodic Mode
    self.write(0x320, lvt_timer);
    ```

    若要禁用 LVT LINT0 则需要将 M 设置为 1，参考代码如下：

    ```rs
    self.write(0x350, 1 << 16); // set Mask
    ```

- 设置计时器相关寄存器。

    APIC 中控制计时器的寄存器包括 TDCR、TICR 和 LVT Timer。其中，TDCR 用于设置分频系数，TICR 用于设置初始计数值。

    - TDCR(0x3E0) 的分频系数决定了总线时钟与计时器时钟的比例，也即计时器的计数频率。
    - TICR(0x380) 的初始计数值决定了计时器的计数周期，每当计数到 0 时，就会触发中断。

    分频系数和 TDCR 寄存器的取值关系如下表所示，第二比特总是为 0：

    | 分频系数（Timer Divide） | 寄存器值 | 分频系数（Timer Divide） | 寄存器值 |
    | :----------------------: | :------: | :----------------------: | :------: |
    |           By 1           |  0b1011  |           By 2           |  0b0000  |
    |           By 4           |  0b0001  |           By 8           |  0b0010  |
    |          By 16           |  0b0011  |          By 32           |  0b1000  |
    |          By 64           |  0b1001  |          By 128          |  0b1010  |

    其参考的设置代码如下：

    ```rs
    self.write(0x3E0, 0b1011); // set Timer Divide to 1
    self.write(0x380, 0x20000); // set initial count to 0x20000
    ```

- 清除错误状态寄存器。

    APIC 中的错误状态寄存器（Error Status Register, 0x280）用于记录 APIC 内部的错误状态。当 APIC 发生错误时，CPU 会将错误信息写入此寄存器。为了避免错误状态寄存器中的错误信息影响后续的错误处理，我们需要在初始化 APIC 时清除错误状态寄存器中的错误信息。

    参考代码如下：

    ```rs
    self.write(0x280, 0);
    self.write(0x280, 0);
    ```

- 设置 ICR 寄存器。

    中断命令寄存器由两个 32 位寄存器组成，一个在 0x300，另一个在 0x310。它用于向不同的处理器发送中断。在写入 0x300 时发出中断，但在写入 0x310 时不发出中断。因此，要发送中断命令，应首先写入 0x310，然后写入 0x300。

    中断命令寄存器的位描述如下：

    <table class="inst">
    <tr>
        <td class="inst-numnode" colspan="3"></td>
        <td class="inst-numnodel">63</td>
        <td class="inst-numnode" colspan="3"></td>
        <td class="inst-numnoder">56</td>
        <td class="inst-numnode" colspan="17"></td>
        <td class="inst-numnoder">32</td>
    </tr>
    <tr>
        <td colspan="3" >0x310</td>
        <td colspan="5" class="inst-node-little">DF</td>
        <td colspan="18" class="inst-node-little">-</td>
    </tr>
    <tr>
        <td class="inst-numnode" colspan="3"></td>
        <td class="inst-numnodel">31</td>
        <td class="inst-numnode" colspan="4"></td>
        <td class="inst-numnoder">20</td>
        <td class="inst-numnode" colspan="1"></td>
        <td class="inst-numnoder">18</td>
        <td class="inst-numnode" colspan="1"></td>
        <td class="inst-numnoder">16</td>
        <td class="inst-numnoder">15</td>
        <td class="inst-numnoder">14</td>
        <td class="inst-numnoder">13</td>
        <td class="inst-numnoder">12</td>
        <td class="inst-numnoder">11</td>
        <td class="inst-numnode" colspan="2"></td>
        <td class="inst-numnoder">8</td>
        <td class="inst-numnode" colspan="4"></td>
        <td class="inst-numnoder">0</td>
    </tr>
    <tr>
        <td colspan="3" >0x300</td>
        <td colspan="6" class="inst-node-little">-</td>
        <td colspan="2" class="inst-node-little">DSH</td>
        <td colspan="2" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">TM</td>
        <td colspan="1" class="inst-node-little">LV</td>
        <td colspan="1" class="inst-node-little">-</td>
        <td colspan="1" class="inst-node-little">DS</td>
        <td colspan="1" class="inst-node-little">DM</td>
        <td colspan="3" class="inst-node-little">DMode</td>
        <td colspan="5" class="inst-node-little">Vector</td>
    </tr>
    </table>

    具体的配置配置细节这里不做理解要求，只需要按照如下描述进行配置即可：

    - DSH（Destination Shorthand）：设置为 2，始终将中断发送给所有 APIC
    - DMode（Delivery Mode）：设置为 5，INIT De-assert 模式
    - LV（Level）：设置为 0，INIT De-assert 模式
    - TM（Trigger Mode）：设置为 1，INIT De-assert 模式

    参考代码如下：

    ```rs
    self.write(0x310, 0); // set ICR 0x310
    const BCAST: u32 = 1 << 19;
    const INIT: u32 = 5 << 8;
    const TMLV: u32 = 1 << 15; // TM = 1, LV = 0
    self.write(0x300, BCAST | INIT | TMLV); // set ICR 0x300
    const DS: u32 = 1 << 12;
    while self.read(0x300) & DS != 0 {} // wait for delivery status
    ```

!!! warning "上述例子并不是完整的初始化代码，你需要参考 [APIC 可编程中断控制器](../../wiki/apic.md)，补全相关代码。"

为了提高代码的可读性，你可以尝试为不同的寄存器安排不同的**标识常量**，也可以**通过枚举的方式**标识这些寄存器，这可能需要修改 `read` 和 `write` 函数的参数类型。

## 时钟中断

在顺利配置好 XAPIC 并初始化后，APIC 的中断就被成功启用了。为了响应时钟中断，我们需要为 IRQ0 Timer 设置中断处理程序。创建 `src/interrupt/clock.rs` 文件，参考如下代码，为 Timer 设置中断处理程序：

```rust
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Timer as usize]
        .set_handler_fn(clock_handler);
}

pub extern "x86-interrupt" fn clock_handler(_sf: InterruptStackFrame) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if inc_counter() % 0x10000 == 0 {
            info!("Tick! @{}", read_counter());
        }
        super::ack();
    });
}

static COUNTER: /* FIXME */ = /* FIXME */;

#[inline]
pub fn read_counter() -> u64 {
    // FIXME: load counter value
}

#[inline]
pub fn inc_counter() -> u64 {
    // FIXME: read counter value and increase it
}
```

仅仅开启 APIC 的中断并不能触发中断处理，这是因为 CPU 的中断并没有被启用。在 `src/lib.rs` 中，所有组件初始化完毕后，需要为 CPU 开启中断，你可以通过如下代码实现：

```rust
x86_64::instructions::interrupts::enable();
```

它使用 `sti` 汇编指令，将 `RFLAGS` 寄存器中的 `IF` 位置 1，从而开启中断。

!!! question "实验任务"

    补全上述代码任务，并尝试修改你的代码，调节时钟中断的频率，并观察 QEMU 中的输出。

    说明你修改了哪些代码，如果想要中断的频率减半，应该如何修改？

## 串口输入中断

遵循 I/O 中断处理的 Top half & Bottom half 原则，在中断发生时，我们只在中断处理中做尽量少的事：读取串口的输入，并将其放入缓冲区。而在中断处理程序之外，我们可以在合适的时机，从缓冲区中读取数据，并进行处理。

为了开启串口设备的中断，你需要参考如下代码，在 `src/drivers/uart16550.rs` 的 `init` 函数末尾为串口设备开启中断：

```c
#define PORT 0x3f8          // COM1

static int init_serial() {
   outb(PORT + 1, 0x00);    // Disable all interrupts

   // ...

   outb(PORT + 1, 0x01);    // Enable interrupts
   return 0;
}
```

为了承接全部（可能的）用户输入数据，并将它们统一在标准输入，需要为输入准备缓冲区，并将其封装为一个驱动，创建 `src/drivers/input.rs` 文件，并借助 `crossbeam_queue` crate 实现一个输入缓冲区。

!!! tip "在 memory 初始化的过程中，我们已经有了内核堆分配的能力，可以动态分配内存。"

按照下列描述，补全 `src/drivers/input.rs` 驱动代码：

1. 使用 `crossbeam_queue::ArrayQueue` 存储用户输入的数据。

    借助 `once_mutex!` 和 `guard_access_fn!` 宏，构造一个上锁的全局静态变量 `INPUT_BUFFER`。

    此缓冲区大小和存储的数据类型由你自行决定，一个参考的缓冲区大小为 128。

2. 实现并暴露 `init` 函数。

    初始化 `INPUT_BUFFER`，完成后输出日志：`Input Initialized.` 并在在 `src/lib.rs` 中调用它，在操作系统启动时进行。

    请注意：`ysos_kernel::init` 函数中组件的初始化存在顺序，各种组件间可能存在**依赖关系**。由于输入缓冲区初始化是动态分配内存，因此需要在 `memory` 模块初始化之后，才能进行初始化。

3. 实现并暴露 `push_key` 函数。

    按照你所定义的类型，对 `INPUT_BUFFER` 上锁后，将数据放入缓冲区。若缓冲区已满，则丢弃数据，并使用 `warn!` 宏输出相关日志。

4. 实现并暴露 `try_pop_key` 函数。

    从缓冲区中**非阻塞**取出数据。若缓冲区为空或上锁失败，则返回 `None`。

    *Note: 或许需要在这一过程中暂时关闭中断。*

5. 实现并暴露 `pop_key` 函数。

    利用 `try_pop_key` 函数，从缓冲区中**阻塞**取出数据。循环等待，直到缓冲区中有数据。

6. 实现并暴露 `get_line` 函数。

    从缓冲区中**阻塞**取出数据，并将其实时打印出来。直到遇到换行符 `\n`。将数据转换为 `String` 类型，并返回。

    对于 `0x08` 和 `0x7F` 字符，表示退格，你需要对其进行特殊处理。若当前字符串不为空，则删除最后一个字符，并将其从屏幕上删除。

    删除操作可以通过发送 `0x08`、`0x20`、`0x08` 序列实现。你可以在串口驱动中将它封装为 `backspace` 函数。

    *Note: `String::with_capacity` 可以帮助你预先分配足够的内存。*

串口的输入中断与时钟中断类似，在 `src/interrupt/serial.rs` 中补全代码，为 IRQ4 Serial0 设置中断处理程序：

```rust
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Serial0 as usize]
        .set_handler_fn(serial_handler);
}

pub extern "x86-interrupt" fn serial_handler(_st: InterruptStackFrame) {
    receive();
    super::ack();
}

/// Receive character from uart 16550
/// Should be called on every interrupt
fn receive() {
    // FIXME: receive character from uart 16550, put it into INPUT_BUFFER
}
```

你需要补全 `receive` 函数，利用刚刚完成的 `input` 驱动，将接收到的字符放入缓冲区。

## 用户交互

在完善了输入缓冲区后，可以在 `src/main.rs` 中，使用 `get_line` 函数来获取用户输入的一行数据，并将其打印出来、或进行更多其他的处理，实现响应用户输入的操作。

为了避免时钟中断频繁地打印日志，你可以在 `clock_handler` 中，删除输出相关的代码，只保留计数器的增加操作。之后在 `get_line` 中，打印计数器的值，以便证明时钟中断的正确执行。

```rust
#![no_std]
#![no_main]

use ysos::*;
use ysos_kernel as ysos;

extern crate alloc;

boot::entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static boot::BootInfo) -> ! {
    ysos::init(boot_info);

    loop {
        print!("> ");
        let input = input::get_line();

        match input.trim() {
            "exit" => break,
            _ => {
                println!("You said: {}", input);
                println!("The counter value is {}", interrupt::clock::read_counter());
            }
        }
    }

    ysos::shutdown(boot_info);
}
```

## 思考题

1. 为什么需要在 `clock_handler` 中使用 `without_interrupts` 函数？如果不使用它，可能会发生什么情况？

2. 考虑时钟中断进行进程调度的场景，时钟中断的频率应该如何设置？太快或太慢的频率会带来什么问题？请分别回答。

3. 输入缓冲区在什么情况下会满？如果缓冲区满了，用户输入的数据会发生什么情况？

4. 进行下列尝试，并在报告中保留对应的触发方式及相关代码片段：

    - 尝试用你的方式触发 Triple Fault，开启 `intdbg` 对应的选项，在 QEMU 中查看调试信息，分析 Triple Fault 的发生过程。
    - 尝试触发 Double Fault，观察 Double Fault 的发生过程，尝试通过调试器定位 Double Fault 发生时使用的栈是否符合预期。
    - 通过访问非法地址触发 Page Fault，观察 Page Fault 的发生过程。分析 Cr2 寄存器的值，并尝试回答为什么 Page Fault 属于**可恢复的异常**。

5. 如果在 TSS 中为中断分配的栈空间不足，会发生什么情况？请分析 CPU 异常的发生过程，并尝试回答什么时候会发生 Triple Fault。

## 加分项

1. 😋 为**全部可能的 CPU 异常**设置对应的处理程序，使用 `panic!` 输出异常信息。

2. 😋 你如何定义用于计数的 `COUNTER`，它能够做到线程安全吗？如果不能，如何修改？

3. 🤔 操作 APIC 时存在大量比特操作，尝试结合使用 `bitflags` 和 `bit_field` 来定义和操作这些寄存器的值，从而获得更好的可读性。

4. 🤔 你的串口输入驱动是否能正确的处理中文甚至 emoji 输入？如何能够正确处理？
