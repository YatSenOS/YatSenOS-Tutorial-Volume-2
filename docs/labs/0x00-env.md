# 实验零：环境搭建与实验准备

!!! tip "代码是一场无声的交流，有些人是优秀的诗人，能够将抽象的想法转化为优雅的语言，而有些人则是忠实的翻译者，将逻辑转换成计算机可理解的语言。"

## 实验目的

1. Rust 学习和巩固，了解标准库提供的基本数据结构和功能。
2. QEMU 与 Rust 环境搭建，尝试使用 QEMU 启动 UEFI Shell。
3. 了解 x86 汇编、计算机的启动过程，UEFI 的启动过程，实现 UEFI 下的 `Hello, world!`。

## 实验环境

我们支持并推荐如下平台进行实验：

- Ubuntu 22.04 LTS
- Ubuntu 22.04 LTS with WSL 2
- macOS with Apple Silicon（请自行安装相关依赖）
- 其他可行的平台，但我们不提供技术支持

需要安装的基本软件环境，括号中提供在 Ubuntu 中对应的包名：

- QEMU x86_64 (qemu-system-x86)
- Rust nightly toolchain (rustup)
- make, gcc, gdb 等基本编译工具 (build-essential)

## 实验基础知识

!!! note "善用 LLM 进行学习"

    对于现代计算机专业的学生，我们建议并要求大家学习借助 LLM（Large Language Model）进行学习，这是一种非常有效的学习方法，可以帮助你更快的学习到知识。

    对于不理解的知识点和概念，我们建议优先参考文档、借助 LLM 进行实践，在仍然无法解决的情况下再向他人提问。

### Linux 使用基础

Linux 是一个开源的类 Unix 操作系统内核，它是一个典型的多用户、多任务的操作系统，可以运行在各种平台上，如服务器、PC、手机等。常见的 Linux 发行版有 Ubuntu、Debian、Arch、Kali 等。

与常规的 GUI 交互方式不同，Linux 系统通常使用命令行来与用户进行交互，命令行是一种通过键入命令来完成与计算机的交互的方式，它可以让用户完成一个操作系统所能提供的一切功能。

本次操作系统实验的最终目标也是实现一个能够和用户进行命令行交互的操作系统，因此我们推荐你多多使用命令行来完成实验。

你可以通过下面的一些链接来对命令行的使用进行学习，也可以把它们作为参考文档随用随取：

- [The Missing Semester](https://missing-semester-cn.github.io/2020/shell-tools)
- [UNIX basics tutorial](https://berkeley-scf.github.io/tutorial-unix-basics/)
- [GNU/Linux Command-Line Tools Summary](https://tldp.org/LDP/GNU-Linux-Tools-Summary/html/index.html)
- [「实用技能拾遗」课程 S1 by TonyCrane](https://slides.tonycrane.cc/PracticalSkillsTutorial/2023-spring-cs/#/)
- [「实用技能拾遗」课程 S2 by TonyCrane](https://slides.tonycrane.cc/PracticalSkillsTutorial/2023-fall-ckc/#/)

### UEFI 启动基础

#### 计算机的启动

在「计算机组成原理」课程中，各位应该都编写过一个简单的 CPU，而这个 CPU 带有一段预先编写好的程序。在那时，我们的 CPU 只需要支持最简单的固定的外部设备，因此我们可以直接编写程序来满足这些外部设备的需要，并且把程序固化到 CPU 的 ROM 中。

但是，现实中的计算机（指 IBM PC）具有多种不同的设备，而 CPU 需要适应多种外部设备和执行环境，因此人们发明了 DDR、PCI、PS/2、USB 等多种不同的通信协议，从而构建了 CPU 和外部设备通信的标准；同时，计算机通过引入 BIOS 这个中间层来实现设备的初始化。

在现在的计算机中，主板芯片组（如 AMD B450）会带有一块小型的 ROM，其中存放了初始化计算机的各种设备的代码。主板制造商通过连接 CPU 的地址线，使得芯片组上的程序的入口地址和 CPU 通电后的默认指令地址相同。这样，计算机在启动后就会执行这段代码，来检测内存、初始化主板芯片组、检测设备，而这段代码就被称为 BIOS。

在完成了基本的初始化过程后，BIOS 将加载磁盘扇区 0 的内容，放置在 `0x7C00` 地址，然后跳转到该地址执行引导程序。引导程序将完成探测内存布局、加载操作系统内核等工作，并最终进入到操作系统内核中。

这里叙述的引导过程有所简化，没有讨论在没有内存的情况下如何完成内存的初始化等内容。知乎用户[老狼](https://www.zhihu.com/people/mikewolfwoo)有一系列文章叙述了计算器 BIOS 和 UEFI 相关的知识，包括 Cache As RAM、可信启动等，各位读者可以简单参考作为补充。

#### BIOS 与 UEFI

为了有效地让操作系统（引导程序）能够知晓系统的情况（如内存布局，也就是内存地址的分布情况），BIOS 向操作系统提供了一系列 [系统调用](https://wiki.osdev.org/BIOS)，这些调用通过中断触发。当触发中断后，BIOS 之前在 CPU 中注册的中断处理程序就会被执行，从而执行对应的功能。

BIOS 本身没有任何的规范定义，而是各个厂家在漫长的历史实践中，建立了一系列约定，使得各个不同厂家生产的计算机能够获得相似的表现。但是，因为缺乏规范，这样的历史实践并不能保证在所有的计算机上表现一致。

此外，在 BIOS 启动的引导程序运行时，系统的内存布局存在一个特殊的约定，从而使得引导程序不必获得完整的内存布局就可以执行一些简单的工作。这段内存的大小为 1MB，对于较大的引导程序来说有些紧张。同时，一个扇区的大小只有 512B，这也限制了引导程序的代码大小。因此，现在的引导扇区程序（如 Grub）往往采用两阶段引导的模式，第一阶段称为 Boot，放置在引导扇区中；而第二阶段称为 Loader，放置在磁盘的一个特殊区域；从而突破 BIOS 的代码和内存限制。

最后，BIOS 基于中断的函数调用产生了两个问题。其一是，中断调用的效率较低，导致计算机启动缓慢。其二是，引导程序和操作系统为了实现高级功能，往往会注册自己的中断处理程序，这可能和 BIOS 发生冲突。

为了更好地解决这一问题，Intel 联合 PC 厂商建立了 UEFI 标准，这是下一代的计算机固件的接口定义。有了 UEFI，操作系统开发人员能够更好地去实现操作系统的功能，编写更加复杂的引导程序，甚至可以在引导程序或固件管理界面访问网络、显卡等高级功能。

!!! note "编写 UEFI 程序并引导启动的相关内容，将会在下一次实验中进一步详细讨论。"

### Rust 基础

Rust 是一门系统编程语言，它有更强的类型检查和内存安全保证，可以避免很多 C/C++ 中常见的内存错误，如缓冲区溢出、空指针引用等。

Rust 语言的基础语法可以参考 [Rust圣经](https://course.rs/) 或者 [Rust Programming Language](https://doc.rust-lang.org/book/) 等资料。

当熟悉 Rust 的语法与特性后，可以尝试去完成 [Rustlings](https://github.com/rust-lang/rustlings) 的练习，这些练习可以帮助你更好的理解 Rust 语言的特性。

其他可参考的学习资料：

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [清华的 Rust 课程](https://lab.cs.tsinghua.edu.cn/rust/)

值得注意的是，本实验内容并不要求你对 Rust 语言有深入的了解，只需要你能够 **理解并使用** Rust 语言的以下内容：

- **基本语法**

    变量绑定、常量、表达式、基本类型、条件语句、模式匹配、函数

- **所有权与结构化数据**

    所有权、移动语义、借用与可变引用、结构体、元组结构体、单位元结构体、枚举

- **标准库**

    `String`、`Vec<T>`、`Result<T, E>`、`Option<T>`、错误处理、单元测试

- **泛型、特型与生命周期**

    泛型、特型、标准库提供的常用特性、生命周期入门

- **项目管理与常用库**

    Cargo 项目结构、命名规范、智能指针

### QEMU 使用基础

QEMU 是一个开源的虚拟机软件，它可以模拟多种硬件平台，如 x86、ARM、MIPS 等，可以运行在多种操作系统上，如 Linux、Windows、macOS 等。

可以使用类似于如下的命令行运行 QEMU：

```sh
$ qemu-system-x86_64 -bios ./ovmf.fd -net none \
    -m 96M -drive format=raw,file=fat:rw:./esp -nographic
```

其中 `-bios` 指定了 UEFI 的固件，`-net none` 指定了网络设备，`-m` 指定了内存大小，`-drive` 指定了硬盘，`-nographic` 指定了不使用图形界面，转而将串口 IO 重定向到标准输入输出。

为了退出 QEMU，可以使用 `Ctrl + A` 再输入 `A`。

在调试时，可以使用 `-s` 参数来启动 GDB 调试服务，是 `-gdb tcp:1234` 的简写，并使用 `-S` 参数来暂停 CPU 的执行，等待 GDB 连接。

当遇到 Triple Fault 时，可以使用 `-no-reboot` 参数来阻止 QEMU 重启。并使用 `-d int,cpu_reset` 参数来打印中断和 CPU 重置的调试信息，这部分对于中断调试很有帮助。

可以参考 [官方文档](https://www.qemu.org/docs/master/system/index.html) 获取更多的 QEMU 使用信息。


## 实验步骤

!!! warning "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

### 安装 Linux 系统

Linux 有许多发行版，这里出于环境一致性考虑，推荐使用 Ubuntu 22.04。

其他发行版（如 Debian，Arch，Kali）也可以满足实验需求，但**请注意内核版本、QEMU 版本都不应低于本次实验的参考标准**。

#### 使用 WSL2

对于 Windows 10/11 的用户来说，可以使用 WSL（Windows Subsystem Linux）来安装 Linux 系统，WSL 意为面向 Windows 的 Linux 子系统，微软为其提供了很多特性方便我们使用，我们可以在 Windows 上运行 Linux 程序。

你可以使用如下指令在 Windows 上安装 WSL2：

```bash
wsl --install -d Ubuntu
```

上述指令将会安装 WSL2 的全部依赖，并下载 Ubuntu 作为默认的发行版本。在安装过程中可能会重启电脑，安装完成后，你可以在 Windows 的应用列表中找到 Ubuntu，点击运行即可。

关于其他的配置，可以在网上找到大量的参考资料，请自行搜索阅读，或寻求 LLM 的帮助。

#### 使用 Vmware Workstation 安装 Linux

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
    sudo apt install qemu-system-x86 build-essential
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

#### 初始化你的仓库

本实验设计存在一定的**前后依赖关系**，你可能需要在实验过程中自己逐步构建自己的操作系统。

为了更好的管理你的代码、更好的展示你的进度，建议使用 git 来管理本次实验代码。

!!! note "请注意，git 可以离线使用，我们并不要求你将代码上传到远程仓库。"

1. 克隆本仓库到本地：

    ```bash
    $ git clone https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2
    ```

4. 参考[实验 0x00 参考代码](https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2/tree/main/src/0x00/)的文件结构，初始化你的仓库。

    选择一个合适的目录，并拷贝此文件夹的内容到你的仓库中：

    !!! warning "不要直接运行如下代码"

    ```bash
    $ cp -Lr YatSenOS-Tutorial-Volume-2/src/0x00/* .
    ```

5. 初始化你的仓库：

    ```bash
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

#### 使用 QEMU 启动 UEFI Shell

UEFI Shell 是一个基于 UEFI 的命令行工具，它可以让我们在 UEFI 环境下进行一些简单的操作。

在不挂载任何硬盘的情况下，我们可以使用如下命令启动 UEFI Shell：

```bash
qemu-system-x86_64 -bios ./assets/OVMF.fd -net none -nographic
```

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

#### YSOS 启动！

##### 配置 Rust Toolchain

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

##### 编写 UEFI 程序

编译一个 UEFI 程序时，我们没有操作系统所提供的标准库，也没有操作系统提供的 Interpreter，因此我们需要使用 `#![no_std]` 来声明我们的程序不依赖标准库，使用 `#![no_main]` 来声明我们的程序不依赖操作系统的入口函数。

同时，我们需要使用 `core` 和 `alloc` crate 来提供一些基本的数据结构和功能，使用 `uefi` crate 来提供 UEFI 程序运行时所需要的各种信息。

!!! note "使用 `docs.rs` 获取你需要的信息"

    Rust 提供了 [docs.rs](https://docs.rs/) 来帮助我们查看 crate 的文档，你可以在其中搜索你需要的 crate，然后查看其文档。

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

`efi_main` 通过 `#[entry]` 被指定为 UEFI 程序的入口函数，`efi_main` 函数的参数 `system_table` 是一个 `SystemTable<Boot>` 类型的变量，它包含了 UEFI 程序运行时所需要的各种信息，如内存映射、文件系统、图形界面等。

!!! note "你可以通过上述文档进一步详细了解这些内容。"

在 `efi_main` 函数中，首先对 `system_table` 和 `log` 进行初始化，然后进入一个死循环，每次循环输出一条日志后等待一段时间。

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

在项目根目录下运行 `make run`，预期得到如下输出：

```bash
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
```

!!! tip "尝试解释 `Makefile` 中所进行的操作"

至此，你已经做好了编写 OS 的准备工作。

接下来的实验中，我们将会进一步学习如何启用一个最小化的内核。

## 思考题

1. 了解现代操作系统（Windows）的启动过程，`legacy BIOS` 和 `UEFI` 的区别是什么？
2. 利用 `cargo` 的包管理和 `docs.rs` 的文档，我们可以很方便的使用第三方库。这些库的源代码在哪里？它们是什么时候被编译的？
3. 为什么我们需要使用 `#[entry]` 而不是直接使用 `main` 函数作为程序的入口？
