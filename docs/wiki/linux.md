# Linux 环境配置

Linux 是一个开源的类 Unix 操作系统内核，它是一个典型的多用户、多任务的操作系统，可以运行在各种平台上，如服务器、PC、手机等。常见的 Linux 发行版有 Ubuntu、Debian、Arch、Kali 等。

与常规的 GUI 交互方式不同，Linux 系统通常使用命令行来与用户进行交互，命令行是一种通过键入命令来完成与计算机的交互的方式，它可以让用户完成一个操作系统所能提供的一切功能。

本次操作系统实验的最终目标也是实现一个能够和用户进行命令行交互的操作系统，因此建议你多尝试使用命令行来完成实验。

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

### 使用 VMware Workstation

参考 [VMware Workstation 安装 Ubuntu 22.04 LTS](https://zhuanlan.zhihu.com/p/569274366) 教程。

## 安装项目开发环境

在正确安装 Linux 系统后，需要安装和配置开发环境，包括 gcc, make, python, qemu 等。

为了保障 Linux 软件源的正常、快速访问，请参考 [Ubuntu 软件仓库镜像使用帮助](https://help.mirrors.cernet.edu.cn/ubuntu/) 提供的文档进行软件源更换。

!!! note "校内镜像源"

    我们还提供有**仅供校内、不确保一定可用**的内网镜像源：[Matrix 软件源镜像服务](https://mirrors.matrix.moe)

    请注意，使用上述镜像站会让你享受到更好的下载速度，但你同时也需要**承担不可用时带来的风险，并具有自主更换到其他镜像站的能力**。

1. 使用以下命令更新 apt 源并进行软件升级：

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
rustc 1.77.0-nightly (11f32b73e 2024-01-31)

$ qemu-system-x86_64 --version
QEMU emulator version 6.2.0 (Debian 1:6.2+dfsg-2ubuntu6.15)

$ gcc --version
gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0

$ gdb --version
GNU gdb (Ubuntu 12.1-0ubuntu1~22.04) 12.1
```

!!! tip "如果上述内容不足以让你完全配置好开发环境，可以参考 THU Rust 课程的 [环境配置](https://lab.cs.tsinghua.edu.cn/rust/environment/)"

## 验证工作

在安装完成之后，你应该能正常使用 `make` 和 `python` 命令，并用它们来运行项目的编译、运行脚本。

## 参考资料

你可以通过下面的一些链接来对命令行的使用进行学习，也可以把它们作为参考文档随用随取：

1. [The Missing Semester](https://missing-semester-cn.github.io/2020/shell-tools)
2. [UNIX basics tutorial](https://berkeley-scf.github.io/tutorial-unix-basics/)
3. [GNU/Linux Command-Line Tools Summary](https://tldp.org/LDP/GNU-Linux-Tools-Summary/html/index.html)
4. [「实用技能拾遗」课程 S1 by TonyCrane](https://slides.tonycrane.cc/PracticalSkillsTutorial/2023-spring-cs/#/)
5. [「实用技能拾遗」课程 S2 by TonyCrane](https://slides.tonycrane.cc/PracticalSkillsTutorial/2023-fall-ckc/#/)
