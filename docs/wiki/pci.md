# PCI 总线

## 简介

PCI 全称是 **P**eripheral **C**omponent **I**nterconnect，是一种连接主板和外部硬件设备的总线标准，其改进版本 PCIe (PCI Express) 在现代计算机中广泛使用。

## BDF

Bus（总线号）、Device（设备号）和 Function（功能号）共同组成了所谓的 BDF 号，它就像是 PCI(e) 系统中的邮政编码。

!!! note "在 Linux 操作系统中，输入 `lspci` 命令，输出最左侧的数字就是 BDF 号"

PCI(e) 系统最多支持 **256** 条总线，Bus 0 是根总线，直接连接在 CPU 内部的 Host Bridge 上。外部的设备例如独立显卡、NVMe 固态硬盘等，通常会被分配到 Bus 1、Bus 2 等不同的总线号上。

每一条固定的总线上，理论上可以并排挂载最多 **32** 个物理芯片，每一个物理芯片代表一个 Device。

一个物理芯片内部，可能集成了多个互不干扰、功能完全独立的子核心，这就是所谓的多功能设备（Multi-Function Device）。其中 Function 0 是必须存在的核心主功能，单个设备最多支持 **8** 个功能，例如主板上的集成声卡和网卡可能做在同一个芯片上。这时，Function 0 负责以太网卡，Function 1 负责高保真音频，Function 2 负责蓝牙。在操作系统看来，它们就像是三个完全独立的硬件，有各自独立的配置空间和 BAR。

例如对于 BDF `01:1f.0`：

- `01` (Bus 1)： 位于 1 号总线上。

- `1f` (Device 31)： 该总线上的 31 号设备。

- `.0` (Function 0)： 开启的是该设备的主功能。

## PCI 配置空间

传统的 PCI 每个设备有 256 字节的配置空间，现代的 PCIe (PCI Express) 则扩展到了 4096 字节。[OSDev 文档](https://wiki.osdev.org/PCI#Configuration_Space) 中详细展示了不同 Header Type 的 PCI 配置空间的结构。其中我们介绍比较重要的 3 个部分：**设备标识**、**Header Type 寄存器**和**基地址寄存器**（Base Address Register, **BAR**）。

!!! warning "PCI 配置空间遵循**小端**存储顺序！"

### 设备标识

配置空间的前 4 个字节为 PCI 标识，包括低 16 比特的厂商标识和高 16 比特的设备标识。这是操作系统内核决定这个设备该使用什么驱动程序的主要依据。

### Header Type 寄存器

在配置空间偏移量 `0x0E` 处的一个 8 位（1 字节）寄存器被称为 Header Type（头部类型）寄存器，它告诉操作系统这个设备属于 PCI 标准限定的 3 个 Header Type 中的哪一个类型，并决定了接下来的配置空间布局。

Header Type 寄存器的最高位（Bit 7）是一个固定的标识位，如果为 0，说明这是一个单功能设备；如果这一位为 1，说明这是一个多功能设备，扫描时需要继续遍历从 1 到 7 的全部 Function。

### 基地址寄存器(BAR)

不同的 Header Type 所对应的设备，BAR 寄存器的位置与数量也不同，但寄存器的长度一定是 32 位。BAR 的最低位是一个固定的标识位，它的值代表：

- `1`: PMIO (Port-mapped I/O) 模式

- `0`: MMIO (Memory-mapped I/O) 模式

??? note "PMIO 和 MMIO"

    关于 PMIO 和 MMIO 可以在 [串口输出简介](./uart.md) 看到。PCI 规范在设计 BAR 的时候，特意用最低的比特位 Bit 0 作为标识位区分并兼容传统的 PMIO 和现代的 MMIO。

    在古老的 ISA 总线时代也就是 PCI 诞生之前，那时候的网卡或声卡使用 Port I/O，它们的端口地址是在出厂时就用硬件电路焊死的，或者通过板载的跳线开关手动拨码决定的。如果两个设备使用相同的端口地址就会产生冲突。PCI 总线为了解决这个痛点提出了即插即用（Plug and Play）：所有的外设在出厂时，绝对不能写死自己的 I/O 端口地址，必须由固件/操作系统动态分配。

## 访问配置空间

最简单直接的方式是传统的 CAM (Configuration Access Mechanism) 方式，通过 Host Bridge 在 I/O 空间里固定开放的两个 32 位的端口实现：

- (CONFIG_ADDRESS) 地址端口：`0xCF8` 

- (CONFIG_DATA) 数据端口：`0xCFC` 

详情请参考 [OSDev: Configuration Space Access Mechanism #1](https://wiki.osdev.org/PCI#Configuration_Space_Access_Mechanism_#1)。

上面的访问方式相当于通过 PMIO 访问，更现代但也更复杂的方式当然是利用 MMIO，这也就是 ECAM (Enhanced Configuration Access Mechanism)。这个方式下 Host Bridge 会在系统的物理内存地址空间中，强行划出一块很大的连续区域（通常是 256MB），专门映射给整个 PCIe 体系的配置空间。

??? note "Hypercall"

    在 pKVM 保护的虚拟机中，Guest OS 内核不再允许直接通过传统的硬件方式（无论是 IO 端口还是 MMIO）去访问真实的或物理模拟的 PCI 配置空间。类似 Syscall 是用户态程序向内核申请服务，Hypercall 是虚拟机内核向 Hypervisor（pKVM）申请服务。

    在 pKVM 环境下，Guest OS 把想访问的 BDF、偏移量、读取长度等参数塞进 CPU 寄存器，然后执行一条特殊的 CPU 汇编指令（Intel 上是 `vmcall`，AMD 上是 `vmmcall`），主动陷入到 pKVM Hypervisor 中，pKVM 读出数据，把结果放回寄存器，再恢复虚拟机运行。

## 分配 BAR 地址

本实验所使用的 UEFI 固件 [OVMF](../../wiki/uefi.md#ovmf) 已经帮我们完成了 BAR 地址的分配，如下简单讲解地址的分配过程。

固件会扫描总线也就是对 BDF 空间进行遍历。遍历过程中，寄存器位移可以设置为固定的值，例如设置为 `0x00` 来看是否能读出有效的设备标识。同时，由于上面提到过的 Header Type 寄存器最高位的存在，可以每次只遍历 Function 0，根据这一标识位决定是否继续遍历该设备的其它 Function。然后，对于有效的设备，读取 Header Type 寄存器确认其配置空间布局。

接下来修改配置空间偏移 `0x04` 的 Command 寄存器，关掉 I/O 和 Memory Decode。这一步是为了避免部分老旧硬件会把接下来需求探测写入 BAR 的 `0xFFFFFFFF` 误当成真正的物理地址去解析，导致系统死锁。

然后进行 BAR 需求探测：依次向该设备的各个 32 位 BAR 写入全 1（`0xFFFFFFFF`），然后读取 BAR。对于最低位 Bit 0 为 0 也就是 MMIO 类型，还需要再检查 Bits 2-1 所标识的 Type，若为 `0x2` 则代表该 BAR 和它的下一个 BAR 组成了一个 64 位 BAR， 需要探测下一个 BAR 作为高 32 位地址，计算时拼起来处理。

BAR 申请的空间大小就是读取出来的除了标识位和保留位之外没有被写为 `1` 的剩余低位组成的大小。这其实表明只有这些高位是可写的，剩余的低位、标识位和保留位都是不可写的，同时固件分配的地址低位必须全为 0，也就是相当于分配的地址必须是这个空间大小的倍数，这样分配好的地址才能完整地写入 BAR，并约定剩余不可写的低位均视为 0。

## 参考资料

- [OSDev: PCI](https://wiki.osdev.org/PCI)