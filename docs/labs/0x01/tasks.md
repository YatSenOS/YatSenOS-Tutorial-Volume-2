# 实验一：操作系统的启动

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 编译内核 ELF

与常规实验中直接将内核编译为二进制文件不同，而本实验需要将内核编译为 ELF 格式的文件，并将它存储在 UEFI 可以访问的文件系统中。

!!! note 请阅读 [ELF 文件格式](../../wiki/elf.md) 部分，了解什么是 ELF 文件。

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

值得注意的是，为了后续实验代码编写的便利，我们将内核的代码段、数据段、BSS 段等都设置为了 4KB 对齐。

请尝试在 `pkg/kernel` 目录下运行 `cargo build --release`，之后找到编译产物，并使用 `readelf` 命令查看其基本信息，回答以下问题：

// TODO: some questions

## 在 UEFI 中加载内核

<!-- TODO -->

## UART 与日志输出

### 串口

请阅读[串口输出简介](../../wiki/uart.md)一节，完成以下问题。

1. 思考题：在 QEMU 中，我们通过指定 `-nographic` 参数来禁用图形界面，这样 QEMU 会默认将串口输出重定向到主机的标准输出。假如我们将 `Makefile` 中取消该选项，QEMU 的输出窗口会发生什么变化？请观察指令 `make QEMU_OUTPUT= run` 的输出，结合截图分析对应现象。

!!! tip "现象观察提示"
    若此时启动 QEMU 的输出提示是 `vnc server running on ::1:5900`，则说明 QEMU 的图形界面被启用并通过端口5900输出。

    你可以考虑使用 `VNC Viewer` 来观察 QEMU 界面。

2. 思考题：观察实验代码，串口驱动是在进入内核后启用的，那么在驱动加载前，显示的内容是如何输出的？请给出你的回答。

3. 编程题：请实现 `uart16550` 模块的 `pub fn send(&mut self, data: u8)` 函数，完成对串口的输出。


### 日志输出

1. 编程题：TODO：**我的构思是给一个超简化的输出版本，让他们自己实现一个日志完整的 log 系统，而 UART 部分只需要填空感受过程就好。**

## 尝试搭建调试环境

### 基本调试环境搭建(必做)

依据[调试教程](../../wiki/debug.md)的**基本方法：GDB **提示，搭建基于命令行的 GDB 调试环境。

1. 对关键步骤截图，并简单解释你是怎么做的。

2. 回答问题：请解释指令 `layout asm` 的功能。倘若我想找到当前运行内核所对应的 Rust 源码，我该使用什么 GDB 指令？请给出你的回答与对应的截图。

3. 回答问题：假如在编译时没有启用 `DBG_INFO=true`，调试过程会有什么不同？请给出你的回答与对应的截图。

### 进阶调试方法尝试

阅读[调试教程](../../wiki/debug.md)的剩余部分，回答以下问题。

!!! note "任务说明"
    你仅需选择 pwndbg 调试 **或** VSCode 调试的问题来回答。

    我们更推荐尝试 pwndbg 调试，在调试疑难杂症时，往往是汇编和寄存器的蛛丝马迹提示你 Debug。

    VSCode 调试作为 Bonus 加分。

1. pwndbg 调试：

   1. 请简要介绍你的 pwndbg 配置过程，并给出必要的截图。
   2. 请截图介绍 pwndbg 调试页面的主要功能，并简要说明其作用。
   3. 请列出你认为**5条**最有用的 pwndbg调试指令，并简要说明其作用。

2. VSCode 调试：
    1. 请简要介绍你的 VSCode 配置过程，并给出必要的截图。
    2. 请截图介绍 VSCode 调试页面的主要功能，并简要说明其作用。
    3. 请解释 `./.vscode/launch.json` 中的各字段的作用，按需添加你的指令。
