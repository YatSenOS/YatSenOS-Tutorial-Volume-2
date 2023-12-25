# 实验一：操作系统的启动

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 编译内核 ELF

与常规实验中直接将内核编译为二进制文件不同，而本实验需要将内核编译为 ELF 格式的文件，并将它存储在 UEFI 可以访问的文件系统中。

!!! note "请阅读 [ELF 文件格式](../../wiki/elf.md) 部分，了解什么是 ELF 文件。"

为了达到这一目的，需要对 Rust 的编译目标、链接配置进行一些修改，这部分内容已经为大家准备好，你可以在 [实验 0x01 参考代码](https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2/tree/main/src/0x01/pkg/kernel/config) 中看到进行这些配置的方式。

在 `pkg/kernel/config` 中，引用了 `config/x86_64-unknown-none.json` 的编译目标配置，该配置文件如下所示：

```json
{
  "llvm-target": "x86_64-unknown-none",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "linker-flavor": "ld.lld",
  "target-endian": "little",
  "target-pointer-width": "64",
  "target-c-int-width": "32",
  "arch": "x86_64",
  "os": "none",
  "executables": true,
  "linker": "rust-lld",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float",
  "panic-strategy": "abort",
  "pre-link-args": {
    "ld.lld": ["-Tpkg/kernel/config/kernel.ld", "-export-dynamic"]
  }
}
```

这个配置文件描述了 `cargo` 和 `rustc` 应该如何编译内核，这里指定了端序、指针长度、架构、链接器、链接脚本、目标架构等信息。具体细节留作读者自行探索。

`"-Tpkg/kernel/config/kernel.ld"` 指定了链接脚本的位置，该链接脚本描述了内核的链接方式，其基本内容如下所示：

```ld
ENTRY(_start)
KERNEL_BEGIN = 0xffffff0000000000;

SECTIONS {
  . = KERNEL_BEGIN;

  . ... ALIGN(4K):
  {
    *( ... )
  }

  ...
}
```

它描述了内核的入口地址为 `_start`，并将此 ELF 文件对应的虚拟地址空间的起始地址设置为 `0xffffff0000000000`。此外，它还描述了内核的各个段的链接方式。

值得注意的是，为了后续实验代码编写的便利，这里将内核的代码段、数据段、BSS 段等都设置为了 4KB 对齐。

!!! question "实验任务"

    在 `pkg/kernel` 目录下运行 `cargo build --release`，之后找到编译产物，并使用 `readelf` 命令查看其基本信息，回答以下问题：

    - 请查看编译产物的架构相关信息，与配置文件中的描述是否一致？
    - 请找出编译产物的 segments 的数量，并且说明每一个 segments 的权限、是否对齐，以及找出内核的入口点。

## 在 UEFI 中加载内核

经过上述的配置，内核将会被编译为一个 ELF 文件，下一步需要在 UEFI 程序中加载这个文件、准备好内核的运行环境，最后跳转到内核进行执行。这一过程中，这个 UEFI 程序所扮演的角色就是 bootloader。

!!! note "为了帮助大家了解如何进行项目代码的结构组织，本次实验给出的参考代码中包含了完整的文件组织结构。"

实验在 `pkg/boot` 中提供了一些基本的功能实现：

- `allocator.rs`：为 `uefi` crate 中的 `UEFIFrameAllocator` 实现 `x86_64` crate 所定义的 `FrameAllocator<Size4KiB>` trait，以便在页面分配、页表映射时使用。
- `config.rs`：提供了一个读取并解析 `boot.conf` 的基本实现，可以使用它来自定义 bootloader 的行为、启动参数等等。
- `fs.rs`：提供了在 UEFI 环境下打开文件、列出目录、加载文件、释放 `ElfFile` 的功能，你可以参考这部分代码了解与文件系统相关操作的基本内容。在后期的实验中，你将自己实现对文件系统的相关操作。
- `lib.rs`：这部分内容定义了 bootloader 将要传递给内核的信息、内核的入口点、跳转到内核的实现等等。定义在 `lib.rs` 中是为了能够在内核实现中引用这些数据结构，确保内核与 bootloader 的数据结构一致。
- `main.rs`：这里是 bootloader 的入口点，你可以在这里编写你的 bootloader 代码。

同时在 `pkg/elf` 中实验提供了加载 ELF 文件的相关代码，其中也有需要你自己实现的部分。

这一个 package 将被 `boot` 和 `kernel` 共同引用，并用于加载内核和用户程序的 ELF 文件。你可以参考 `Cargo.toml` 来了解这一部分的依赖关系。

!!! warning "请留意代码中标注有 `FIXME:` 的部分，这些部分需要你自己实现。"

此部分的核心代码任务被放置在 `pkg/boot/src/main.rs` 中，你需要按照下列步骤完成这一部分的实现。

### 加载相关文件

1. 加载配置文件：使用配置文件描述内核栈大小、内核栈地址等内容。
2. 加载内核 ELF：根据配置文件中的信息，加载内核 ELF 文件到内存中，并将其加载为 `ElfFile` 以便进行后续的操作。

为了方便你的实现，在 `pkg/boot/src/fs.rs` 中，提供了一些函数可供调用，对于一个正常的文件读取流程，你可以参考如下代码：

```rust
let mut file = open_file(bs, file_path);
let buf = load_file(bs, &mut file);
```

### 更新控制寄存器

`x86_64` 封装了一些控制寄存器的操作，你可以在 `x86_64` crate 中找到它们的定义。

其中对于一些标志位的操作使用了 `bitflags` 宏进行实现，你可以参考 [bitflags](https://docs.rs/bitflags/latest/bitflags/) 了解它的使用方法。

更新寄存器的值时，可以使用 `update` 函数，以 `Cr0::update` 为例，这个函数的定义如下：

```rust
#[inline]
pub unsafe fn update<F>(f: F)
where
    F: FnOnce(&mut Cr0Flags),
{
    let mut flags = Self::read();
    f(&mut flags);
    unsafe {
        Self::write(flags);
    }
}
```

它接受一个闭包作为参数，这个闭包接受一个 `&mut Cr0Flags` 的参数，你可以在这个闭包中对 `Cr0Flags` 进行修改，最后通过 `Self::write` 将修改后的值写入寄存器。

对于 `Cr0Flags` 的定义你可以在 `x86_64` crate 中找到，它是一个 `bitflags` 宏生成的结构体，你可以通过 `flags.insert`、`flags.remove` 等方法对其进行修改。

一个简单的例子如下，相关标志位的具体定义可以通过 IDE 跳转或查阅文档进行了解：

```rust
unsafe {
  Cr0::update(|f| f.insert(Cr0Flags::CACHE_DISABLE));
}
```

### 映射内核文件

在成功读取内核 ELF 文件并禁用根页表的写保护后，你需要将内核的代码段、数据段、BSS 段等映射到虚拟地址空间中。你可以参考和使用 `pkg/elf/src/lib.rs` 中的相关函数进行映射工作。

!!! tip "一些提示"

    - `physical_memory_offset` 在配置结构体中，它描述了物理地址进行线性映射的偏移量，你可能会使用到。
    - 你可以使用如下的代码初始化帧分配器：

        ```rust
        let mut frame_allocator = UEFIFrameAllocator(bs);
        ```

    - `pkg/elf/src/lib.rs` 中的 `load_segment` 函数需要你进行补全。**请认真学习实验文档所提供的有关分页内存权限管理、内核 ELF 文件格式的内容，以便你能够完成这一部分的实现。**
    - 阅读配置文件定义中有关内核栈的内容，利用相关参数来初始化内核栈。
    - 别忘了将你修改过的控制寄存器恢复原样。

### 跳转执行

在将内核的 ELF 文件加载并映射到合适的虚拟地址空间后，下一个目标就是跳转到内核的入口点，从而开始执行内核代码。为了达到这个目标，你还需要完成以下任务：

1. 退出启动时服务：通过调用 `exit_boot_services` 退出启动时服务，这样 UEFI 将会回收一些内存资源、退出对硬件的控制，从而将控制权交给内核。
2. 跳转到内核：通过调用 `jump_to_entry` 跳转到内核的入口点，开始执行内核代码。

### 调试内核

最后，你需要检验是否成功加载了内核：

将断点设置在内核的入口处，使用 GDB 调试你的 bootloader，观察内核的入口点是否正确、页面是否按照 ELF 的描述正确映射、内核的代码段是否正确加载等等。

*你可能需要先完成后续章节进行调试环境的配置。*

!!! tip "遇到了奇怪的问题？尝试更改 `log::set_max_level(log::LevelFilter::Info);` 来调整日志输出的等级，以便你能够观察到更多的日志输出。"

!!! question "实验任务"

    完成上述代码任务，回答如下的问题：

    - `set_entry` 函数做了什么？为什么它是 unsafe 的？
    - 如何为内核提供直接访问物理内存的能力？你知道几种方式？代码中所采用的是哪一种？
    - `jump_to_entry` 函数做了什么？它将传递给内核参数留在了哪里？借助调试器进行说明。
    - 为什么 ELF 文件中不描述栈的相关内容？栈是如何被初始化的？它可以被任意放置吗？
    - `entry_point!` 宏做了什么？内核为什么需要使用它声明自己的入口点？

## 搭建调试环境

依据[调试教程](../../wiki/debug.md)的相关内容，搭建基于命令行的 GDB 调试环境。

作为实验的推荐调试环境，你需要配置好 `gef` 插件以进行更加灵活的二进制调试。同时利用 VSCode 进行调试也是一个不错的选择，鼓励你进行尝试，它将会作为实验的加分项目之一。

!!! question "实验任务"

    根据上述调试教程，回答以下问题，并给出你的回答与必要的截图：

    - 请解释指令 `layout asm` 的功能。倘若想找到当前运行内核所对应的 Rust 源码，应该使用什么 GDB 指令？
    - 假如在编译时没有启用 `DBG_INFO=true`，调试过程会有什么不同？
    - 你如何选择了你的调试环境？截图说明你在调试界面（TUI 或 GUI）上可以获取到哪些信息？

## UART 与日志输出

### 串口

请阅读[串口输出简介](../../wiki/uart.md)一节，完成以下问题。

1. 思考题：在 QEMU 中，我们通过指定 `-nographic` 参数来禁用图形界面，这样 QEMU 会默认将串口输出重定向到主机的标准输出。假如我们将 `Makefile` 中取消该选项，QEMU 的输出窗口会发生什么变化？请观察指令 `make QEMU_OUTPUT= run` 的输出，结合截图分析对应现象。

!!! tip "现象观察提示"
若此时启动 QEMU 的输出提示是 `vnc server running on ::1:5900`，则说明 QEMU 的图形界面被启用并通过端口 5900 输出。

    你可以考虑使用 `VNC Viewer` 来观察 QEMU 界面。

2. 思考题：观察实验代码，串口驱动是在进入内核后启用的，那么在驱动加载前，显示的内容是如何输出的？请给出你的回答。

3. 编程题：请实现 `uart16550` 模块的 `pub fn send(&mut self, data: u8)` 函数，完成对串口的输出。

### 日志输出

1. 编程题：TODO：**我的构思是给一个超简化的输出版本，让他们自己实现一个日志完整的 log 系统，而 UART 部分只需要填空感受过程就好。**

## 思考题

1. 在 `pkg/kernel` 的 `Cargo.toml` 中，指定了依赖中 `boot` 包为 `default-features = false`，这是为了避免什么问题？请结合 `pkg/boot` 的 `Cargo.toml` 谈谈你的理解。
2. 在 `pkg/boot/src/main.rs` 中参考相关代码，聊聊 `max_phys_addr` 是如何计算的，为什么要这么做？

## 加分项

1. 🤔 “开发者是愿意用安全换取灵活的”，所以，我要把代码加载到栈上去，可当我妄图在栈上执行代码的时候，却得到了 `Segment fault`，你能解决这个问题吗？

    请尝试利用 `gcc` 在 Linux 平台上编译一个简单的 C 语言程序，将其编译为 ELF 格式的文件，并尝试在栈上执行它，使它输出 `Hello, world!`。

    !!! question "通过了解 ELF 文件格式、编译链接、内存分页等知识，尝试解决这个问题。"
