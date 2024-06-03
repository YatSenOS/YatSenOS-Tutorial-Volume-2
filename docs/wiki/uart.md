# 串口输出简介

## 概述

处理器与外部设备进行通信有两种方式：并行通信与串行通信。与之对应的是两种硬件接口：并行接口与串行接口。并行接口通常用于高速数据传输，例如连接打印机、显示器、硬盘等设备。由于替代技术的普及和串口协议的迭代，并口其在计算机领域的使用程度已经大幅下降。

串口（Serial Port）是一种常见的计算机接口，用于在计算机和外部设备之间进行**串行**数据传输。串口是一种通用的调试接口，几乎所有计算机和嵌入式设备都提供了串口接口。这使得串口成为一种广泛支持的调试方法，可以在各种硬件平台和操作系统上使用。

串口提供了**低级别的硬件访问能力**，可以**直接**与设备进行通信。因此，串口通常用于**低级别的系统调试和硬件调试**，例如在操作系统启动之前或操作系统不可用的情况下进行调试。

基于实用性和实现的简便考虑，实验将采用串口作为操作系统的输入输出接口，从而避免对输出图像相关驱动、渲染相关问题的考虑。通过将操作系统的输出重定向到串口和终端程序，让它们完成输出信息的显示渲染工作。

!!! note "串口与屏幕显示"

    串口输出与常见的屏幕显示不同，这是两套**独立**的输出逻辑，请大家注意甄别。

## UART 与串口

在 x86 系统上，串口（Serial Port）通常使用 RS-232 协议，通过 UART 实现串行数据的发送和接收。UART 是在串行交互界面上负责对数据完成编解码硬件芯片。相较于其他串行数据交互协议，UART 的特点如下：

1. 异步通信：UART 采用异步通信方式，发送方和接收方之间没有时钟信号同步，而是通过数据帧中的起始位和停止位来同步数据。

2. 全双工通信：UART 采用全双工通信方式，发送方和接收方可以同时发送和接收数据。

!!! note "关于 UART 细节"

    经讨论，TA 们认为 UART 硬件细节不是实验的重点，因此不要求同学们掌握 UART 细节，只需了解 UART 16550 接口的调用方法即可。

    如果你对 UART 协议的细节感兴趣，可以参考 [Serial Ports - OSDev](https://wiki.osdev.org/Serial_Ports) 以及 [UART串口知识整理 - 知乎](https://zhuanlan.zhihu.com/p/467003598)。

## UART 16550

UART 16550 是一种集成电路芯片，用于串口通信。它是最常见和广泛使用的 UART 芯片之一，具有高可靠性和兼容性。UART 16550 芯片提供了一个标准的串行接口，可用于将计算机与外部设备进行数据传输。

在 x86_64 体系结构中，UART 16550 常用于完成与串口设备的通信，对应的接口也被命名为 COM 接口。COM 接口会被映射到标准的 I/O 端口，可以通过**读写 I/O 端口**来完成与串口通信。

COM 端口和 I/O 端口的映射关系你可以在 [Port Addresses](https://wiki.osdev.org/Serial_Ports#Port_Addresses) 中找到。

在本实验设计中，只需要关心 COM1 端口即可，它的 I/O 端口地址为 `0x3F8`。

## x86 I/O 端口

!!! note "关于 I/O 的访问方式"

    CPU 与计算机外部 I/O 设备的常见交互模式分为 Memory-mapped I/O (MMIO) 和 Port-mapped I/O 两种，这两种方式也被称为统一编址和独立编址。

    - Memory-mapped I/O (MMIO) 即通过将需要进行交互的 I/O 设备的相关寄存器映射到某一段内存地址空间，从而实现对 I/O 设备的访问。在启用虚拟内存机制的系统中，这些内存空间**同样需要通过虚拟地址进行访问**。

    - Port-mapped I/O 即将 I/O 设备的相关寄存器编址在相对与内存地址独立的地址空间，并使用专门的指令与 I/O 设备进行交互。在 x86 系统中，I/O 端口的地址空间为 0x0000 - 0xFFFF，可以通过 `in` 和 `out` 指令进行访问。

    由于历史遗留原因，x86 架构中同时存在 MMIO 和 port-mapped I/O 两种访问方式，不过由于 Port I/O 地址空间太小和其他的一些需求，现代硬件越来越偏向于使用 MMIO。

    有关串口设备驱动实现的更多信息，请参考 [Serial Ports - OSDev](https://wiki.osdev.org/Serial_Ports)。

在 x86 体系中，I/O 端口的读写是通过 `in` 和 `out` 指令完成的。`in` 指令用于从 I/O 端口读取数据，`out` 指令用于向 I/O 端口写入数据。

在使用 [x86_64 crate](https://docs.rs/x86_64) 时，可以通过 `x86_64::instructions::port` 模块中的 [`PortGeneric`](https://docs.rs/x86_64/latest/x86_64/instructions/port/struct.PortGeneric.html) 结构体来完成对 I/O 端口的读写。

!!! question "x86_64 crate 中是怎么封装 I/O 端口的？你能否通过查看源码找到答案？"

## 参考资料

1. [UART 16550 - docs.rs](https://docs.rs/uart_16550)
2. [Serial Ports - OSDev](https://wiki.osdev.org/Serial_Ports)
3. [UART 16550 Tutorial - byterunner](http://byterunner.com/16550.html)
4. [UART 16550 Tutorial](http://www.larvierinehart.com/serial/serialadc/serial.htm)
5. [UART 串口知识整理 - 知乎](https://zhuanlan.zhihu.com/p/467003598)
