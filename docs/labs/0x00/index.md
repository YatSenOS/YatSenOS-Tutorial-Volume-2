# 实验零：环境搭建与实验准备

!!! tip "代码是一场无声的交流 <br/>有些人是优秀的诗人，能够将抽象的想法转化为优雅的语言 <br/>而有些人则是忠实的翻译者，将逻辑转换成计算机可理解的语言。"

    <p align="right" style="font-weight: bold">by ChatGPT</p>

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

对于本次实验内容，你需要参考学习如下实验资料：

- [Linux 使用指导](../../wiki/linux.md)
- [Rust 语言学习](../../wiki/rust.md)
- [UEFI 启动过程](../../wiki/uefi.md)
- [QEMU 使用参考](../../wiki/qemu.md)

## 实验任务与要求

1. 请各位同学独立完成作业，任何抄袭行为都将使本次作业判为 0 分。
2. 请参考 [代码规范](../../general/coding_convention.md) 进行实验代码编写。
3. 依据 [实验任务](./tasks.md) 完成实验。
