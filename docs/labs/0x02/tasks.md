# 实验二：中断处理

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## GDT、TSS 与 IDT

## 注册中断处理程序

在 `src/interrupt/mod.rs` 中，参考如下代码，将中断描述符表（IDT）的注册工作委托给各个模块。IDT 只会被初始化一次，因此这里使用了 `lazy_static` 宏来实现懒加载，而不对它进行上锁或使用 Once 保护。

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

## 时钟中断

## 内存管理（此处不作重点）

## 串口输入中断

## 内核缓冲区

## 用户交互
