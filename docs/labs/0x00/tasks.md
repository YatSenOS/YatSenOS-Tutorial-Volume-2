# 实验零：环境搭建与实验准备

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 配置实验环境

我们推荐在以下环境进行实验：

- Windows 10/11
- Ubuntu 22.04 LTS
- Ubuntu 22.04 LTS on WSL 2
- macOS with Apple Silicon

以上环境经过我们的测试和验证，可以正常进行实验。对于其他常用的 Linux 发行版，通常也可以正常进行实验，但我们不提供技术支持。

### 安装项目开发环境

!!! note "选择你喜欢的环境"

    本实验在 Windows 上进行项目开发是**完全可行的**，但是我们提供的各种工具的选项可能有所出入。

    在 Windows 平台上我们建议通过 VSCode + Python + CodeLLDB 插件进行开发、调试。

    在 Linux 平台上我们建议通过 VSCode (Remote) + Python / make + GDB 结合 gef 进行开发、调试。


- 对于选择使用 Windows 的同学，请参考 [Windows 环境配置](../../wiki/windows.md) 进行配置。

- 对于选择使用 Linux 的同学，请参考 [Linux 环境配置](../../wiki/linux.md) 进行配置。

- 对于选择使用 macOS 的同学，请安装 `brew` 和相应工具，参考 [Linux 环境配置](../../wiki/linux.md) 进行配置。

## 尝试使用 Rust 进行编程

我们预留了一些 Rust 编程任务，请你学习 Rust 并尝试在 Linux 环境下实现它们。

!!! tip "编程提示"

    - 如果代码格式不确定或写法不明确，记得常用 `cargo fmt` 和 `cargo clippy`。
    - 在你不熟悉新语言的时候，我们非常推荐你借助 LLM 进行学习。
    - 在满足题目描述的情况下，如有需要，**参数类型和返回值类型可以自行选择和修改**。

1. 使用 Rust 编写一个程序，完成以下任务：

    1. 创建一个函数 `count_down(seconds: u64)`

        该函数接收一个 u64 类型的参数，表示倒计时的秒数。

        函数应该每秒输出剩余的秒数，直到倒计时结束，然后输出 `Countdown finished!`。

    2. 创建一个函数 `read_and_print(file_path: &str)`

        该函数接收一个字符串参数，表示文件的路径。

        函数应该尝试读取并输出文件的内容。如果文件不存在，函数应该使用 `expect` 方法主动 panic，并输出 `File not found!`。

        !!! tip "尝试使用 `io::Result<()>` 作为返回值，并使用 `?` 将错误向上传递。"

    3. 创建一个函数 `file_size(file_path: &str) -> Result<u64, &str>`

        该函数接收一个字符串参数，表示文件的路径，并返回一个 `Result`。

        函数应该尝试打开文件，并在 `Result` 中返回文件大小。如果文件不存在，函数应该返回一个包含 `File not found!` 字符串的 Err。

        !!! tip "尝试将 `std::io::Result` 转换为 `std::Result`，你可能需要 `map_err` 等函数。"

    4. 在 `main` 函数中，按照如下顺序调用上述函数：

        - 首先调用 `count_down(5)` 函数进行倒计时
        - 然后调用 `read_and_print("/etc/hosts")` 函数尝试读取并输出文件内容
        - 最后使用 `std::io` 获取几个用户输入的路径，并调用 `file_size` 函数尝试获取文件大小，并处理可能的错误。

        !!! tip "`main` 函数的返回值可以是 `Result`，参考文档看看它能做什么？"

    注意：在处理文件操作时，需要使用到 Rust 的文件处理相关库，如 `std::fs` 和 `std::io`。在处理错误时，需要使用到 Rust 的错误处理机制，如 `expect` 和 `unwrap` 等。

2. 实现一个进行字节数转换的函数，并格式化输出：

    1. 实现函数 `humanized_size(size: u64) -> (f64, &'static str)` 将字节数转换为人类可读的大小和单位

        使用 1024 进制，并使用二进制前缀（B, KiB, MiB, GiB）作为单位

    2. 补全格式化代码，使得你的实现能够通过如下测试：

        ```rust
        #[test]
        fn test_humanized_size() {
            let byte_size = 1554056;
            let (size, unit) = humanized_size(byte_size);
            assert_eq!("Size :  1.4821 MiB", format!(/* FIXME */));
        }
        ```

        !!! note "Cargo 提供了良好的单元测试、集成测试支持，你可以参考 [编写测试](https://course.rs/test/write-tests.html) 进行使用"

            作为一个使用实例，可以在 `main.rs` 最后添加如下代码：

            ```rust
            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn some_test() {
                    // do something
                    // then assert the result
                }
            }
            ```

            上述测试代码将会在你执行 `cargo test` 时被执行。

            - `#[cfg(test)]` 表示该模块仅在测试时被编译
            - `use super::*;` 表示引入当前模块的所有内容（tests 模块是当前模块的子模块）
            - `#[test]` 表示该函数是一个测试函数，会被 `cargo test` 执行


3. **自行搜索学习如何利用现有的 crate** 在终端中输出彩色的文字

    输出一些带有颜色的字符串，并尝试直接使用 `print!` 宏输出一到两个相同的效果。

    尝试输出如下格式和内容：

    - `INFO: Hello, world!`，其中 `INFO:` 为绿色，后续内容为白色
    - `WARNING: I'm a teapot!`，颜色为黄色，加粗，并为 `WARNING` 添加下划线
    - `ERROR: KERNEL PANIC!!!`，颜色为红色，加粗，并尝试让这一行在控制行窗口居中
    - 一些你想尝试的其他效果和内容……

    !!! tip "如果你想进一步了解，可以尝试搜索 **ANSI 转义序列**"


4. 使用 `enum` 对类型实现同一化

    实现一个名为 `Shape` 的枚举，并为它实现 `pub fn area(&self) -> f64` 方法，用于计算不同形状的面积。

    - 你可能需要使用模式匹配来达到相应的功能
    - 请实现 `Rectangle` 和 `Circle` 两种 `Shape`，并使得 `area` 函数能够正确计算它们的面积
    - 使得你的实现能够通过如下测试：

        ```rust
        #[test]
        fn test_area() {
            let rectangle = Shape::Rectangle {
                width: 10.0,
                height: 20.0,
            };
            let circle = Shape::Circle { radius: 10.0 };

            assert_eq!(rectangle.area(), 200.0);
            assert_eq!(circle.area(), 314.1592653589793);
        }
        ```

        !!! note "可以使用标准库提供的 `std::f64::consts::PI`"

5. 实现一个元组结构体 `UniqueId(u16)`

    使得每次调用 `UniqueId::new()` 时总会得到一个新的不重复的 `UniqueId`。

    - 你可以在函数体中定义 `static` 变量来存储一些全局状态
    - 你可以尝试使用 `std::sync::atomic::AtomicU16` 来确保多线程下的正确性（无需进行验证）
    - 使得你的实现能够通过如下测试：

        ```rust
        #[test]
        fn test_unique_id() {
            let id1 = UniqueId::new();
            let id2 = UniqueId::new();
            assert_ne!(id1, id2);
        }
        ```

## 运行 UEFI Shell

### 初始化你的仓库

本实验设计存在一定的**前后依赖关系**，你可能需要在实验过程中自己逐步构建自己的操作系统。

为了更好的管理你的代码、更好的展示你的进度，建议使用 git 来管理本次实验代码。

!!! note "请注意，git 可以离线使用，我们并不要求你将代码上传到远程仓库。"

1. 克隆本仓库到本地：

    ```bash
    $ git clone https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2
    ```

2. 参考[实验 0x00 参考代码](https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2/tree/main/src/0x00/)的文件结构，初始化你的仓库。

    选择一个合适的目录，并拷贝此文件夹的内容到你的仓库中：

    !!! warning "不要直接运行如下代码，选择自己的工作文件夹，Windows 环境请注意命令和路径的格式"

    ```bash
    $ cp -Lr YatSenOS-Tutorial-Volume-2/src/0x00 /path/to/your/workdir
    ```

    !!! note "我们使用 `/path/to/your/workdir` 指代你的工作区，**请将其替换为你的工作区路径**"

3. 初始化你的仓库：

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
    ysos.py
    ```

### 使用 QEMU 启动 UEFI Shell

UEFI Shell 是一个基于 UEFI 的命令行工具，它可以让我们在 UEFI 环境下进行一些简单的操作。

在不挂载任何硬盘的情况下，我们可以使用如下命令启动 UEFI Shell：

!!! note "OVMF 是面向虚拟机的 UEFI 固件，参考 [UEFI 使用参考](../../wiki/uefi.md#ovmf)"

```bash
qemu-system-x86_64 -bios ./assets/OVMF.fd -net none -nographic
```

> 你可能会需要在 Windows 环境下使用 `qemu-system-x86_64.exe` 的绝对路径来代替这里的 `qemu-system-x86_64`

!!! note "QEMU 的相关参数含义，参考 [QEMU 使用参考](../../wiki/qemu.md)"

在预期下将会看到如下输出：

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

### 运行第一个 UEFI 程序

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

在项目根目录下运行 `make run` 或 `python ysos.py run`，预期得到如下输出：

```bash
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
[ INFO]: pkg/boot/src/main.rs@017: Hello World from UEFI bootloader!
```

至此，你已经做好了编写 OS 的准备工作。

## 思考题

1. 了解现代操作系统（Windows）的启动过程，`UEFI` 和 `Legacy`（`BIOS`）的区别是什么？

2. 尝试解释 Makefile 中的命令做了哪些事情？或许你可以参考下列命令来得到更易读的解释：

    ```bash
    python ysos.py run --dry-run
    ```

3. 利用 `cargo` 的包管理和 `docs.rs` 的文档，我们可以很方便的使用第三方库。这些库的源代码在哪里？它们是什么时候被编译的？

4. 为什么我们需要使用 `#[entry]` 而不是直接使用 `main` 函数作为程序的入口？

## 加分项

1. 😋 基于控制行颜色的 Rust 编程题目，参考 `log` crate 的文档，为不同的日志级别输出不同的颜色效果，并进行测试输出。

2. 🤔 基于第一个 Rust 编程题目，实现一个简单的 shell 程序：

    - 实现 `cd` 命令，可以切换当前工作目录（可以不用检查路径是否存在）
    - 实现 `ls` 命令，尝试列出当前工作目录下的文件和文件夹，以及有关的信息（如文件大小、创建时间等）
    - 实现 `cat` 命令，输出某个文件的内容

    !!! question "路径的切换是很容易出现问题的操作，你的程序能正常处理 `cd ../../././../a/b/c/../.././d/` 吗？"

3. 🤔 尝试使用线程模型，基于 `UniqueId` 的任务：

    - 尝试证明 `static mut` 变量在多线程下的不安全（可能获得相同的 `UniqueId`）
    - 尝试验证 `AtomicU16` 来实现 `UniqueId` 时的正确性

    !!! question "你对 Rust 的 `unsafe` 有什么看法？"
