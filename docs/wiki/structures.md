# x64 数据结构概述

首先需要明确的是，GDT、TSS、IDT 及页表等概念均为 x86_64 架构下约定的**数据结构**，他们用于存储有关于 CPU 运作的相关信息，根据存储的内容来进行地址解析、权限管理、中断处理等操作。

它们可以被存储在内存的任何位置，但需要利用特定的寄存器来指定其位置，当 CPU 需要访问这些数据结构时，会根据寄存器中的地址来访问。

!!! note "阅读提示"

    `x86_64` crate 已经提供了相关数据结构的实现：[structures mod](https://docs.rs/x86_64/latest/x86_64/structures/index.html)。

    为了减轻负担，实验将使用 `x86_64` crate 提供的数据结构，而不是自己造轮子。不过，你仍然需要了解这些数据结构的作用和使用方法。

## 全局描述符表

全局描述符表（Global Descriptor Table, GDT）是 x86 架构下的一种数据结构，用于存储段描述符、定义和管理内存段的访问权限和属性。它是操作系统内核与硬件之间的桥梁，允许操作系统精确地控制不同段的访问权限。

在整个系统中，全局描述符表 GDT 只有一张（一个处理器对应一个 GDT），GDT 可以被放在内存的任何位置，但 CPU 必须知道 GDT 的入口，也就是基地址放在哪里，Intel 的设计者门提供了一个寄存器 GDTR 用来存放 GDT 的入口地址，程序员将 GDT 设定在内存中某个位置之后，可以通过 LGDT 指令将 GDT 的入口地址装入此寄存器，从此以后，CPU 就根据此寄存器中的内容作为 GDT 的入口来访问 GDT 了。

在 32 位保护模式下，GDT 中的每个条目都是一个段描述符（Segment Descriptor）。段描述符包含了段的起始地址、段的大小、段的访问权限等信息。

在 64 位长模式开启分页机制下，段描述符的作用被大大削弱，地址转换和访问权限由分页机制来控制，但它仍然被用于记录一些系统状态信息、进行特权级管理等。

## 任务状态段

任务状态段（Task State Segment, TSS）也是一种数据结构，它存储有关任务的相关数据。

在 32 位保护模式下，它主要用于存储与任务（任务切换）和中断处理相关的信息。TSS 包含了处理器在任务切换时需要保存和恢复的一些状态信息。每个任务都有一个相应的 TSS，通过任务寄存器（Task Register，TR）来引用。

在 64 位长模式下，TSS 的结构与 32 位不同，它并不直接与任务切换挂钩，但是它仍然被用于存储当特权级栈和中断栈：

- 特权级栈：最多三个，当权限级别从较低权限级别更改为较高权限级别时使用的栈指针。
- 中断栈：最多七个，当中断发生时，如果定义了对应的中断栈，则使用对应栈指针。它可以用于存储不同**大小和用途**的中断栈，以处理不同的中断。
- I/O 映射基址：包含从 TSS 底部到 I/O 权限位图的 16 位偏移量，在我们的实验中不会使用到。

## 中断描述符表

中断描述符表（Interrupt Descriptor Table, IDT）是用于存储中断门描述符的数据结构。

<table class="wikitable"><caption>Interrupt Descriptor Table (64-bit)</caption><tr><th> Address </th><th> Content</th></tr><tr><td> IDTR Offset + 0 </td><td> Entry 0</td></tr><tr><td> IDTR Offset + 16 </td><td> Entry 1</td></tr><tr style="text-align: center;"><td> <b>...</b> </td><td> <b>...</b></td></tr><tr><td> IDTR Offset + 4080 </td><td> Entry 255</td></tr></table>

使用 IDT 时，需要为每个可能的中断分配一个唯一的中断门描述符，对于 x86_64 架构，前 32 个中断号（0-31）被 Intel 保留，用于处理 CPU 异常，之后的描述符是用户自定义的中断处理程序，可以被操作系统自定义。

中断门描述符中的地址字段指向相应中断处理程序的入口地址、中断栈等信息。在中断发生时，中断上下文的信息将会被保留在中断栈中，并将处理程序的地址放置到 RIP 寄存器中来进行调用。这一过程中遵守了 x86_64 的相关调用约定，因此中断处理程序也需要遵守相关的 ABI 约定，在 x86_64 中，应当通过 `iretq` 指令来结束中断调用。

中断门其余字段包含了一些必要的标志位，描述符存储的信息如下所示：

<table class="wikitable"><caption>Gate Descriptor (64-bit)</caption><tr><th colspan="7" style="text-align: left;">127&#160;&#160;&#160;<span style="float: right;">96</span></th></tr><tr><td colspan="7">Reserved</td></tr><tr><th colspan="7" style="text-align: left;">95&#160;&#160;&#160;<span style="float: right;">64</span></th></tr><tr><td colspan="7"><b>Offset</b><br />63&#160;&#160;&#160;<span style="float: right;">32</span></td></tr><tr><th style="width: 50%; text-align: left;">63&#160;&#160;&#160;<span style="float: right;">48</span></th><th style="width: 3.1%">47</th><th style="width: 7%; text-align: left;">46&#160;&#160;&#160;<span style="float: right;">45</span></th><th style="width: 3.1%">44</th><th style="width: 12.5%; text-align: left;">43&#160;&#160;&#160;<span style="float: right;">40</span></th><th style="width: 15.625%; text-align: left;">39&#160;&#160;&#160;<span style="float: right;">35</span></th><th style="width: 9.375%; text-align: left;">34&#160;&#160;&#160;<span style="float: right;">32</span></th></tr><tr><td><b>Offset</b><br />31&#160;&#160;&#160;<span style="float: right;">16</span></td><td style="text-align: center; vertical-align: top;"><b>P</b></td><td><b>DPL</b><br />1&#160;&#160;&#160;<span style="float: right;">0</span></td><td style="text-align: center; vertical-align: top;">0</td><td><b>Gate Type</b><br />3&#160;&#160;&#160;<span style="float: right;">0</span></td><td style="vertical-align:top">Reserved</td><td><b>IST</b><br />2&#160;&#160;&#160;<span style="float: right;">0</span></td></tr><tr><th style="text-align: left;">31&#160;&#160;&#160;<span style="float: right;">16</span></th><th style="text-align: left;" colspan="6">15&#160;&#160;&#160;<span style="float: right;">0</span></th></tr><tr><td><b>Segment Selector</b><br />15&#160;&#160;&#160;<span style="float: right;">0</span></td><td colspan="6"><b>Offset</b><br />15&#160;&#160;&#160;<span style="float: right;">0</span></td></tr></table>

可以看到，它存储了权限级别（DPL）、中断处理程序的入口地址（Offset）、中断栈（IST）等信息。

门类型（Gate Type）字段指定了中断门的类型，包括：

- 中断门（Interrupt Gate）：用于处理中断，中断处理程序会被调用，中断返回后，CPU 会恢复中断前的执行状态。
- 陷阱门（Trap Gate）：用于处理陷阱，陷阱处理程序会被调用，不同之处在于，对于中断门，中断在进入时自动禁用，并在 IRET 时重新启用，而陷阱门则不会主动禁用中断。
- 任务门（Task Gate）：曾用于任务切换，它在 x86-64 上已被完全删除。

## 页表

有关页表等内容繁杂，详见 [分页内存简述](./paging.md) 的内容。

## 参考资料

1. [GDT - OSDev](https://wiki.osdev.org/GDT)
2. [TSS - OSDev](https://wiki.osdev.org/TSS)
3. [IDT - OSDev](https://wiki.osdev.org/IDT)
4. [What is the modern usage of the global descriptor table(GTD)?](https://stackoverflow.com/questions/64741681/what-is-the-modern-usage-of-the-global-descriptor-tablegtd)
