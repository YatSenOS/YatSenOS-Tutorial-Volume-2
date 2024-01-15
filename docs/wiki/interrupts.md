# CPU 中断处理

## 概述

在计算机系统中，中断是一种机制，用于打破 CPU 的正常执行流程，以响应特定事件或条件。中断是操作系统与硬件之间进行通信的重要手段。它允许外部设备或系统事件通知 CPU 有关状态的变化。通过中断，操作系统能够实时响应外部事件，例如设备的输入输出、时钟中断等。在 x86-64 架构下，中断主要分为硬件中断和软件中断两类：

1. 硬件中断：硬件中断是由外部设备或硬件引发的中断。这包括了各种设备，如键盘、鼠标、网络接口卡等。硬件中断通常通过中断控制器向 CPU 发送中断信号。

2. 软件中断：软件中断是由程序执行中的特殊指令触发的中断。在 x86-64 架构中，软件中断通过 `int` 指令实现。软件中断通常用于系统调用，允许用户空间程序请求内核提供的服务。

!!! note "中断（Interrupt）和异常（Exception）的区别"

    中断和异常都是打破 CPU 正常执行流程的机制。它们的区别在于，中断是由外部设备或事件引发的，而异常是由程序执行中的错误或非法操作引发的。

总的来说，中断的作用可以总结为以下几点：

1. 异常处理：中断还用于处理系统中的异常情况，例如除零错误、内存访问违规等。通过中断，操作系统可以捕获这些异常并采取适当的措施，确保系统的稳定性。

2. 节省 CPU 资源：使用中断可以避免 CPU 不断轮询外部设备的状态。当设备有数据准备好时，它会触发中断，使得 CPU 可以专注于其他任务，提高系统效率。

3. 实时响应事件：中断允许计算机系统实时响应外部设备或事件的状态变化。例如，当用户按下键盘上的某个键时，键盘控制器可以发出一个中断信号，通知 CPU 有输入事件发生。


## 中断的基本组件

### 中断描述表（IDT）

!!! note "阅读提示"
      基于 `x86_64` crate，在本实验中无需管理 IDT 相关的底层细节。以下内容作为参考资料，供大家深入理解。

在 x86-64 架构中，中断描述表（IDT）是用于存储中断门描述符的数据结构。IDT 是一个数组，每个元素对应一个中断门。

<table class="wikitable">
<caption>Interrupt Descriptor Table (64-bit)
</caption>
<tr>
<th> Address </th>
<th> Content
</th></tr>
<tr>
<td> IDTR Offset + 0 </td>
<td> Entry 0
</td></tr>
<tr>
<td> IDTR Offset + 16 </td>
<td> Entry 1
</td></tr>
<tr>
<td> IDTR Offset + 32 </td>
<td> Entry 2
</td></tr>
<tr style="text-align: center;">
<td> <b>...</b> </td>
<td> <b>...</b>
</td></tr>
<tr>
<td> IDTR Offset + 4080 </td>
<td> Entry 255
</td></tr></table>

在实际应用中，计算机使用中断寄存器（IDTR）来指定 IDT 的起始地址和大小，从而进一步对中断处理程序寻址。IDTR（IDT Register）是一个 48（x86-64下为80） 位的寄存器，用于存储 IDT 的起始地址和大小。IDTR 的低 16 位存储 IDT 的大小，高 32 位存储 IDT 的起始地址。

<table class="wikitable">
<caption>IDT Descriptor (IDTR):
</caption>
<tr>
<th style="width: 66%; text-align: left;">79 (64-bit Mode)<br />48 (32-bit Mode)&#160;&#160;&#160;<span style="float: right;">16</span>
</th>
<th style="width: 34%; text-align: left; vertical-align: bottom;">15&#160;&#160;&#160;<span style="float: right;">0</span>
</th></tr>
<tr>
<td><b>Offset</b><br />63 (64-bit Mode)<br />31 (32-bit Mode)&#160;&#160;&#160;<span style="float: right;">0</span>
</td>
<td><b>Size</b><br /><br />15&#160;&#160;&#160;<span style="float: right;">0</span>
</td></tr></table>

使用 IDT 时，需要为每个可能的中断分配一个唯一的中断门描述符。中断门描述符中的地址字段指向相应中断处理程序的入口地址；中断门其余字段包含了一些必要的标志位。

<table class="wikitable">
<caption>Gate Descriptor (64-bit):
</caption>
<tr>
<th colspan="7" style="text-align: left;">127&#160;&#160;&#160;<span style="float: right;">96</span>
</th></tr>
<tr>
<td colspan="7">Reserved
</td></tr>
<tr>
<th colspan="7" style="text-align: left;">95&#160;&#160;&#160;<span style="float: right;">64</span>
</th></tr>
<tr>
<td colspan="7"><b>Offset</b><br />63&#160;&#160;&#160;<span style="float: right;">32</span>
</td></tr>
<tr>
<th style="width: 50%; text-align: left;">63&#160;&#160;&#160;<span style="float: right;">48</span>
</th>
<th style="width: 3.125%">47
</th>
<th style="width: 6.25%; text-align: left;">46&#160;&#160;&#160;<span style="float: right;">45</span>
</th>
<th style="width: 3.125%">44
</th>
<th style="width: 12.5%; text-align: left;">43&#160;&#160;&#160;<span style="float: right;">40</span>
</th>
<th style="width: 15.625%; text-align: left;">39&#160;&#160;&#160;<span style="float: right;">35</span>
</th>
<th style="width: 9.375%; text-align: left;">34&#160;&#160;&#160;<span style="float: right;">32</span>
</th></tr>
<tr>
<td><b>Offset</b><br />31&#160;&#160;&#160;<span style="float: right;">16</span>
</td>
<td style="text-align: center; vertical-align: top;"><b>P</b>
</td>
<td><b>DPL</b><br />1&#160;&#160;&#160;<span style="float: right;">0</span>
</td>
<td style="text-align: center; vertical-align: top;">0
</td>
<td><b>Gate Type</b><br />3&#160;&#160;&#160;<span style="float: right;">0</span>
</td>
<td style="vertical-align:top">Reserved
</td>
<td><b>IST</b><br />2&#160;&#160;&#160;<span style="float: right;">0</span>
</td></tr>
<tr>
<th style="text-align: left;">31&#160;&#160;&#160;<span style="float: right;">16</span>
</th>
<th style="text-align: left;" colspan="6">15&#160;&#160;&#160;<span style="float: right;">0</span>
</th></tr>
<tr>
<td><b>Segment Selector</b><br />15&#160;&#160;&#160;<span style="float: right;">0</span>
</td>
<td colspan="6"><b>Offset</b><br />15&#160;&#160;&#160;<span style="float: right;">0</span>
</td></tr></table>

### 中断服务例程

中断服务例程（Interrupt Service Routine，ISR）是中断处理过程的核心部分，负责实际处理中断事件。由前介绍，不同的中断处理程序对应不同的中断表项，处理不同的中断情况。基于中断响应过程提供的系统上下文，中断服务例程包含了具体的处理逻辑，可能涉及与硬件设备的通信、数据处理、状态更新等操作。

值得注意的是，中断服务例程可能包含一些临界区（Critical Section，，一段在执行期间不能被其他中断打断的代码）。为了确保系统的正确性和稳定性，中断服务例程通常需要使用同步机制，如**禁用中断**（即所谓关中断）或使用互斥锁来保护临界区。

!!! note "实现提示"
      中断处理例程是本次实验的主要代码实现部分，需要大家仔细阅读并理解。

### TSS

!!! note "阅读提示"
      同样的，`x86_64` crate 已经提供了 TSS 的实现。但正确理解 TSS 有助于大家更好地实现中断处理过程，以下内容供大家深入学习。

Task State Segment（TSS）是 Intel x86 体系结构中的一个数据结构，用于存储与任务（任务切换）和中断处理相关的信息。TSS 包含了处理器在任务切换时需要保存和恢复的一些状态信息。每个任务都有一个相应的TSS，通过任务寄存器（Task Register，TR）来引用。

下面是TSS的一些关键成员和功能：

1. **任务寄存器（TR）：** TR 寄存器用于存储 TSS 的选择子（Selector）。当进行任务切换时，通过加载不同的 TR 值，可以实现不同任务之间的切换。TR 的选择子包含了 TSS 在全局描述符表（Global Descriptor Table，GDT）中的位置。

2. **堆栈指针（Stack Pointers）：** TSS包含了不同特权级别（Ring）下的堆栈指针（ESP 和 SS）。在任务切换时，当前任务的堆栈指针会被保存到 TSS，然后加载新任务的堆栈指针。

3. **段寄存器和选择子：** TSS 中包含了段寄存器（CS、DS、ES、FS、GS、SS）和相应的选择子。这些信息在任务切换时用于设置新任务的段寄存器。

4. **I/O许可位图（I/O Permission Bitmap）：** TSS 可以包含一个 I/O 许可位图，用于限制任务对 I/O 端口的访问权限。这有助于实现任务间的隔离。

5. **任务状态标志（Task State Flag）：** TSS 中包含了一些标志位，其中最重要的是 NT 标志（Nested Task），用于指示任务是否支持嵌套任务。如果NT标志被设置，任务切换时会发生特殊处理。

6. **链接字段（Link Field）：** TSS 中包含了一个链接字段，用于构建任务链表。当任务切换发生时，当前任务的 TSS 会将其链接字段指向新任务的TSS，形成一个任务链表。

7. **返回地址：** TSS 中包含了任务的返回地址。当任务切换时，当前任务的程序计数器（EIP）会被保存到 TSS，然后加载新任务的 EIP。

TSS 的主要作用是在任务切换时保存和恢复任务的上下文信息，确保任务切换的正确性。它在多任务操作系统中起到关键作用，允许系统在不同任务之间切换执行，实现多任务并发。因此，**在中断处理过程中，常用的中断栈由 TSS 描述，TSS 是保存上下文的具体数据结构。**

## 中断的处理过程

基于以上介绍，我们可以总结出中断的处理过程：

1. **触发中断**

   中断可以由硬件（硬件中断）或软件（软件中断）引发。硬件中断通常由外部设备发出，而软件中断由执行 `int` 指令触发。

2. **保存当前状态（上下文）**

   在处理中断之前，CPU 会保存当前的执行状态，包括寄存器内容、标志寄存器等。在处理 trap 时，有可能会改变系统的状态。所以在真正处理 trap 之前，我们有必要对系统的当前状态进行保存，在处理完成之后，我们再将系统恢复至原先的状态，就可以确保之前的程序继续正常运行。 这里的系统状态通常是指寄存器，这些寄存器也叫做CPU的上下文 ( Context )。

3. **查找中断描述符**

   CPU 根据中断向量号查找 IDT 中对应的中断门描述符。

4. **执行中断处理程序**

   CPU 跳转到中断门描述符指定的中断处理程序的入口地址，开始执行中断处理代码。

5. **中断处理程序执行**

   中断处理程序负责处理特定中断类型。它可能包括设备输入输出、异常处理、系统调用等。

6. **恢复状态**

   中断处理程序执行完毕后，CPU 恢复之前保存的状态，继续执行被中断的程序。

## 常见的中断

以下是一些常见的中断类型：

1. **时钟中断（Timer Interrupt）：** 由系统定时器触发，用于维护系统时间，进行任务调度和时间片轮转。

2. **硬件设备中断（Hardware Interrupt）：** 由硬件设备触发，例如键盘、鼠标、磁盘等设备产生的中断，通知系统有新的数据或事件需要处理。

3. **系统调用中断（System Call Interrupt）：** 被用户程序请求操作系统服务时触发的中断，通过软中断或特定指令（如 `syscall` ）实现。

4. **软件异常中断（Software Exception Interrupt）：** 一些软件生成的异常，如调试中断、断点中断等，用于调试和程序跟踪。

5. **NMI中断（Non-Maskable Interrupt）：** 一种高优先级的中断，通常用于处理严重系统错误，无法被屏蔽。

6. **IRQ中断（Interrupt Request）：** 用于连接外部硬件设备的中断，包括可编程中断控制器（PIC）或高级可编程中断控制器（APIC）的 IRQ 线。

## 结论

在 x86-64 架构下，中断是操作系统与硬件之间进行通信的关键机制。通过合理设计和配置中断描述表（IDT），系统能够高效地响应各类中断事件。了解中断的作用、中断的分类以及中断的处理过程，有助于理解计算机系统中的事件处理机制，为系统设计和调试提供基础支持。

## 参考资料

1. https://wiki.osdev.org/IDT
2. https://wiki.osdev.org/GDT
3. https://wiki.osdev.org/TSS