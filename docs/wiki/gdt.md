# GDT、TSS 与 IDT

## 中断描述表（IDT）

!!! note "阅读提示"

    基于 `x86_64` crate，在本实验中无需管理 IDT 相关的底层细节。

    以下内容作为参考资料，供大家深入理解。

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
<tr style="text-align: center;">
<td> <b>...</b> </td>
<td> <b>...</b>
</td></tr>
<tr>
<td> IDTR Offset + 4080 </td>
<td> Entry 255
</td></tr></table>

在实际应用中，计算机使用中断寄存器（IDTR）来指定 IDT 的起始地址和大小，从而进一步对中断处理程序寻址。IDTR（IDT Register）是一个 48（x86-64 下为 80） 位的寄存器，用于存储 IDT 的起始地址和大小。IDTR 的低 16 位存储 IDT 的大小，高 32 位存储 IDT 的起始地址。

<table class="wikitable">
<caption>IDT Descriptor (IDTR)
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
<caption>Gate Descriptor (64-bit)
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
<th style="width: 3.1%">47
</th>
<th style="width: 7%; text-align: left;">46&#160;&#160;&#160;<span style="float: right;">45</span>
</th>
<th style="width: 3.1%">44
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



### TSS

!!! note "阅读提示"
同样的，`x86_64` crate 已经提供了 TSS 的实现。但正确理解 TSS 有助于大家更好地实现中断处理过程，以下内容供大家深入学习。

Task State Segment（TSS）是 Intel x86 体系结构中的一个数据结构，用于存储与任务（任务切换）和中断处理相关的信息。TSS 包含了处理器在任务切换时需要保存和恢复的一些状态信息。每个任务都有一个相应的 TSS，通过任务寄存器（Task Register，TR）来引用。

下面是 TSS 的一些关键成员和功能：

1. **任务寄存器（TR）：** TR 寄存器用于存储 TSS 的选择子（Selector）。当进行任务切换时，通过加载不同的 TR 值，可以实现不同任务之间的切换。TR 的选择子包含了 TSS 在全局描述符表（Global Descriptor Table，GDT）中的位置。

2. **堆栈指针（Stack Pointers）：** TSS 包含了不同特权级别（Ring）下的堆栈指针（ESP 和 SS）。在任务切换时，当前任务的堆栈指针会被保存到 TSS，然后加载新任务的堆栈指针。

3. **段寄存器和选择子：** TSS 中包含了段寄存器（CS、DS、ES、FS、GS、SS）和相应的选择子。这些信息在任务切换时用于设置新任务的段寄存器。

4. **I/O 许可位图（I/O Permission Bitmap）：** TSS 可以包含一个 I/O 许可位图，用于限制任务对 I/O 端口的访问权限。这有助于实现任务间的隔离。

5. **任务状态标志（Task State Flag）：** TSS 中包含了一些标志位，其中最重要的是 NT 标志（Nested Task），用于指示任务是否支持嵌套任务。如果 NT 标志被设置，任务切换时会发生特殊处理。

6. **链接字段（Link Field）：** TSS 中包含了一个链接字段，用于构建任务链表。当任务切换发生时，当前任务的 TSS 会将其链接字段指向新任务的 TSS，形成一个任务链表。

7. **返回地址：** TSS 中包含了任务的返回地址。当任务切换时，当前任务的程序计数器（EIP）会被保存到 TSS，然后加载新任务的 EIP。

TSS 的主要作用是在任务切换时保存和恢复任务的上下文信息，确保任务切换的正确性。它在多任务操作系统中起到关键作用，允许系统在不同任务之间切换执行，实现多任务并发。因此，**在中断处理过程中，常用的中断栈由 TSS 描述，TSS 是保存上下文的具体数据结构。**
