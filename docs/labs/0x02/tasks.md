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
- `allocator.rs`：注册内核堆分配器，为内核堆分配提供能力。从而允许内核使用 `alloc` crate 进行需要动态内存分配的操作、使用其中定义的数据结构，如 `Vec`、`String`、`Box` 等。

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

!!! question "实验任务"

    补全上述代码任务，并进行下列尝试，并在报告中保留对应的触发方式及相关代码片段：

    1. 尝试用你的方式触发 Triple Fault，开启 `intdbg` 对应的选项，在 QEMU 中查看调试信息，分析 Triple Fault 的发生过程。
    2. 补全 Double Fault 的中断处理函数，观察 Double Fault 的发生过程。尝试通过调试器定位 Double Fault 发生时的栈是否符合预期。
    3. 补全 Page Fault 的中断处理函数，通过访问非法地址触发 Page Fault，观察 Page Fault 的发生过程。分析 Cr2 寄存器的值，并尝试回答为什么 Page Fault 属于可恢复的异常。

## 注册中断处理程序

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

可编程中断控制器（PIC）是构成 x86 架构的重要组成部分之一。得益于这一类芯片的存在，x86 架构得以实现中断驱动的操作系统设计。中断是一种处理外部事件的机制，允许计算机在运行过程中响应异步的、不可预测的事件。PIC的引入为处理中断提供了关键的硬件支持。

最初，x86 架构使用的是 8259 可编程中断控制器，它是一种级联的、基于中断请求线（IRQ）的硬件设备。随着计算机体系结构的发展和性能需求的提高，单一的 8259 PIC 逐渐显露出瓶颈，无法满足现代系统对更高级别中断处理的需求。

为了解决这个问题，高级可编程中断控制器（APIC）被引入到 x86 架构中。APIC 提供了更灵活的中断处理机制，支持更多的中断通道和更先进的中断处理功能。它采用了分布式的架构，允许多个处理器在系统中独立处理中断，从而提高了整个系统的并行性和性能。

!!! note "请阅读 [APIC 可编程中断控制器](../../wiki/apic.md) 部分，了解什么是 APIC 可编程中断控制器。"

根据上述文档，你需要在 `src/interrupt/apic/xapic.rs` 中补全 APIC 的初始化代码，以便在后续实验中使用 APIC 实现时钟中断和 I/O 设备中断。

## 时钟中断

在顺利配置好 XAPIC 并初始化后，APIC 的中断就被成功启用了。为了响应时钟中断，我们需要为 IRQ0 Timer 设置中断处理程序。创建 `src/interrupt/clock.rs` 文件，参考如下代码，为 Timer 设置中断处理程序：

```rust
use core::sync::atomic::{AtomicU64, Ordering};
use super::consts::*;

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[(Interrupts::IrqBase as u8 + Irq::Timer as u8) as usize]
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

static COUNTER: AtomicU64 = AtomicU64::new(0);

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

    补全上述代码任务，并进行下列尝试：

    1. 如何调节时钟中断的频率？请尝试修改你的代码，更改不同的频率，并观察 QEMU 中的输出。说明你修改了哪些代码，如果想要中断的频率减半，应该如何修改？
    2. 考虑时钟中断进行进程调度的场景，时钟中断的频率应该如何设置？太快或太慢的频率会带来什么问题？请分别回答。
    3. `without_interrupts` 函数的作用是什么？为什么需要使用它？它的实现原理是什么？

## 串口输入中断

## 内核缓冲区

## 用户交互
