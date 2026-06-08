# 实验八：扩展实验

!!! tip "海阔凭鱼跃，天高任鸟飞。<br/>&nbsp;<span style="font-weight: bold; float: right">——《古今贤文》</span>"

## 实验任务与要求

首先，祝贺你已经顺利完成了前面的实验，你已经拥有了一个基本的操作系统内核，它可以在 QEMU 中运行，并且可以通过串口输出进行交互。

它有自己的进程管理机制，可以从磁盘中加载用户程序进行执行；它有自己的同步互斥机制，可以让多个线程安全地访问共享资源；它有自己的内存管理机制，可以在内核态和用户态之间进行内存的分配和释放。

在这个实验中，你需要在这个基础上，**自主实现一些更加有趣的功能**，例如图形输出、文件系统、内存分配、块设备缓存层等。

请从下列任务中，**选择一到两个目标进行实现**：

---

### VGA 显示输出

串口输出只是最基本的交互方式，尝试在 bootloader 中，利用 `get_handle_for_protocol::<GraphicsOutput>` 获取到 VGA 显示的模式、分辨率、缓冲区地址等信息。

之后通过利用 `embedded-graphics` 等库，实现一个简单的图形输出驱动，赋予操作系统绘制图形的能力，之后实现一个简单的 Shell 显示，并激活键盘输入的驱动，通过和串口启动共享缓冲区的方式，响应键盘输入。

#### 实现目标

1. 在 bootloader 中，获取图形输出的相关信息，并传递给内核。
2. 实现一个简单的图形输出驱动，支持基本的 VGA 图形绘制操作。
3. 利用 `embedded-graphics` 实现字符渲染。
4. 为 VGA 显示实现一个 Shell 输出，使其能够输出字符、清屏等，并将其对接到合适的日志输出。

#### 加分项

1. 利用键盘输入中断，为 QEMU 的 GUI 界面实现输入输出。
2. 尝试将 Shell 部分限制在屏幕的下半部分，并在上半部分实现一些图像的绘制。作为一个例子，你可以利用 `sleep` 等方式，实现一个不断绘制转动的钟表的进程，并让它作为一个后台进程运行。

---

### 可读写的临时文件系统

根据现有的其他 OS 实验中的文件系统设计、或参考现有的文件系统设计，结合你学过的算法、组织数据的方式，实现一个简单的文件系统的驱动。

可以是目录结构文件系统（例如参考 FAT32），可以是日志文件系统（例如参考 ext4、NTFS）。实现之后利用帧分配器在操作系统初始化分配一段内存空间，将其作为块设备，使用你的文件系统对其进行格式化、挂载、读写等操作。为了更好的实现你可能需要适当增大启动 OS 的内存量。

#### 实现目标

1. 将启动目录挂载至 `/boot` 目录下。
2. 将临时文件系统挂载至 `/tmp` 目录下。
3. 创建 `/tmp/mydir` 目录。
4. 创建 `/tmp/mydir/hello.txt` 文件。
5. 向 `/tmp/mydir/hello.txt` 文件写入一段字符序列，包含你的学号。
6. 读取 `/tmp/mydir/hello.txt` 文件，将其中的字符序列打印到屏幕上。

#### 加分项

1. 尝试实现文件系统的硬链接，并测试读写操作。
2. 修改 QEMU 参数，挂载一块虚拟磁盘，尝试实现文件的持久化。

---

### 内存管理算法

在上次实验实现了堆内存的分配和释放后，用户态有了自己的能力去管理自己的页面，但是其实际的动态内存分配还是依赖于内核提供的服务。

在这个实验目标中，你可以尝试实现一些内存管理算法，例如 Buddy 算法、Slab 算法等，将其作为内存分配器，为用户态提供更加灵活的内存分配服务。

不过，在这一目标中，你大概率需要一个链表，你可以找一些现成的实现，或者选择完全的自己实现。

#### 实现目标

1. 在 `crates` 下添加一个独立的新 `package`，用于实现能够测试的内存分配器。
2. 参考 `storage` 包进行测试配置，并配置好 `no_std` 等属性。
3. 在其中实现你的内存管理算法，参考 `LockedHeap` 的内存初始化方式，管理一段内存。
4. 在 `lib` 中为你的内存分配器声明一个 `feature`，并将它声明为 `#[global_allocator]`。
5. 编写测试程序，并测试你的内存分配器能否正确分配内存。

#### 加分项

1. 成功利用 `brk` 和你自己实现的内存分配器，实现用户态的内存管理。
2. 尝试让你的内存分配器支持调用 `brk` 进行扩容。

!!! tip "将你的内存分配器作为 `static` 变量在线程间共享使用有助于避免线程导致的内存错误。"

---

### 块设备的缓存层

在 `storage` 包中，基于目前已有的实现，实现一个块设备的缓存层。

这里给出一个缓存设备结构体的设计参考：

```rust
pub struct CachedDevice<B, C>
where
    B: BlockTrait,
    C: CacheManager<B>,
{
    cache: C,
    device: Arc<dyn BlockDevice<B>>,
}
```

其中，`CacheManager` trait 定义缓存层的缓存管理器，在预期的实现中，你并不需要在这里实现具体的缓存算法和数据结构，而将其留在 `kernel` 中实现。

#### 实现目标

1. 对设备缓存层、缓存管理器、缓存块进行抽象，并进行泛型设计。
2. 缓存管理器提供缓存块的获取、存储能力。
3. 缓存块实现脏数据标记、缓存块的读写，并为自身实现 `Drop` 时的自动写回。
4. 为设备缓存层实现 `BlockDevice<B>` trait。
5. 在内核中定义实际的缓存结构体，为它实现缓存管理器的相关功能。
6. 在最后的实现中，你应当能够将缓存层作为 `Fat16` 的 `inner`。

#### 加分项

1. 向操作系统暴露缓存的使用情况，并在系统状态中进行展示。
2. 验证缓存是否能带来一定的性能提升，设计相关测试并记录输出。

---

### 多核进程调度

在之前的实现中，进程只会在同一个核心上被调度执行，而在实际的多核系统中，多个核心可以并行地执行多个进程。

在这个目标中，你需要实现一个简单的多核调度器，使得内核能够在多个核心上调度多个进程。

!!! note "实现**不要求性能**，结合能够找到的资料和相关理论知识，能够验证多核运行就是成功。"

#### 实现目标

1. 修改 `apic` 初始化部分，利用 `cpuid` 使得它能够在多核系统中正确初始化多个 LAPIC。
2. 初始化部分建议查阅文档，并参考其他现有的实现。
3. 尝试实现操作系统的多核调度，并进行适当的日志显示。
4. 尝试在用户态验证，并输出运行的核心序号、进程 ID 等。

#### 加分项

1. 尝试实现进程亲和性，使得进程能够在特定的核心上运行。
2. 实现多核多队列模型，或尝试实现任一负载均衡算法（二选一）。

---

### TCP/IP 网络协议栈

在这个目标中，你需要实现基础的 TCP/IP 网络协议栈，包括 ARP、IPv4、ICMP、TCP、UDP 协议的支持，使得你的操作系统能够通过网络与宿主机、互联网交互。为了避免重复造轮子，我们使用 `virtio-drivers` 和 `smoltcp` 开发，它们分别是当前 Rust 社区最为成熟的面向嵌入式或裸机环境的 VirtIO 设备驱动和 TCP/IP 网络协议栈实现。

??? note "VirtIO 虚拟网卡及其驱动"

    VirtIO 是一套专为虚拟机环境设计的半虚拟化设备标准，目标是在虚拟机和宿主机之间提供高性能的 I/O 通道。QEMU 对 VirtIO 有原生支持。virtio-net 是 VirtIO 标准中的网卡设备，它连接到 PCI 总线上，是虚拟化实验环境中的首选网络设备。

    编写 PCI 设备驱动是比较繁琐的工作，所幸 [rCore OS 社区](https://rcore-os.cn)已经为我们造好了大部分轮子：[virtio-drivers](https://github.com/rcore-os/virtio-drivers) 提供了面向裸机环境的丰富的 VirtIO 驱动支持，帮助我们简化了很多底层细节工作。

首先修改脚本将启动 QEMU 的参数中的 `-net none` 改为如下，这将添加 VirtIO 网络设备到虚拟机 PCI 总线上，将宿主机 TCP 5555 端口转发到虚拟机的 5555 端口：

```bash
-netdev user,id=net0,hostfwd=tcp::5555-:5555,hostfwd=udp::5555-:5555 \
-device virtio-net-pci,netdev=net0,mac=52:54:00:12:34:56  # 指定一个任意的 MAC
```

前面的实验外设都是通过串口连接的，你可以在 [PCI 总线](../../wiki/pci.md)中了解 PCI 总线通信的基础知识。在现代的 VirtIO 1.0+ 版本下，网卡的厂商标识（Vendor ID）为 `0x1AF4`，设备标识（Device ID）为 `0x1041`。

??? note "VirtIO 网卡的 PCI 标识"

    所有标准的 VirtIO 设备（包括网卡、块设备等）都共享同一个厂商标识 `0x1AF4`，代表 Qumranet / Red Hat。设备标识则根据 VirtIO 协议版本的不同分为两种情况：

    - `0x1000`：对应传统模式(Legacy)。在老版本或兼容模式下，所有的 VirtIO 网卡都直接使用这个固定的设备标识。

    - `0x1041`：对应现代模式(Modern,1.0+)。VirtIO 1.0 规范规定，现代模式的 PCI Device ID 计算公式为 `0x1040 + Virtio_Device_ID`。由于网卡的 VirtIO Subsystem ID 是 `1`，因此计算得出 `0x1040 + 1 = 0x1041`。

#### 实现目标

整体的实现思路分为如下目标：

1. 创建新的数据结构 `VirtioHal` 实现 `virtio-drivers::Hal` trait，包括连续物理内存（DMA）的分配与释放、MMIO 物理地址向虚拟地址的转换等功能，这是使用网卡驱动 `virtio_drivers::device::net::VirtIONet` 的前提。

2. 创建新的数据结构 `VirtIONetDevice` 作为网卡设备的抽象。利用网卡驱动 `virtio_drivers::device::net::VirtIONet` 为 `VirtIONetDevice` 实现 `smoltcp::phy::Device` trait，这样就可以使用 `smoltcp` 提供的上层网络协议栈功能。

4. 初始化内核的时候扫描 PCI 总线找到 virtio-net 设备，创建并使用 `VirtIONetDevice` 来创建 `smoltcp::iface::Interface` 作为网络接口。

5. 创建新的数据结构 `Socket` 作为 `ysos::Resource` 的一个新的扩展资源类型，它将对应 `smoltcp::socket::Socket` 使得我们能够利用 `smoltcp` 提供上层的 ARP、IP、TCP/UDP 协议功能。

    - 推荐用 `smoltcp::iface::SocketSet` 存储（单个网络设备的）全部套间字，`ysos::Resource` 仅存储一个引用，这样的好处是可以方便 `poll()` 的调用；

    - 或者直接使用 `smoltcp::socket::Socket` 作为 `ysos::Resource::Socket`，在调用 `poll()` 的时候需要收集所有进程的资源表中的 Socket；

6. `smoltcp` 是单线程协作式调度，`smoltcp::iface::Interface::poll()` 需要在单个线程中定期调用才能推进协议栈状态，所以需要在内核主循环或定时中断处理程序中轮询网络接口，对每一个网络接口调用 `poll()` 处理它们的全部套间字。

7. 在内核态验证网络功能，尝试通过宿主机的 5555 端口和内核交互。

#### 加分项

1. 利用 `smoltcp` 支持更多的网络协议，例如 DHCP、IPv6、ICMPv6 等。
2. 实现网络相关系统调用，包括 `Socket` 资源的创建、建立连接、发送数据、接收数据等。
3. 利用上面所实现的系统调用，在用户态编写网络测试程序验证网络相关功能。

#### 参考资料

- [PCI 总线](../../wiki/pci.md)
- [OSDev: Virtio](https://wiki.osdev.org/Virtio)
- [VirtIO 规范（v1.4）](https://docs.oasis-open.org/virtio/virtio/v1.4/virtio-v1.4.html)
- [rCore: virtio设备驱动程序](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter9/2device-driver-2.html)
- [virtio-drivers 文档](https://docs.rs/virtio-drivers)
- [smoltcp 文档](https://docs.rs/smoltcp)
- [Computer Networks: A Systems Approach](https://book.systemsapproach.org/)

---
