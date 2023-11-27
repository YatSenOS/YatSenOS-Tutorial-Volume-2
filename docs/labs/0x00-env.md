# 实验零：环境搭建与实验准备

1. 安装 Linux 系统虚拟机或者 WSL（Windows Subsystem Linux）
2. 安装 OS 运行环境
3. 学习基础 Rust 语法
4. UEFI 启动

## 安装 Linux 系统

Linux 有许多发行版，这里推荐使用 Ubuntu 22.04 LTS，当然其他发行版（如 Arch, Kali）也可以，注意版本不要太低。

### WSL 安装 Linux

使用 Windows 10/11 的用户来说，可以使用 WSL（Windows Subsystem Linux）来安装 Linux 系统，WSL 是 Windows 10/11 内置的 Linux 子系统，可以在 Windows 上运行 Linux 程序。由于本次实验没有图形界面的需求，因此 WSL 也能满足我们的需求。

使用管理员权限打开 PowerShell，输入以下命令：

```powershell
# 启用适用于 Linux 的Windows 子系统
dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart

# 启用虚拟化
dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart

# 设置 WSL 2 为默认版本
wsl --set-default-version 2
```

中间可能需要重启电脑。

之后打开 Microsoft Store，搜索 Ubuntu，选择 Ubuntu 22.04 LTS，点击获取即可。 

或者使用最简单的办法运行：
```
wsl --install -d Ubuntu-22.04
```

### 使用 Vmware Workstation 安装 Linux

参考 [Vmware Workstation 安装 Ubuntu 22.04 LTS](https://zhuanlan.zhihu.com/p/569274366) 教程。

## 安装 OS 运行环境

在安装好 Linux 系统后，需要安装开发环境，包括 gcc, make, qemu 等。

首先将 apt 源更改为 Matrix 源（使用校内的镜像站速度更快），使用你最喜欢的编辑器来修改 `/etc/apt/sources.list` 文件，将其中的内容替换为以下内容：（可选）

```bash
# 默认注释了源码镜像以提高 apt update 速度，如有需要可自行取消注释
deb https://mirrors.matrix.moe/ubuntu/ jammy main restricted universe multiverse
# deb-src https://mirrors.matrix.moe/ubuntu/ jammy main restricted universe multiverse
deb https://mirrors.matrix.moe/ubuntu/ jammy-updates main restricted universe multiverse
# deb-src https://mirrors.matrix.moe/ubuntu/ jammy-updates main restricted universe multiverse
deb https://mirrors.matrix.moe/ubuntu/ jammy-backports main restricted universe multiverse
# deb-src https://mirrors.matrix.moe/ubuntu/ jammy-backports main restricted universe multiverse
deb https://mirrors.matrix.moe/ubuntu/ jammy-security main restricted universe multiverse
# deb-src https://mirrors.matrix.moe/ubuntu/ jammy-security main restricted universe multiverse
```

之后执行以下命令更新 apt 源：

```bash
sudo apt update && sudo apt upgrade
```

安装 qemu 等工具：

```bash
sudo apt install qemu-system-x86 build-essential
```

安装 rustup：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

安装好后可以进入项目代码目录，执行 `make build` 来编译项目，执行 `make run` 来运行项目。

## Rust 基础

Rust 是一门系统编程语言，它有更强的类型检查和内存安全保证，可以避免很多 C/C++ 中常见的内存错误，如缓冲区溢出、空指针引用等。

Rust 语言的基础语法可以参考 [Rust圣经](https://course.rs/) 或者 [Rust Programming Language](https://doc.rust-lang.org/book/) 等资料。

当熟悉 Rust 的语法与特性后，需要你去完成 [Rustlings](https://github.com/rust-lang/rustlings) 的所有练习，这些练习可以帮助你更好的理解 Rust 语言的特性。

其他可参考的学习资料：
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [清华的 Rust 课程](https://lab.cs.tsinghua.edu.cn/rust/)

注：这一部分不要求你对 Rust 语言有深入的了解，只需要你能够理解 Rust 语言的基本语法，以及能够阅读 Rust 代码即可。

## 实验准备——UEFI 启动 

### 前置知识 BIOS

BIOS（Basic Input/Output System）是一种计算机固件（firmware），它位于计算机主板上的一个芯片中。BIOS 提供了计算机系统启动时的最基本的硬件初始化和自检功能，以及与操作系统和硬件设备的交互接口。

BIOS 的主要功能包括以下几个方面：

自检（POST）：BIOS 进行电源自检（Power-On Self-Test），检测计算机硬件的状态和功能是否正常。这些自检过程包括检查内存、检测硬盘、检测键盘和显示器等。

硬件初始化：在计算机开机时，BIOS 负责对各种硬件设备进行初始化，包括处理器、内存、硬盘、显示适配器等。它检测和配置这些硬件设备，为它们提供基本的运行环境。

启动顺序：BIOS 确定计算机启动时的设备顺序。它指定计算机首先从哪个设备（如硬盘、光驱、USB 等）读取操作系统的引导程序。

### UEFI

UEFI（Unified Extensible Firmware Interface）是一种相对新型的固件接口，是 BIOS 的一种升级替代方案。

其实他们做的事情没有本质区别，都是**初始化硬件和提供硬件的软件抽象**，但是具体的组成和实现方式有很大的不同，细节可参考 [UEFI启动流程概览](https://zhuanlan.zhihu.com/p/483888207)，但本次实验不需要深入了解。

下面是 BIOS 和 UEFI 的对比：
|  | BIOS | UEFI |
| --- | --- | --- |
| 启动方式 | 16 位实模式 | 保护模式 |
| 引导方式 | MBR | ESP系统分区+GPT |
| 引导程序 | MBR 的第一个扇区 | EFI程序 |

对于操作系统的开发初期而言，使用 UEFI 大大简化了流程

1. 可以直接编译出一个 EFI 程序，虚拟机直接加载运行。
2. 不需要编写汇编，从 MBR 切换到保护模式。

而且我们可以使用 Rust 的 [uefi-rs](https://github.com/rust-osdev/uefi-rs) 库来调用 UEFI 的接口，避免了自己编写汇编来调用 UEFI 接口的麻烦。

### YSOS 启动！

打开项目代码 `pkg/boot/src/main.rs` 文件

由于我们 Rust 标准库 `std` 需要通过系统调用来获取操作系统的服务，而我们要编写的 YSOS 是运行在裸机上的，不能依赖操作系统的服务，因此我们需要使用 `#![no_std]` 来声明我们的程序不依赖标准库，使用 `#![no_main]` 来声明我们的程序不依赖操作系统的入口函数。

```rust
#![no_std]
#![no_main]
```

`efi_main` 通过 `#[entry]` 被指定为 UEFI 程序的入口函数，`efi_main` 函数的参数 `system_table` 是一个 `SystemTable<Boot>` 类型的变量，它包含了 UEFI 程序运行时所需要的各种信息，如内存映射、文件系统、图形界面等。
在 `efi_main` 函数中，首先对 `system_table` 和 `log` 进行初始化，然后进入一个死循环，每次循环输出一条日志后等待一段时间。

```rust
#[entry]
fn efi_main(image: uefi::Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");
    log::set_max_level(log::LevelFilter::Info);

    loop {
        info!("Hello World from UEFI bootloader!");

        for _ in 0..0x10000000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}
```

在项目根目录下运行 `make run`，可以看到如下输出：

```bash
BdsDxe: failed to load Boot0001 "UEFI QEMU DVD-ROM QM00003 " from PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Master,0x0): Not Found
BdsDxe: loading Boot0002 "UEFI QEMU HARDDISK QM00001 " from PciRoot(0x0)/Pci(0x1,0x1)/Ata(Primary,Master,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU HARDDISK QM00001 " from PciRoot(0x0)/Pci(0x1,0x1)/Ata(Primary,Master,0x0)
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
```

这就表示程序进入了 UEFI Shell。

输入 `Ctrl + a` 再输入 `x` 退出 qemu。
