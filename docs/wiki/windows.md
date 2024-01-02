# Windows 环境配置

!!! note "推荐使用 WSL2 来进行实验"

    虽然在 Windows 上安装项目开发环境是**完全可行的**，但是由于 GDB 在 Windows 上的支持并不能提供最好的体验，因此可能会影响调试效率。

    当前，调试效率因人而异，在大多数时候输出 + QEMU Shell 调试已经足够满足实验需求。

    除此之外，通过配置 VSCode 的方式得到的调试体验通常是一致的，因此这也是一个可选项。

## 安装 rustup

请前往 [rustup 官网](https://rustup.rs/) 下载并运行 `rustup-init.exe`，安装选项请阅读下列内容：

如果想要变更安装路径，可以通过指定如下环境变量来实现：

- `RUSTUP_HOME`：rustup 的安装路径
- `CARGO_HOME`：cargo 的安装路径

rustup 将会把 `CARO_HOME\bin` 添加到 `PATH` 环境变量中。

### 工具链的选择

rust 提供了两种 windows 上的工具链：`msvc` 和 `gnu`，详细信息可以参考 [Windows - The rustup book](https://rust-lang.github.io/rustup/installation/windows.html)。

- `msvc`：使用 Visual Studio 的 C++ 编译器，需要安装 Visual Studio。

    在安装 Visual Studio 时，需要选择如下组件：

    - `MSVC v143 - VS 2022 C++ x64/x86 build tools (latest)`
    - `Windows 11 SDK (10.0.22621.0)`

    `msvc` 工具链可以提供更好的 Windows 应用兼容性，也是 Windows 上开发 rust 应用推荐的工具链。

    二者总空间需求约为 4GB。若要安装 Visual Studio 本体，可能需要更多的空间。

    此工具链本体占用约为 600MB。推荐本来就有 Visual Studio 的同学使用此工具链。

- `gnu`：使用 GNU 工具链，需要安装 MinGW，会随工具链一起自动安装。

    此工具链总体占用约 1GB。推荐未安装 Visual Studio 的同学使用此工具链。

    **你需要在安装时候进行选择，输入 `stable-x86_64-pc-windows-gnu`**

    在安装后，也需要切换工具链，可以通过如下指令来实现：

    ```bash
    rustup set default-host x86_64-pc-windows-gnu
    ```

其余安装选择保持默认即可。

对于实验来说，编译产物均为 ELF 文件，且不涉及 Windows 相关 SDK。因此二者均可完美达成实验要求，具体安装情况可根据自身情况选择。

## 安装 QEMU

你可以通过 `winget` 来安装 QEMU：

```bash
winget install SoftwareFreedomConservancy.QEMU
```

QEMU 的全部二进制将被安装在 `C:\Program Files\qemu` 下。

如果你通过其他方式安装了 QEMU，你需要将 QEMU 的安装路径添加到 `PATH` 环境变量中。

!!! note "`ysos.py` 将会自动查找此路径，如果你的 QEMU 安装在其他路径下，请将对应路径添加到 `PATH` 环境变量中"

## 安装 Python

!!! note "如果你已经安装了 Python 3，并且在 Shell 中可以通过 `python` 命令来调用，可以跳过此步骤"

Python 的安装多种多样，同时由于包管理的混乱，也有很多虚拟环境管理器。

推荐通过 `winget` 来安装 Python 和 `mamba`，`mamba` 是一个基于 `conda` 的包管理器，比原生的 `conda` 解析速度更快、效果更好。

```bash
winget info CondaForge.Miniforge3
```

安装后，打开开始菜单中的 `Miniforge Prompt`，输入如下命令来初始化 PowerShell：

```bash
conda init powershell
```

然后，关闭当前 PowerShell 窗口，重新打开一个新的 PowerShell 窗口，检查是否安装成功：

```bash
mamba --version
```

!!! tip "不要在 `base` 环境下安装额外的包"

    为了保证基础环境的整洁、稳定，我们不推荐在 `base` 环境下安装额外的包。

    可以选择一个你喜欢的路径（可以是其他驱动器），创建一个新的环境，然后在新的环境中安装额外的包。这也是推荐的做法。

    ```bash
    mamba create -p "E:\Environment\miniforge\envs\main" python=3.11
    ```

    这里的 `E:\Environment\miniforge\main` 只是一个示例，你可以根据自己的喜好来选择路径。

    在成功安装新的环境之后，你需要修改用户目录下的 `.condarc` 文件，将 `envs_dirs` 的值修改为你新建的环境的路径。

    下列是一个示例：

    ```yaml
    changeps1: false
    envs_dirs:
      - E:\Environment\miniforge\envs
      - C:\Users\<USER>\miniforge3\envs
    pkgs_dirs:
      - E:\Environment\miniforge\pkgs
      - C:\Users\<USER>\miniforge3\envs\pkgs
    solver: libmamba
    channel_priority: flexible
    ```

    之后，如果你使用的是 PowerShell，则可以选择配置 `$PROFILE` 来自动激活环境：

    ```bash
    notepad $PROFILE
    ```

    在打开的文件中，添加如下内容：

    ```bash
    conda activate main
    ```

在安装好 Python 之后，你可以尝试运行 `ysos.py`，或者用 Python 做其他的事情。

## 验证工作

在配置好环境之后，你应当能够通过 `python ysos.py -h` 来查看帮助信息，并完成实验所需的全部任务。

有关 GDB 和 VSCode 的安装配置、使用方式请自行搜索学习。
