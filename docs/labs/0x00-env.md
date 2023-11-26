# 实验零：环境搭建与实验准备

1. 安装 Linux 系统虚拟机或者 WSL（Windows Subsystem Linux）
2. 安装 OS 运行环境
3. 学习基础 Rust 语法以及 x86 汇编语法

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

之后打开 Microsoft Store，搜索 Ubuntu，选择 Ubuntu 22.04 LTS，点击获取即可。 或者运行：
```
wsl --install -d Ubuntu-22.04
```

### 使用 Vmware Workstation 安装 Linux

参考 [Vmware Workstation 安装 Ubuntu 22.04 LTS](https://zhuanlan.zhihu.com/p/569274366) 教程。

## 安装 OS 运行环境

在安装好 Linux 系统后，需要安装开发环境，包括 gcc, make, qemu 等。

首先将 apt 源更改为 Matrix 源（使用校内的镜像站速度更快），编辑 `/etc/apt/sources.list` 文件，将其中的内容替换为以下内容：

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