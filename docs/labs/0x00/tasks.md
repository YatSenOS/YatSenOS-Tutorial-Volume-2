# 实验零：环境搭建与实验准备

!!! warning "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 安装 Linux 系统

Linux 有许多发行版，这里出于环境一致性考虑，推荐使用 Ubuntu 22.04。

其他发行版（如 Debian，Arch，Kali）也可以满足实验需求，但**请注意内核版本、QEMU 版本都不应低于本次实验的参考标准**。

### 使用 WSL2

对于 Windows 10/11 的用户来说，可以使用 WSL（Windows Subsystem Linux）来安装 Linux 系统，WSL 意为面向 Windows 的 Linux 子系统，微软为其提供了很多特性方便我们使用，我们可以在 Windows 上运行 Linux 程序。

你可以使用如下指令在 Windows 上安装 WSL2：

```bash
wsl --install -d Ubuntu
```

上述指令将会安装 WSL2 的全部依赖，并下载 Ubuntu 作为默认的发行版本。在安装过程中可能会重启电脑，安装完成后，你可以在 Windows 的应用列表中找到 Ubuntu，点击运行即可。

关于其他的配置，可以在网上找到大量的参考资料，请自行搜索阅读，或寻求 LLM 的帮助。

### 使用 Vmware Workstation 安装 Linux

参考 [Vmware Workstation 安装 Ubuntu 22.04 LTS](https://zhuanlan.zhihu.com/p/569274366) 教程。

### 安装 OS 运行环境

在正确安装 Linux 系统后，我们需要安装和配置开发环境，包括 gcc, make, qemu 等。

为了保障 Linux 软件源的正常、快速访问，请参考 [Ubuntu 软件仓库镜像使用帮助](https://help.mirrors.cernet.edu.cn/ubuntu/) 提供的文档进行软件源更换。

!!! note "校内镜像源"

    我们还提供有**仅供校内、不确保一定可用**的内网镜像源：[matrix 镜像站](https://mirrors.matrix.moe)

    请注意，使用上述镜像站会让你享受到更好的下载速度，但你同时也需要**承担不可用时带来的风险，并具有自主更换到其他镜像站的能力**。

1. 使用以下命令更新 apt 源：

    ```bash
    sudo apt update && sudo apt upgrade
    ```

2. 安装 qemu 等工具：

    ```bash
    sudo apt install qemu-system-x86 build-essential gdb
    ```

3. 安装 rustup：

    !!! note "rustup 安装过程中存在一些可配置选项，请按照默认选项进行安装。"

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
    ```

在安装完成后，请使用如下命令，确保你的相关软件包**不低于**如下标准：

```bash
$ rustc --version
rustc 1.74.0 (79e9716c9 2023-11-13)

$ qemu-system-x86_64 --version
QEMU emulator version 6.2.0 (Debian 1:6.2+dfsg-2ubuntu6.15)

$ gcc --version
gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0

$ gdb --version
GNU gdb (Ubuntu 12.1-0ubuntu1~22.04) 12.1
```

## 尝试使用 Rust 进行编程

<!-- DOC TODO -->

## 初始化你的仓库

本实验设计存在一定的**前后依赖关系**，你可能需要在实验过程中自己逐步构建自己的操作系统。

为了更好的管理你的代码、更好的展示你的进度，建议使用 git 来管理本次实验代码。

!!! note "请注意，git 可以离线使用，我们并不要求你将代码上传到远程仓库。"

1. 克隆本仓库到本地：

    ```bash
    $ git clone https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2
    ```

4. 参考[实验 0x00 参考代码](https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2/tree/main/src/0x00/)的文件结构，初始化你的仓库。

    选择一个合适的目录，并拷贝此文件夹的内容到你的仓库中：

    !!! warning "不要直接运行如下代码，选择你自己的工作文件夹"

    ```bash
    $ cp -Lr YatSenOS-Tutorial-Volume-2/src/0x00 /path/to/your/workdir
    ```

    !!! note "我们使用 `/path/to/your/workdir` 指代你自己的工作区，**请将其替换为你自己的工作区路径**"

5. 初始化你的仓库：

    ```bash
    $ cd /path/to/your/workdir
    $ git init
    $ git add .
    $ git commit -m "init"
    ```

4. 通过如下方式校验文件是否完整：

    ```bash
    $ git ls-tree --full-tree -r --name-only HEAD
    .gitignore
    Cargo.toml
    Makefile
    assets/OVMF.fd
    pkg/boot/.cargo/config
    pkg/boot/Cargo.toml
    pkg/boot/src/main.rs
    rust-toolchain.toml
    ```

## 使用 QEMU 启动 UEFI Shell

UEFI Shell 是一个基于 UEFI 的命令行工具，它可以让我们在 UEFI 环境下进行一些简单的操作。

在不挂载任何硬盘的情况下，我们可以使用如下命令启动 UEFI Shell：

!!! note "OVMF 是面向虚拟机的 UEFI 固件，参考 [UEFI 使用参考](../../wiki/uefi.md#ovmf)"

```bash
qemu-system-x86_64 -bios ./assets/OVMF.fd -net none -nographic
```

!!! note "QEMU 的相关参数含义，参考 [QEMU 使用参考](../../wiki/qemu.md)"

你将会看到如下输出：

```log
UEFI Interactive Shell v2.2
EDK II
UEFI v2.70 (EDK II, 0x00010000)
Mapping table
     BLK0: Alias(s):
          PciRoot(0x0)/Pci(0x1,0x1)/Ata(0x0)
Press ESC in 4 seconds to skip startup.nsh or any other key to continue.
Shell>
```

!!! tip "使用 <kbd>Ctrl</kbd> + <kbd>A</kbd> 后输入 <kbd>X</kbd> 可以退出 QEMU"


## YSOS 启动！

### 配置 Rust Toolchain

仓库提供的 `rust-toolchain.toml` 文件指定了需要使用的 Rust 工具链版本：

```toml
[toolchain]
channel = "nightly"
profile = "minimal"
components = [ "rust-src", "rustfmt", "clippy" ]
targets = [ "x86_64-unknown-uefi" ]
```

为了编译 UEFI 程序，我们需要使用 `x86_64-unknown-uefi` 编译目标。

同时，我们需要使用 `rust-src` 组件来编译标准库，使用 `rustfmt` 组件来格式化代码，使用 `clippy` 组件来获取一些代码编写建议。

为了编译内核和启用一些面向裸机的特性，我们需要使用 `nightly` 版本的 Rust 工具链。

在配置好的工作区中执行编译时，Rust 会自动下载并安装对应的工具链。

### 编写 UEFI 程序

编译一个 UEFI 程序时，我们没有操作系统所提供的标准库，也没有操作系统提供的 Interpreter，因此我们需要使用 `#![no_std]` 来声明我们的程序不依赖标准库，使用 `#![no_main]` 来声明我们的程序不依赖操作系统的入口函数。

同时，我们需要使用 `core` 和 `alloc` crate 来提供一些基本的数据结构和功能，使用 `uefi` crate 来提供 UEFI 程序运行时所需要的各种信息。

有关 [core](https://docs.rs/core/) crate 的介绍：

> **The Rust Core Library**
>
> The Rust Core Library is the **dependency-free** foundation of The **Rust Standard Library**. It is the portable glue between the language and its libraries, **defining the intrinsic and primitive building blocks of all Rust code**. It links to no upstream libraries, no system libraries, and no libc.
>
> The core library is minimal: **it isn’t even aware of heap allocation**, nor does it provide concurrency or I/O. These things require platform integration, and this library is **platform-agnostic**.


有关 [alloc](https://docs.rs/alloc/) crate 的介绍：

> **The Rust core allocation and collections library**
> This library provides smart pointers and collections for managing heap-allocated values.
>
> This library, like core, normally doesn’t need to be used directly since its contents are re-exported in the std crate. Crates that use the `#![no_std]` attribute however will typically not depend on std, so they’d use this crate instead.

有关 [uefi](https://docs.rs/uefi/) crate 的介绍：

> Rusty wrapper for the [Unified Extensible Firmware Interface](https://uefi.org).
>
> See the [Rust UEFI Book](https://rust-osdev.github.io/uefi-rs/HEAD/) for a tutorial, how-tos, and overviews of some important UEFI concepts. For more details of UEFI, see the latest [UEFI Specification](https://uefi.org/specifications).

!!! note "获取详细信息，参考 [Rust 语言基础](../../wiki/rust.md#善用-docsrs)"

在 `pkg/boot/src/main.rs` 中，完善如下的代码，修改注释部分，使用你的学号进行输出：

```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;

use core::arch::asm;
use uefi::prelude::*;

#[entry]
fn efi_main(image: uefi::Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");
    log::set_max_level(log::LevelFilter::Info);

    let std_num = /* FIXME */;

    loop {
        info!("Hello World from UEFI bootloader! @ {}", std_num);

        for _ in 0..0x10000000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}
```

`efi_main` 通过 `#[entry]` 被指定为 UEFI 程序的入口函数，`efi_main` 函数的参数 `system_table` 是一个 `SystemTable<Boot>` 类型的变量，它包含了 UEFI 程序运行时所需要的各种信息，如内存映射、文件系统、图形界面等。

在 `efi_main` 函数中，首先对 `system_table` 和 `log` 进行初始化，然后进入一个死循环，每次循环输出一条日志后等待一段时间。

在项目根目录下运行 `make run`，预期得到如下输出：

```bash
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
```

至此，你已经做好了编写 OS 的准备工作。

## 思考题

1. 了解现代操作系统（Windows）的启动过程，`legacy BIOS` 和 `UEFI` 的区别是什么？
2. 尝试解释 Makefile 中的命令做了哪些事情？
2. 利用 `cargo` 的包管理和 `docs.rs` 的文档，我们可以很方便的使用第三方库。这些库的源代码在哪里？它们是什么时候被编译的？
3. 为什么我们需要使用 `#[entry]` 而不是直接使用 `main` 函数作为程序的入口？
