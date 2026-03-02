# Linux 环境配置

Linux 是一个开源的类 Unix 操作系统内核，它是一个典型的多用户、多任务的操作系统，可以运行在各种平台上，如服务器、PC、手机等。常见的 Linux 发行版有 Ubuntu、Debian、Arch、Kali 等。

与常规的 GUI 交互方式不同，Linux 系统通常使用命令行来与用户进行交互，命令行是一种通过键入命令来完成与计算机的交互的方式，它可以让用户完成一个操作系统所能提供的一切功能。

本次操作系统实验的最终目标也是实现一个能够和用户进行命令行交互的操作系统，因此建议你多尝试使用命令行来完成实验。

环境的准备工作主要包括：
- 安装 Linux 系统
- 需要安装和配置开发环境 (工具链)
    - Make: 构建脚本
    - Python: 构建脚本
    - QEMU: 提供虚拟硬件运行我们编写的 OS
    - gdb: 调试工具
    - gcc: Rust 开发环境的前置条件
    - Rust 开发环境及工具链
- 检查工具版本，测试能否正常构建和调试

!!! tip "如果如下内容不足以让你完全配置好开发环境，可以参考 THU Rust 课程的 [环境配置](https://lab.cs.tsinghua.edu.cn/rust/environment/)"

## 安装 Linux 系统

Linux 有许多发行版，这里出于环境一致性考虑，推荐使用 Ubuntu 24.04 或 **麒麟 Linux Advanced Server V11（Kylin OS）**。

其他发行版（如 Debian，Arch，Kali）也可以满足实验需求，但**请注意内核版本、QEMU 版本都不应低于本次实验的参考标准**。

!!! info "Kylin OS"

    Kylin OS (银河麒麟高级服务器操作系统) 最初由国防科技大学主导研发，在中国 Linux 市场保持多年占有率第一名，广泛用于国内政府、企业和高校服务器环境，本实验对其提供完整支持。

    麒麟 Linux Advanced Server V11 既支持 x86-64，也支持 **aarch64（ARM64）** 架构。对于 aarch64 平台的麒麟 OS，通过 `qemu-system-x86_64` 进行**跨架构模拟**，即可运行本实验构建的 x86-64 操作系统镜像。

### 使用 WSL2

对于 Windows 10/11 的用户来说，可以使用 WSL（Windows Subsystem Linux）来安装 Linux 系统，WSL 意为面向 Windows 的 Linux 子系统，微软为其提供了很多特性方便我们使用，我们可以在 Windows 上运行 Linux 程序。

你可以使用如下指令在 Windows 上安装 WSL2：

```bash
wsl --install -d Ubuntu
```

上述指令将会安装 WSL2 的全部依赖，并下载 Ubuntu 作为默认的发行版本。在安装过程中可能会重启电脑，安装完成后，你可以在 Windows 的应用列表中找到 Ubuntu，点击运行即可。

关于其他的配置，可以在网上找到大量的参考资料，请自行搜索阅读，或寻求 LLM 的帮助。

### 使用其他虚拟机软件

如果你不想使用 WSL2，也可以使用其他虚拟机软件，如 VMware Workstation、VirtualBox 等，安装 Ubuntu 的最新发行版，相关安装教程请自行搜索。

!!! warning "使用须知"

    请注意，你需要自行处理如下问题，以达到与 WSL 2 类似的能力：

    - 与 Windows 之间的剪贴板共享（需要安装 VMware Tools 等辅助工具和 Guest 侧驱动）
    - 与 Windows 之间的文件共享 (需要配置共享文件夹，或者使用网络共享协议)

    如果有需要在 Windows 上使用 SSH 连接到虚拟机，你需要在虚拟机中安装 SSH 服务，并配置网络连接。

    **以上内容都需要你具有一定的 Windows 和 Linux 系统的使用经验，如果你不确定自己是否能够完成这些操作，请使用 WSL 2。**

### 使用实体机

如果你已经拥有了一台 Linux 服务器或者台式机，笔者相信你的折腾能力。

你可以使用任何你喜欢的发行版，但请注意内核版本、QEMU 版本都不应低于实验的参考标准。

## 使用系统包管理器安装依赖

=== "Kylin OS"

    麒麟 OS 使用基于 RPM 体系的 `yum` / `dnf` 作为包管理器，二者本质上都是 `dnf-3` 的软链接，所以选择用哪一个没有区别，但更推荐使用现代化的 `dnf` 而不是为了兼容旧版本所保留的 `yum`。

    ```bash
    # 更新软件包缓存
    sudo dnf makecache

    # 安装或更新
    sudo dnf install -y \
        gcc \
        gdb \
        make \
        mtools \
        python \
        qemu-system-x86_64
    ```

=== "Ubuntu"

    Ubuntu 属于 Debian 系，使用 `apt` 作为包管理器。为了保障软件源的正常、快速访问，请参考 [Ubuntu 软件仓库镜像使用帮助](https://help.mirrors.cernet.edu.cn/ubuntu/) 提供的文档进行软件源更换。使用 [中山大学开源软件镜像加速服务](https://mirror.sysu.edu.cn) 可以在校内享受到更好的下载速度。

    ```bash
    # 更新 apt 源
    sudo apt update && sudo apt upgrade

    # 安装或更新
    sudo apt install -y \
        build-essential \
        gdb \
        qemu-system-x86
    ```

在安装完成后，请使用如下命令，确保你的相关软件包**不低于**如下标准：

=== "Kylin OS v11 (Swan25)"

    这里使用 Kylin Linux Advanced Server V11 (Swan25) x86-64 提供参考值。

    ```bash
    $ qemu-system-x86_64 --version
    QEMU emulator version 8.2.0 (qemu-8.2.0-27.p11.ky11)
    
    $ gcc --version
    gcc (GCC) 12.3.1 (kylin 12.3.1-62.p03.ky11)
    
    $ gdb --version
    GNU gdb (GDB) KylinOS 14.1-2.p03.ky11
    ```

=== "Ubuntu 24.04"

    ```bash
    $ qemu-system-x86_64 --version
    QEMU emulator version 8.2.2 (Debian 1:8.2.2+ds-0ubuntu1.4)

    $ gcc --version
    gcc (Ubuntu 13.2.0-23ubuntu4) 13.2.0

    $ gdb --version
    GNU gdb (Ubuntu 15.0.50.20240403-0ubuntu1) 15.0.50.20240403-git
    ```

=== "Ubuntu 22.04"

    ```bash
    $ qemu-system-x86_64 --version
    QEMU emulator version 6.2.0 (Debian 1:6.2+dfsg-2ubuntu6.15)

    $ gcc --version
    gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0

    $ gdb --version
    GNU gdb (Ubuntu 12.1-0ubuntu1~22.04) 12.1
    ```

!!! warning "已知问题：aarch64 上的 GDB 调试"

    在 aarch64 平台上，`gdb` 无法直接调试 x86-64 程序。如需调试，需要安装 `gdb-multiarch`：

    ```bash
    # dnf 安装
    sudo dnf install gdb-multiarch

    # apt 安装
    sudo apt install gdb-multiarch

    # 使用
    gdb-multiarch target/x86_64-unknown-uefi/debug/...
    ```

    参考 [实验调试指南](./debug.md) 使用 QEMU 的 GDB stub 进行远程调试。

## 安装 Rust 开发环境与工具链

1. 安装 rustup：

    !!! note "rustup 安装过程中存在一些可配置选项，请按照默认选项进行安装。"

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
    ```

    !!! tip "如果遇到了网络问题，请参考 [rsproxy.cn](https://rsproxy.cn/) 进行配置。"

2. 按照仓库中的 `rust-toolchain` 文件，安装对应的 Rust 工具链。

    !!! tip "第一次在拥有 `rust-toolchain.toml` 文件的目录下运行 `cargo` 命令时，Rustup 会自动安装对应的工具链。"

3. 检查编译器版本**不低于**如下标准：

    ```bash
    $ rustc --version
    rustc 1.93.1 (01f6ddf75 2026-02-11)

    $ rustc +nightly --version
    rustc 1.96.0-nightly (38c0de8dc 2026-02-28)
    ```

## 验证工作

在安装完成之后，你应该能正常使用 `make` 和 `python` 命令，并用它们来运行项目的编译、运行脚本。

```bash
cd src/0x00
python3 ysos.py build   # 或 make build
```

!!! success "预期成功输出"

    如果能够看到 `cargo build` 的编译输出，并在 `esp/` 目录下生成 `.efi` 文件，说明环境配置成功。

更完整地验证能否编译、运行、调试内核将在[实验一：操作系统的启动](../labs/0x01/tasks.md)中展开。

## 参考资料

你可以通过下面的一些链接来对命令行的使用进行学习，也可以把它们作为参考文档随用随取：

1. [The Missing Semester](https://missing-semester-cn.github.io/2020/shell-tools)
2. [UNIX basics tutorial](https://berkeley-scf.github.io/tutorial-unix-basics/)
3. [GNU/Linux Command-Line Tools Summary](https://tldp.org/LDP/GNU-Linux-Tools-Summary/html/index.html)
4. [「实用技能拾遗」课程 S1 by TonyCrane](https://slides.tonycrane.cc/PracticalSkillsTutorial/2023-spring-cs/#/)
5. [「实用技能拾遗」课程 S2 by TonyCrane](https://slides.tonycrane.cc/PracticalSkillsTutorial/2023-fall-ckc/#/)
