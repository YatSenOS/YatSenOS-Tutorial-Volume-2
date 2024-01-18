# 分页内存简述

!!! tip "在 lab 1 中，由于面向 x64 的 UEFI 已经开启了分页机制，所以这里会将一部分概念提前介绍。"

## 页表

页表是操作系统中常用的内存模型，利用它可以实现虚拟内存、进行内存隔离、避免内存碎片等。作为操作系统内存管理的核心，在现代化的 OS 开发中，页表的参与是必不可少的。

在操作系统课程中，有关页表的内容往往被放置在后面的章节中，但是为了面向现代化的实验内容，这里需要大家对**页表是什么、能做什么、为什么需要**三个问题有一个初步的了解。

在计算机组成原理课程中，已经有过对于页表的介绍，它是一个**虚拟地址**到**物理地址**的映射表，通过它可以实现虚拟内存。

然而于这门课程所介绍的有所不同，在 x64 架构中，页表不再是一个简单连续的映射表，而是一个**树状结构**，这样的结构被称作**多级页表**。在本实验的实现中，会使用到**四级页表（PML4）**。

每一级的页表索引号都可以通过虚拟地址推导出来，对于一个 64 位的虚拟地址：

<table class="inst">
<tr>
    <td class="inst-numnodel">64</td>
    <td class="inst-numnode" colspan="16"></td>
    <td class="inst-numnoder">48</td>
    <td class="inst-numnode" colspan="8"></td>
    <td class="inst-numnoder">39</td>
    <td class="inst-numnode" colspan="8"></td>
    <td class="inst-numnodel">30</td>
    <td class="inst-numnode" colspan="8"></td>
    <td class="inst-numnodel">21</td>
    <td class="inst-numnode" colspan="8"></td>
    <td class="inst-numnodel">12</td>
    <td class="inst-numnode" colspan="12"></td>
    <td class="inst-numnodel">0</td>
</tr>
<tr>
    <td colspan="17" class="inst-node-little"></td>
    <td colspan="9" class="inst-node-little">L4 Index</td>
    <td colspan="9" class="inst-node-little">L3 Index</td>
    <td colspan="9" class="inst-node-little">L2 Index</td>
    <td colspan="9" class="inst-node-little">L1 Index</td>
    <td colspan="14" class="inst-node-little">Page Offset</td>
</tr>
</table>

为了降低学习难度和实现的负担，本实验中将直接使用 [x86_64 crate](https://docs.rs/x86_64) 中所提供的 `PageTable` 结构体和 `Cr3` 寄存器的封装。

具体的寻址过程可以参考 [地址转换范例](https://os.phil-opp.com/zh-CN/paging-introduction/#di-zhi-zhuan-huan-fan-li) 中的描述。

## 页表表项

页表中的每一个表项都是一个 64 位的数据，能够描述一个页表项的属性，及一些权限管理：

- Readable：是否可读，表示是否允许从这个页读取数据。
- Writable：是否可写，表示是否允许向这个页写入数据。
- Executable：是否可执行，表示是否允许执行这个页上的指令。

这三个主要权限一般被简写为 R/W/X，这部分内容将在处理 ELF 文件时用到。

它的具体结构可以参考 [页表格式](https://os.phil-opp.com/zh-CN/paging-introduction/#ye-biao-ge-shi) 中的描述。

## 虚拟内存

通过利用页表，操作系统可以实现虚拟内存，将一部分分配出的物理内存映射到虚拟地址空间中。

利用这种方式，操作系统可以限制用户进程的内存访问权限，实现内存隔离，同时也可以避免内存碎片的产生：对于用户程序所看到的虚拟地址空间，它是连续的，但是对于操作系统来说，它可以将这些虚拟地址映射到任意的物理地址上，因此可以随用随取，按需分配。

## x64 相关寄存器

在 x64 架构中，有一些特殊的寄存器与页表相关，且在本实验中会用到，它们分别是：

- CR0：控制寄存器，存储了一些控制系统运行状态的标志位，包括根页表的写保护位。
- CR2：页错误地址寄存器，存储了最近一次页错误的**虚拟地址**。
- CR3：页表根地址寄存器，存储了页表的**物理地址**，也就是 PML4 的地址。

## 参考资料

以下的资料提供了更加详细的页表介绍，作为这里简略介绍的补充，建议大家尝试进行阅读。

1. [Introduction to Paging](https://os.phil-opp.com/paging-introduction/)
2. [内存分页初探（上文翻译）](https://os.phil-opp.com/zh-CN/paging-introduction/#di-zhi-zhuan-huan-fan-li)
3. [Page Table - OSDev](https://wiki.osdev.org/Page_table)
4. [CPU Registers in x86 - OSDev](https://wiki.osdev.org/CPU_Registers_x86)
