# 实验四：用户程序与系统调用

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 合并实验代码

!!! tip "如何使用本次参考代码"

    本次给出的参考代码为**增量补充**，即在上一次实验的基础上进行修改和补充。因此，你需要将本次参考代码与上一次实验的代码进行合并。

    文件的目录与上一次实验相同，因此你可以直接将本次参考代码的 `src` 目录下的文件复制到上一次实验的目录结构下，覆盖同名文件。

    合并后的代码并不能直接运行，你需要基于合并后的代码、按照文档进行修改补充，才能逐步实现本次实验的功能。

!!! warning "项目组织说明"

    1. 本次实验中给出的均为**参考代码片段**，所有**代码内容、代码结构**均可按照需要自行调整。
    2. 部分代码的使用**需要自行补全 `lib.rs` 和 `mod.rs` 等文件**，你也可以**添加、修改任何所需函数**。
    3. 功能在逐步实现的过程中，部分未使用代码可以进行注释以通过编译检查。

在 `pkg/app` 中，定义提供了一些用户程序，这些程序将会在编译后提供给内核加载运行。

在 `pkg/syscall` 中，提供系统调用号和调用约束的定义，将会在内核和用户库中使用，在下文中会详细介绍。

在 `pkg/lib` 中，定义了用户态库并提供了一些基础实现，相关内容在下文中会详细介绍。

在 `pkg/kernel` 中，添加了如下一些模块：

- `interrupt/syscall`：定义系统调用及其服务的实现。
- `memory/user`：用户堆内存分配的实现，会被用在系统调用的处理中，将用户态的内存分配委托给内核。
- `utils/resource`：定义了用于进行 I/O 操作的 `Resource` 结构体，用于处理用户态的读写系统调用。

!!! note "别忘了更新 `Cargo.toml`"

## 用户程序

### 编译用户程序

对于不同的运行环境，即使指令集相同，一个可执行的程序仍然有一定的差异。

与内核的编译类似，在 `pkg/app/config` 中，定义了用户程序的编译目标，并定义了相关的 LD 链接脚本。

在 `Cargo.toml` 中，使用通配符引用了 `pkg/app` 中的所有用户程序。相关的编译过程在先前给出的编译脚本中均已定义，可以直接编译。

通常而言，用户程序并不直接自行处理系统调用，而是由用户态库提供的函数进行调用。

在编写 C 语言时 `printf`、`scanf` 等函数并不是直接调用系统调用，以 gcc on Linux 的一般行为为例，这些函数被定义在 `glibc` 中，而 `glibc` 会处理系统调用。相对应的，在 Windows 上，也会存在 `msvcrt` (Microsoft Visual C Run-time) 等库。

为了让用户态程序更好地与 YSOS 进行交互，处理程序的生命周期，便于编写用户程序等，需要提供用户态库，以便用户程序调用。

用户态库被定义在 `pkg/lib` 中，在用户程序中，编辑 `Cargo.toml`，使用如下方式引用用户库：

```rust
[dependencies]
lib = { path="../../lib", package="yslib"}
```

一个简单的用户程序示例如下所示，同样存在于 `app/hello/src/main.rs` 中：

```rust
#![no_std]
#![no_main]

use lib::*;

extern crate lib;

fn main() -> usize {
    println!("Hello, world!!!");

    233
}

entry!(main);
```

- `#![no_std]` 表示不使用标准库，rust 并没有支持 YSOS 的标准库，需要我们自行实现。
- `#![no_main]` 表示不使用标准的 `main` 函数入口，而是使用 `entry!` 宏定义的入口函数。

`entry!` 宏的定义如下：

```rust
#[macro_export]
macro_rules! entry {
    ($fn:ident) => {
        #[export_name = "_start"]
        pub extern "C" fn __impl_start() {
            let ret = $fn();
            // FIXME: after syscall, add lib::sys_exit(ret);
            loop {}
        }
    };
}
```

在 `__impl_start` 函数中，调用用户程序的 `main` 函数，并在用户程序退出后，进入死循环。在后续完善了进程退出的系统调用后，你需要将 `FIXME` 的部分替换为正确的系统调用。

!!! note "关于 `libc` 的处理"

    在 Linux 中，一个正常的用户程序在编译后也不会直接执行 `main` 函数，而是执行 `_start` 函数，这个函数会通过调用 `__libc_start_main`，最终通过 `__libc_stop_main`、`__exit` 等一系列函数，准备好应用程序需要执行的环境，并在程序退出后进行一些后续的工作。

!!! note "阶段性目标"

    在一切配置顺利之后，应当可以使用 `cargo build` 在用户程序目录中正确地编译用户程序。

### 加载程序文件

在成功编译了用户程序后，用户程序将被脚本移动到 `esp/APP` 目录下，并**以文件夹命名**。

> 由于 FAT16 文件系统的限制，文件名长度不能超过 8 个字符，所以建议使用简短的文件夹名。

目前的内核尚不具备访问磁盘和文件系统，并将它们读取加载的能力（将会在实验六中实现），因此需要另辟蹊径：在 bootloader 中将符合条件的用户程序加载到内存中，并将它们交给内核，用于生成用户进程。

!!! note "修改内核配置文件"

    这就是 lab 1 中 `Config` 含有 `load_apps` 的原因。

    本次实验你应当在 `pkg/kernel/config/boot.conf` 中，将 `load_apps` 设置为 `true`。

为了存储用户程序的相关信息，在 bootloader 中，定义一个 `App` 结构体，并添加“已加载的用户程序”字段到 `BootInfo` 结构体中：

```rust
use arrayvec::{ArrayString, ArrayVec};

/// App information
pub struct App<'a> {
    /// The name of app
    pub name: ArrayString<16>,
    /// The ELF file
    pub elf: ElfFile<'a>,
}

pub type AppList = ArrayVec<App<'static>, 16>;

/// This structure represents the information that the bootloader passes to the kernel.
pub struct BootInfo {
    // ...
    // Loaded apps
    pub loaded_apps: Option<AppList>,
}
```

!!! tip "更好的类型声明？"

    - 使用 `const` 指定用户程序数组的最大长度。
    - 尝试定义 `AppListRef` 类型，用于存储 `loaded_apps.as_ref()` 的返回值类型。
    - 抛弃 `App` 类型的生命周期，直接声明 `ElfFile<'static>`。

之后，在 `pkg/boot/src/fs.rs` 中，创建函数 `load_apps` 用于加载用户程序，并参考 `fs.rs` 中的其他函数，处理文件系统相关逻辑，补全代码：

```rust
/// Load apps into memory, when no fs implemented in kernel
///
/// List all file under "APP" and load them.
pub fn load_apps(bs: &BootServices) -> AppList {
    let mut root = open_root(bs);
    let mut buf = [0; 8];
    let cstr_path = uefi::CStr16::from_str_with_buf("\\APP\\", &mut buf).unwrap();

    let mut handle = { /* FIXME: get handle for \APP\ dir */ };

    let mut apps = ArrayVec::new();
    let mut entry_buf = [0u8; 0x100];

    loop {
        let info = handle
            .read_entry(&mut entry_buf)
            .expect("Failed to read entry");

        match info {
            Some(entry) => {
                let file = { /* FIXME: get handle for app binary file */ };

                if file.is_directory().unwrap_or(true) {
                    continue;
                }

                let elf = {
                    // FIXME: load file with `load_file` function
                    // FIXME: convert file to `ElfFile`
                };

                let mut name = ArrayString::<16>::new();
                entry.file_name().as_str_in_buf(&mut name).unwrap();

                apps.push(App { name, elf });
            }
            None => break,
        }
    }

    info!("Loaded {} apps", apps.len());

    apps
}
```

在 `boot/src/main.rs` 中，`main` 函数中加载好内核的 `ElfFile` 之后，根据配置选项按需加载用户程序，并将其信息传递给内核：

```rust
// ...

let apps = if config.load_apps {
    info!("Loading apps...");
    Some(load_apps(system_table.boot_services()))
} else {
    info!("Skip loading apps");
    None
};

// ...

// construct BootInfo
let bootinfo = BootInfo {
    // ...
    loaded_apps: apps,
};
```

修改 `ProcessManager` 的定义与初始化逻辑，将 `AppList` 添加到 `ProcessManager` 中：

```rust
pub struct ProcessManager {
    // ...
    app_list: boot::AppListRef,
}
```

最后修改 `kernel/src/proc/mod.rs` 的 `init` 函数：

```rust
/// init process manager
pub fn init(boot_info: &'static boot::BootInfo) {
    // ...
    let app_list = boot_info.loaded_apps.as_ref();
    manager::init(kproc, app_list);
}
```

之后，在 `kernel/src/proc/mod.rs` 中，定义一个 `list_app` 函数，用于列出当前系统中的所有用户程序和相关信息：

```rust
pub fn list_app() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let app_list = get_process_manager().app_list();
        if app_list.is_none() {
            println!("[!] No app found in list!");
            return;
        }

        let apps = app_list
            .unwrap()
            .iter()
            .map(|app| app.name.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        // TODO: print more information like size, entry point, etc.

        println!("[+] App list: {}", apps);
    });
}
```

!!! note "阶段性目标"

    在 `kernel/src/main.rs` 初始化内核之后，尝试调用 `list_app` 函数，查看是否成功加载。

### 生成用户程序

在 `kernel/src/proc/mod.rs` 中，添加 `spawn` 和 `elf_spawn` 函数，将 ELF 文件从列表中取出，并生成用户程序：

```rust
pub fn spawn(name: &str) -> Option<ProcessId> {
    let app = x86_64::instructions::interrupts::without_interrupts(|| {
        let app_list = get_process_manager().app_list()?;
        app_list.iter().find(|&app| app.name.eq(name))
    })?;

    elf_spawn(name.to_string(), &app.unwrap().elf)
}

pub fn elf_spawn(name: String, elf: &ElfFile) -> Option<ProcessId> {
    let pid = x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        let process_name = name.to_lowercase();
        let parent = Arc::downgrade(&manager.current());
        let pid = manager.spawn(elf, name, Some(parent), None);

        debug!("Spawned process: {}#{}", process_name, pid);
        pid
    });

    Some(pid)
}
```

??? question "为什么独立一个 `elf_spawn`？"

    在后续的实验中，`spawn` 将接收一个文件路径，操作系统需要从文件系统中读取文件，并将其加载到内存中。

    通过将 `elf_spawn` 独立出来，可以在后续实验中直接对接到文件系统的读取结果，而无需修改后续代码。

**删除或注释上次实验中有关内核线程的代码**，防止后续修改后的进程模型在执行内核线程时遇到意外的问题。

在 `ProcessManager` 中，实现 `spawn` 函数：

```rust
pub fn spawn(
    &self,
    elf: &ElfFile,
    name: String,
    parent: Option<Weak<Process>>,
    proc_data: Option<ProcessData>,
) -> ProcessId {
    let kproc = self.get_proc(&KERNEL_PID).unwrap();
    let page_table = kproc.read().clont_page_table();
    let proc = Process::new(name, parent, page_table, proc_data);
    let pid = proc.pid();

    let mut inner = proc.write();
    // FIXME: load elf to process pagetable
    // FIXME: alloc new stack for process
    // FIXME: mark process as ready
    drop(inner);

    trace!("New {:#?}", &proc);

    // FIXME: something like kernel thread
    pid
}
```

在加载的过程中，你可以复用 `elf::load_elf` 函数。可以通过为它添加参数的方式，控制这一映射的内容是否可以被用户权限访问。

```rust
pub fn load_elf(
    elf: &ElfFile,
    physical_offset: u64,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    user_access: bool
) -> /* return type */ {
    // ...
}
```

在映射页面时，根据此参数决定是否添加 `USER_ACCESSIBLE` 标识位：

```rust
if user_access {
    page_table_flags |= PageTableFlags::USER_ACCESSIBLE;
}
```

!!! note "**对于用户进程而言，不再与内核共享页表，而是通过克隆内核页表获取了自己的页表。这意味着可以为每个用户进程分配同样的栈地址，而不会相互干扰。**"

!!! tip "一些提示"

    - 与内核类似，使用 `elf.header.pt2.entry_point()` 获取 ELF 文件的入口地址。
    - 或许可以在 `ProcesssInner` 中实现一个 `load_elf` 函数，来处理代码段映射等内容。
    - 记得为进程分配好合适的栈空间，并使用 `init_stack_frame` 初始化栈帧。
    - 或许你可以同时实现 **加分项 1** 所描述的功能。

!!! note "阶段性目标？"

    但由于并没有实现任何系统调用服务（包括程序的退出、输入输出、内存分配等），因此你在加载用户程序后，基本无法进行任何操作。

    在这一阶段，为了不触发异常，你只能加载执行一个没有其他作用的死循环程序……

    因此，你需要在完成后续任务后，才能进行整体性调试。

## 系统调用的实现

为了为用户程序提供服务，操作系统需要实现一系列的系统调用，从而为用户态程序提供内核态服务。这些操作包括文件操作、进程操作、内存操作等，相关的指令一般需要更高的权限（相对于用户程序）才能执行。

### 调用约定

系统调用一般有系统调用号、参数、返回值等调用约定，不同的上下文参数对应的系统调用的行为存在不同。

以 x86_64 的 Linux 为例，系统调用的部分调用约定如下所示：

- 系统调用号通过 `rax` 寄存器传递
- 参数通过 `rdi`、`rsi`、`rdx`、`r10`、`r8`、`r9` 寄存器传递
- 参数数量大于 6 时，通过栈传递
- 返回值通过 `rax` 寄存器传递

!!! tip "在系统调用中，由于 `rcx` 寄存器有其他用途，因此使用 `r10` 寄存器代替函数调用约定 `__fastcall` 中的 `rcx` 寄存器。"

实验实现中并不需要那么多的系统调用参数，在 `src/interrupt/syscall/mod.rs` 中定义了一个用于存储系统调用参数的结构体 `SyscallArgs`：

```rust
#[derive(Clone, Debug)]
pub struct SyscallArgs {
    pub syscall: Syscall,
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
}
```

之后在分发器函数 `dispatcher` 中，构造 `SyscallArgs` 结构体，尊重习惯性写法，使用 `rdi` / `rsi` / `rdx` 寄存器传递系统调用参数，之后调用相应的处理函数。

```rust
pub fn dispatcher(context: &mut ProcessContext) {
    let args = super::syscall::SyscallArgs::new(
        Syscall::try_from(context.regs.rax).unwrap_or_default(),
        context.regs.rdi,
        context.regs.rsi,
        context.regs.rdx,
    );

    match args.syscall {
        // ...
    }
}
```

为了能在用户库（调用系统调用侧）和内核态（处理系统调用侧）之间达成系统调用号的一致性，在 `pkg/syscall` 中定义了一个 `Syscall` 枚举，用于存储系统调用号。

```rust
#[repr(usize)]
#[derive(Clone, Debug, TryFromPrimitive)]
pub enum Syscall {
    Read = 0,
    Write = 1,

    Spawn = 59,
    Exit = 60,
    WaitPid = 61,

    ListApp = 65531,
    Stat = 65532,
    Allocate = 65533,
    Deallocate = 65534,

    #[num_enum(default)]
    Unknown = 65535,
}
```

由于一些额外的执念，这里的读写、进程操作的系统调用号基本与 Linux 中功能类似的系统调用号一致，而有些系统调用号则是自定义的。

- `ListApp` 用于列出当前系统中的所有用户程序，由于尚不会进行文件系统的实现，因此需要这样一个系统调用来获取用户程序的信息。
- `Stat` 用于获取系统中的一些统计信息，例如内存使用情况、进程列表等，用于调试和监控。
- `Allocate/Deallocate` 用于分配和释放内存。在当前没有完整的用户态内存分配支持的情况下，可以利用系统调用将其委托给内核来完成。

### 软中断处理

在 [CPU 中断处理](../../wiki/interrupts.md)中介绍了软件中断的使用方式和用途，在 Linux 中，一个基于中断的系统调用可以用如下的汇编实现：

```c
int _start() {
    __asm__(
        // 准备系统调用参数
        "xor    %eax,%eax\n"
        "push   %eax\n"
        "push   $0x68732f2f\n"
        "push   $0x6e69622f\n"
        "mov    %esp,%ebx\n"
        "push   %eax\n"
        "push   %ebx\n"
        "mov    %esp,%ecx\n"
        "mov    $0xb,%al\n"

        // 系统调用中断
        "int    $0x80\n"
    );
}
```

??? note "如果想要测试，需要编译为 32 位的可执行文件"

    你可以使用如下的命令编译：

    ```bash
    gcc syscall.c -o syscall -m32 -nostdlib -nodefaultlibs -fno-exceptions -no-pie
    ```

    你可以使用 `objdump -d syscall` 来查看编译后的汇编代码。

    你可以在 x86_64 的机器上直接运行它。

    这段汇编的代码的作用是调用 `execve` 系统调用，执行 `/bin/sh`，也即一段 `shellcode`。

!!! question "`int 0x80`、`sysenter` 和 `syscall` 的区别？"

    `int 0x80` 是基于中断的系统调用实现，通常在早期的 32 位 x86 系统中使用。

    由于软中断保存、恢复现场、查询中断向量表等操作对于系统调用进入内核态来说过于**重**了，一定程度上影响 CPU 执行用户程序的效率，`sysenter` 和 `syscall` 被引入。

    它们专门用于系统调用，`sysenter` 用于 32 位系统，`syscall` 用于 64 位系统。它们不查询中断描述符表，通过直接访问某些特殊寄存器等指令和 CPU 设计上的优化，从而减少了系统调用的开销。

    你可以查阅资料，了解它们的具体实现和使用方式。但在本实验中，不需要考虑中断带来的性能损耗，实现时使用中断进行系统调用即可。

在 `src/interrupt/syscall/mod.rs` 中，补全中断注册函数，**并在合适的地方调用它**：

1. 在 `idt` 的初始化中，注册 `0x80` 号中断的处理函数为 `syscall_handler`。*`int 0x80` 并非 Irq 中断，直接使用 `consts::Interrupts::Syscall` 即可。*
2. 与时钟中断类似，或许为系统调用准备一个独立的栈是个好主意？
3. 使用 `set_privilege_level` 设置 `DPL` 为 `3`，以便用户态程序可以触发系统调用。

## 用户态库的实现

用户态库是用户程序的基础，它提供了一些基础的函数，用于调用系统调用，实现一些基础的功能。

在这一部分的实现中，着重实现了 `read` 和 `write` 系统调用的封装和内核侧的实现，并通过内存分配、释放的系统调用，给予用户态程序动态内存分配的能力。

后者已经在给出的代码中实现，你可以参考它们的内容进行其他系统调用功能的实现。

### 标准输入输出

为了在系统调用中实现基础的读写操作，代码中定义了一个 `Resource` 枚举，并借用 Linux 中“文件描述符”的类似概念，将其存储在进程信息中。

在 `src/proc/data.rs` 中，修改 `ProcessData` 结构体，类似于环境变量的定义，添加一个“文件描述符表”：

```rust
pub(super) file_handles: Arc<RwLock<BTreeMap<u8, Resource>>>
```

在 `ProcessData` 的 `default` 函数中，初始化此表，并添加标准输入输出的资源：

```rust
let mut file_handles = BTreeMap::new();

// stdin, stdout, stderr
file_handles.insert(0, Resource::Console(StdIO::Stdin));
file_handles.insert(1, Resource::Console(StdIO::Stdout));
file_handles.insert(2, Resource::Console(StdIO::Stderr));
```

之后添加 `handle` 函数，用于在系统调用中根据文件描述符获取资源：

```rust
pub fn handle(&self, fd: u8) -> Option<Resource> {
    self.file_handles.read().get(&fd).cloned()
}
```

系统调用总是为当前进程提供服务，因此可以在 `proc/mod.rs` 中对一些操作进行封装，封装获取当前进程、上锁等操作。以获取当前进程的文件描述符表为例：

```rust
pub fn handle(fd: u8) -> Option<Resource> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().current().read().handle(fd)
    })
}
```

对于 `write` 系统调用，用户程序需要将数据写入到资源中，对此系统调用进行如下约定：

```
fd: arg0 as u8, buf: &[u8] (ptr: arg1 as *const u8, len: arg2)
```

为了便于理解，给出了用户侧进行调用时的示例代码，从 `print!` 的实现开始：

```rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    stdout().write(format!("{}", args).as_str());
}
```

将会调用 `Stdout` 的 `write` 方法：

```rust
impl Stdout {
    pub fn write(&self, s: &str) {
        sys_write(1, s.as_bytes());
    }
}
```

传递给 `sys_write`，由此函数对传入的参数进行处理，并调用系统调用：

```rust
pub fn sys_write(fd: u8, buf: &[u8]) -> Option<usize> {
    let ret = syscall!(
        Syscall::Write,
        fd as usize,
        buf.as_ptr() as usize,
        buf.len() as usize
    ) as isize;
    if ret.is_negative() {
        None
    } else {
        Some(ret as usize)
    }
}
```

系统调用宏 `syscall!` 的实现如下：

```rust
#[macro_export]
macro_rules! syscall {
    // ...
    ($n:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::macros::syscall3($n, $a1 as usize, $a2 as usize, $a3 as usize)
    };
}

#[doc(hidden)]
#[inline(always)]
pub fn syscall3(n: Syscall, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "int 0x80", in("rax") n as usize,
            in("rdi") arg0, in("rsi") arg1, in("rdx") arg2,
            lateout("rax") ret
        );
    }
    ret
}
```

通过内联汇编的形式，将参数列表存入对应的寄存器中，再调用 `int 0x80` 中断，触发系统调用。

在内核正确响应中断，并进行服务分发后，调用对应的服务处理函数。

在 `interrupt/syscall/service.rs` 中，你需要实现 `sys_write` 函数，用于处理 `write` 系统调用，使得用户程序得以进程输出：

1. 使用上述 `proc::handle` 获取文件描述符，并处理 `Option`。
2. 使用 `core::slice::from_raw_parts` 将用户程序的缓冲区转换为 `&[u8]`。
3. 将缓冲区传入资源的 `write` 方法中，并返回写入的字节数。
4. 在分发函数中使用 `context.set_rax` 设置返回值，并调用 `sys_write` 函数。

!!! tip "参考 `sys_allocate` 的实现和相关用户侧代码进行实现"

`sys_read` 的实现与 `sys_write` 类似，为了更好的兼容性和更低的实现难度，在实现 `read` 时候需要注意如下几点：

1. 在 `Resource` 中添加 `read` 方法时，从输入缓冲区中读取数据并写入到用户程序的缓冲区中。
2. 遵循相关原则，系统调用理论上不应等待，如果没有数据可读，应立即返回。
3. 对用户输入的等待行为应当在用户态程序中实现，并应该提供非阻塞的读取方式。
4. 对于一个固定的输入序列，需要特殊处理控制字符，如回车、退格等。这些处理需要在用户态进行，并通过 `write` 系统调用进行反馈输出。
5. 在用户态需要控制好缓冲区的大小，遵守系统调用返回的长度进行处理。

### 进程的退出

### 进程的创建

### 进程的等待

## 运行 Shell

启动其他程序，验证功能

## (可选)内存回收

进程退出，栈、页表、文件内存回收

## 思考题

## 加分项

1. 🤔 尝试在 `ProcessData` 中记录代码段的占用情况，并统计当前进程所占用的页面数量，并在打印进程信息时，将进程的内存占用打印出来。
