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

在 `crates/app` 中，定义提供了一些用户程序，这些程序将会在编译后提供给内核加载运行。

在 `crates/syscall` 中，提供系统调用号和调用约束的定义，将会在内核和用户库中使用，在下文中会详细介绍。

在 `crates/lib` 中，定义了用户态库并提供了一些基础实现，相关内容在下文中会详细介绍。

在 `crates/kernel` 中，添加了如下一些模块：

- `interrupt/syscall`：定义系统调用及其服务的实现。
- `memory/user`：用户堆内存分配的实现，会被用在系统调用的处理中，将用户态的内存分配委托给内核。
- `utils/resource`：定义了用于进行 I/O 操作的 `Resource` 结构体，用于处理用户态的读写系统调用。

!!! tip "别忘了更新 `Cargo.toml`"

## 用户程序

### 编译用户程序

对于不同的运行环境，即使指令集相同，一个可执行的程序仍然有一定的差异。

与内核的编译类似，在 `crates/app/config` 中，定义了用户程序的编译目标，并定义了相关的 LD 链接脚本。

在 `Cargo.toml` 中，使用通配符引用了 `crates/app` 中的所有用户程序。相关的编译过程在先前给出的编译脚本中均已定义，可以直接编译。

通常而言，用户程序并不直接自行处理系统调用，而是由用户态库提供的函数进行调用。

在编写 C 语言时 `printf`、`scanf` 等函数并不是直接调用系统调用。以一般的 GNU/Linux 程序为例，这些函数被定义在 `glibc` (GNU C Library) 中，而 `glibc` 会处理系统调用。相对应的，在 Windows 上，也会存在 `msvcrt` (Microsoft Visual C Run-time) 和 `ucrt` (Universal C Runtime) 等库。

为了让用户态程序更好地与 YSOS 进行交互，处理程序的生命周期，便于编写用户程序等，需要提供用户态库，以便用户程序调用。

用户态库被定义在 `crates/lib` 中，在用户程序中，编辑 `Cargo.toml`，使用如下方式引用用户库：

```rust
[dependencies]
lib = { workspace = true }
```

!!! tip "在根目录给出的 `Cargo.toml` 中，为你准备了目标项目所需要的全部工作区依赖列表，你可以按需引用、删除、添加你需要的依赖"

一个简单的用户程序示例如下所示，同样存在于 `app/hello/src/main.rs` 中：

```rust
#![no_std]
#![no_main]

use lib::*;

extern crate lib;

fn main() -> isize {
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
        #[unsafe(export_name = "_start")]
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

!!! success "阶段性成果"

    在一切配置顺利之后，应当可以使用 `cargo build` 在用户程序目录中正确地编译用户程序。

### 加载程序文件

在成功编译了用户程序后，用户程序将被脚本移动到 `esp/APP` 目录下，并**以文件夹命名**。辅助脚本 `ysos.py` 功能已完备，不过 `Makefile` 需要根据此次实验进行更新。

> 由于 FAT16 文件系统的限制，文件名长度不能超过 8 个字符，所以建议使用简短的文件夹名。

!!! note "关于用户程序的命名"

    由于灵活性限制，`Makefile` 被设计为**只能将包名（`Cargo.toml` 中的 `package.name`）为 `ysos_$$app` 且文件夹名称为 `$$app`** 的用户程序复制到 `esp/APP` 目录下，其他的命名将会面临错误。

    而 `ysos.py` 脚本则会尝试读取 `Cargo.toml` 的配置，根据**对应名称**的直接复制到 `esp/APP` 目录下的**文件夹名称**，因此比 `Makefile` 更加灵活。

    如果你有其他需求，也可以自行修改辅助脚本，使得编译流程的正确执行。

目前的内核尚不具备访问磁盘和文件系统，并将它们读取加载的能力（将会在实验六中实现），因此需要另辟蹊径：在 bootloader 中将符合条件的用户程序加载到内存中，并将它们交给内核，用于生成用户进程。

!!! note "修改内核配置文件"

    这就是 lab 1 中 `Config` 含有 `load_apps` 的原因。

    本次实验你应当在 `crates/kernel/config/boot.conf` 中，将 `load_apps` 设置为 `true`。

为了存储用户程序的相关信息，在 `crates\boot\src\lib.rs` 中，定义一个 `App` 结构体，并添加“已加载的用户程序”字段到 `BootInfo` 结构体中：

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
    - 尝试定义 `AppListRef` 类型，用于存储 `loaded_apps.as_ref()` 的返回值类型，可以只关心 `'static` 生命周期。
    - 抛弃 `App` 类型的生命周期，直接声明 `ElfFile<'static>`。

之后，在 `crates/boot/src/fs.rs` 中，创建函数 `load_apps` 用于加载用户程序，并参考 `fs.rs` 中的其他函数，处理文件系统相关逻辑，补全代码：

```rust
/// Load apps into memory, when no fs implemented in kernel
///
/// List all file under "APP" and load them.
pub fn load_apps() -> AppList {
    let mut root = open_root();
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
    Some(load_apps())
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

!!! success "阶段性成果"

    在 `kernel/src/main.rs` 初始化内核之后，尝试调用 `list_app` 函数，查看是否成功加载。

### 生成用户程序

在 `kernel/src/proc/mod.rs` 中，添加 `spawn` 和 `elf_spawn` 函数，将 ELF 文件从列表中取出，并生成用户程序：

```rust
pub fn spawn(name: &str) -> Option<ProcessId> {
    let app = x86_64::instructions::interrupts::without_interrupts(|| {
        let app_list = get_process_manager().app_list()?;
        app_list.iter().find(|&app| app.name.eq(name))
    })?;

    elf_spawn(name.to_string(), &app.elf)
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
    let page_table = kproc.read().clone_page_table();
    let proc_vm = Some(ProcessVm::new(page_table));
    let proc = Process::new(name, parent, proc_vm, proc_data);

    let mut inner = proc.write();
    // FIXME: load elf to process pagetable
    // FIXME: alloc new stack for process
    // FIXME: mark process as ready
    drop(inner);

    trace!("New {:#?}", &proc);

    let pid = proc.pid();
    // FIXME: something like kernel thread

    pid
}
```

!!! warning "**删除或注释上次实验中有关内核线程的代码，防止后续修改后的进程模型在执行内核线程时遇到意外的问题。**"

在加载的过程中，你可以复用 `elf::load_elf` 函数。可以通过为它添加参数的方式，控制这一映射的内容是否可以被用户权限（Ring 3）代码访问。

```rust
pub fn load_elf(
    elf: &ElfFile,
    physical_offset: u64,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    user_access: bool
) -> /* return type */ {
    // FIXME: just like kernel's load_elf
}
```

在映射页面时，根据此参数决定是否添加 `USER_ACCESSIBLE` 标志位：

```rust
if user_access {
    page_table_flags |= PageTableFlags::USER_ACCESSIBLE;
}
```

这一标志位只应当为用户进程所使用，内核相关代码不应当拥有这一权限。由于用户程序是不可信的，需要以此防止用户态程序访问内核的内存空间。

!!! note "**对于用户进程而言，不再与内核共享页表，而是通过克隆内核页表获取了自己的页表。这意味着可以为每个用户进程分配同样的栈地址，而不会相互干扰。**"

之后在 `ProcessInner` 和 `ProcessVm` 中实现 `load_elf` 函数，来处理代码段映射等内容。

```rust
pub fn load_elf(&mut self, elf: &ElfFile) {
    let mapper = &mut self.page_table.mapper();
    let alloc = &mut *get_frame_alloc_for_sure();

    self.stack.init(mapper, alloc);

    // FIXME: load elf to process pagetable
}
```

同时，需要在 GDT 中为 Ring 3 的代码段和数据段添加对应的选择子，在初始化栈帧的时候将其传入。在 `kernel/src/memory/gdt.rs` 中，你可以使用如下方式添加：

```rust
lazy_static! {
    static ref GDT: /* your type */ = {
        let mut gdt = GlobalDescriptorTable::new();
        // ...
        let user_code_selector = gdt.append(Descriptor::user_code_segment());
        let user_data_selector = gdt.append(Descriptor::user_data_segment());
        // ...
    };
}
```

之后将其通过合适的方式暴露出来，以供栈帧初始化时使用：

```rust
pub fn init_stack_frame(&mut self, entry: VirtAddr, stack_top: VirtAddr) {
    // ...
    let selector = get_user_selector(); // FIXME: implement this function

    self.value.stack_frame.code_segment = selector.user_code_selector;
    self.value.stack_frame.stack_segment = selector.user_data_selector;
    // ...
}
```

!!! tip "一些提示"

    - 与内核类似，使用 `elf.header.pt2.entry_point()` 获取 ELF 文件的入口地址。
    - 记得为进程分配好合适的栈空间，并使用 `init_stack_frame` 初始化程序栈和指令指针。
    - 或许你可以同时实现 **加分项 1** 所描述的功能。

!!! question "阶段性成果？"

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

为了能在用户库（调用系统调用侧）和内核态（处理系统调用侧）之间达成系统调用号的一致性，在 `crates/syscall` 中定义了一个 `Syscall` 枚举，用于存储系统调用号。

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

1. 在 `idt` 的初始化中，注册 `0x80` 号中断的处理函数为 `syscall_handler`。_`int 0x80` 并非 Irq 中断，直接使用 `consts::Interrupts::Syscall` 即可。_
2. 与时钟中断类似，或许为系统调用准备一个独立的栈是个好主意？
3. 使用 `set_privilege_level` 设置 `DPL` 为 `3`，以便用户态程序可以触发系统调用。

## 用户态库的实现

用户态库是用户程序的基础，它提供了一些基础的函数，用于调用系统调用，实现一些基础的功能。

在这一部分的实现中，着重实现了 `read` 和 `write` 系统调用的封装和内核侧的实现，并通过内存分配、释放的系统调用，给予用户态程序动态内存分配的能力。

### 动态内存分配

为了方便用户态程序使用动态内存分配，而不是基于 `brk` 等方式进行完全用户态的动态内存管理，选择使用系统调用的方式，将内存分配的任务委托给内核完成。

与内核堆类似，在 `src/memory/user.rs` 中，定义了用户态的堆。

与内核使用 `static` 在内核 `bss` 段声明内存空间不同，由于在页表映射时需添加 `USER_ACCESSIBLE` 标志位，**用户态堆需要采用内核页面分配的能力完成**。其次需要注意的是，为了调试和安全性考量，这部分内存还需要 `NO_EXECUTE` 标志位。

有关用户态堆初始化的过程需要补全部分代码，指定合适的页面范围，分配并映射相关的页表，你可以参考、使用并修改 `elf::map_range` 完成这里的初始化。

!!! question "内存安全？"

    这里的实现是不够安全的，共享用户态堆意味着用户程序可以任意读取访问其他用户进程的堆空间。

    不过这样的实现可以大幅简化用户态库的相关实现，在能且仅能运行自己的程序、完成实验方面，这样的选择是很合适的。

在系统调用的实现方面，给出的代码中已包含，可以参考它们的内容进行其他系统调用功能的实现。

### 标准输入输出

为了在系统调用中实现基础的读写操作，代码中定义了一个 `Resource` 枚举，并借用 Linux 中“文件描述符”的类似概念，将其存储在进程信息中。

!!! note "缓冲区与系统调用开销"

    为了简化实验，这里的实现并不需要考虑 I/O 缓冲区和批处理的问题。但是在实际的操作系统中，这些考量对性能非常重要。

    在 libc 中，数据在用户态被累积起来，在达到一定数量后，通过一次系统调用进行处理（flush）—— 相较于 1024 次系统调用、每次输出一个字符的总开销，显然具有缓冲区之后的开销更小。

在 `src/proc/data.rs` 中，修改 `ProcessData` 结构体，类似于环境变量的定义，添加一个“文件描述符表”：

```rust
pub(super) resources: Arc<RwLock<ResourceSet>>,
```

在 `ProcessData` 的 `default` 函数中初始化，添加默认的资源：

```rust
resources: Arc::new(RwLock::new(ResourceSet::default()))
```

之后添加 `read` 和 `write` 函数，用于在系统调用中根据文件描述符进行读写：

```rust
pub fn read(&self, fd: u8, buf: &mut [u8]) -> isize {
    self.resources.read().read(fd, buf)
}

pub fn write(&self, fd: u8, buf: &[u8]) -> isize {
    self.resources.read().write(fd, buf)
}
```

系统调用总是为当前进程提供服务，因此可以在 `proc/mod.rs` 中对一些操作进行封装，封装获取当前进程、上锁等操作：

```rust
pub fn read(fd: u8, buf: &mut [u8]) -> isize {
    x86_64::instructions::interrupts::without_interrupts(|| get_process_manager().read(fd, buf))
}

pub fn write(fd: u8, buf: &[u8]) -> isize {
    x86_64::instructions::interrupts::without_interrupts(|| get_process_manager().write(fd, buf))
}
```

对于 `write` 系统调用，用户程序需要将数据写入到资源中，在 `syscall::dispatcher` 中对此系统调用的相关参数进行如下约定：

```
fd: arg0 as u8, buf: &[u8] (ptr: arg1 as *const u8, len: arg2)
```

为了便于理解，给出了用户侧进行调用时的相关调用过程，从 `print!` 的实现开始：

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

从而传递给 `sys_write`，由此函数对传入的参数进行处理，并调用系统调用：

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

1. 使用 `core::slice::from_raw_parts` 将用户程序的缓冲区转换为 `&[u8]`。
2. 将缓冲区传入资源的 `write` 方法中，并返回写入的字节数。
3. 在分发函数中使用 `context.set_rax` 设置返回值，并调用 `sys_write` 函数。

!!! tip "参考 `sys_allocate` 的实现和相关用户侧代码进行实现"

`sys_read` 的实现与 `sys_write` 类似，为了更好的兼容性和更低的实现难度，在实现 `read` 时候需要注意如下几点：

1. 在 `Resource` 中添加 `read` 方法时，从内核输入缓冲区中读取数据并写入到用户程序的缓冲区中。
2. 遵循相关原则，系统调用理论上不应等待，如果没有数据可读，应立即返回。
3. 对用户输入的等待行为应当在用户态程序中实现，并应该提供非阻塞的读取方式。
4. 对于一个固定的输入序列，需要特殊处理控制字符，如回车、退格等。这些处理需要在用户态进行，并通过 `write` 系统调用进行反馈输出。
5. 在用户态需要控制好缓冲区的大小，遵守系统调用返回的长度进行处理。
6. 可以使用 `from_raw_parts_mut` 将用户程序的缓冲区转换为 `&mut [u8]`。

### 进程的退出

与内核线程防止再次被调度的“退出”不同，用户程序的正常结束，需要在用户程序中调用 `exit` 系统调用，以通知内核释放资源。

由于此时通过中断进入内核态，与时钟中断类似，操作系统得以控制**退出中断时的 CPU 上下文**。因此可以在退出的时候清理进程占用的资源，并调用 `switch_next` 函数，切换到下一个就绪的进程。

> 为了实现的简单，不需要实验程序异常退出的相关情况处理，将错误展示出来也更便于调试。

与 `handle` 的实现类似，也可以通过在 `kernel/src/proc/mod.rs` 中实现 `exit` 函数，封装对应的功能，并暴露给系统调用：

```rust
pub fn exit(ret: isize, context: &mut ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        // FIXME: implement this for ProcessManager
        manager.kill_current(ret);
        manager.switch_next(context);
    })
}
```

!!! tip "增加代码重用"

    退出的操作并不只用于当前进程，也可以作用于其他进程，因此可以通过如下方式将它们统一：

    ```rust
    pub fn kill_current(&self, ret: isize) {
        self.kill(processor::get_pid(), ret);
    }

    pub fn kill(&self, pid: ProcessId, ret: isize) {
       // FIXME: get process by pid
       // FIXME: kill process and set return code
    }
    ```

    这只是一种可能的实现方式，你可以根据自己的需求进行调整。

进程退出释放资源的过程基本有如下几步：

1. 释放进程的虚拟内存信息（`ProcessVm`），释放页表和页表占用的页面。

    > 这部分将在后续实验中实现。此处不需要关心，但是建议完成 **加分项 2** 的内容

2. 删除进程的数据（`ProcessData`），可以使用 `take` 方法。

3. 设置进程的返回值，并设置状态为 `Dead`。

!!! success "阶段性成果"

    终于！在实现进程的退出之后，用户程序可以生成并正确退出了！

    尝试修改用户态库中的 `entry!` 和 `panic` 函数，在用户程序中调用 `exit` 系统调用，并传递一个返回值，以验证用户程序的退出功能。

    至此为止，你应当可以生成用户程序、输出 `Hello, world!!!`，并正确退出。

### 进程的创建与等待

操作系统的内核入口点将能够简单实现如下功能：

1. 初始化内核
2. 生成 `init` 进程，并等待它退出
3. 关机

其中，等待进程退出的函数 `ysos::wait` 可以定义在 `kernel/src/lib.rs` 中：

```rust
pub fn wait(init: proc::ProcessId) {
    loop {
        if proc::still_alive(init) {
            // Why? Check reflection question 5
            x86_64::instructions::hlt();
        } else {
            break;
        }
    }
}
```

并在 `kernel/src/proc/mod.rs` 中，补全 `still_alive` 函数：

```rust
#[inline]
pub fn still_alive(pid: ProcessId) -> bool {
    x86_64::instructions::interrupts::without_interrupts(|| {
        // check if the process is still alive
    })
}
```

对于具体的进程操作、目录操作等功能，将会移步到用户态程序进行实现。为了给予用户态程序操作进程、等待进程退出的能力，这里还缺少最后几个系统调用需要实现：`spawn`、`getpid` 和 `waitpid`，对这些系统调用有如下约定：

```rust
// path: &str (arg0 as *const u8, arg1 as len) -> pid: u16
Syscall::Spawn => { /* ... */ },
// None -> pid: u16
Syscall::GetPid => { /* ... */ },
// pid: arg0 as u16 -> status: isize
Syscall::WaitPid => { /* ... */},
```

相关必要函数和参考代码在前文中均有涉及，你可以参考前文的内容进行系统调用的实现。

> 你可以自由定义你的内核与用户态的交互方式了！

!!! note "关于 `waitpid` 的问题"

    在这里的简单实现下，`waitpid` 不进行阻塞，应当立刻返回进程的当前状态。从而本次实验中，在用户态使用**忙等待**的方式判断进程是否退出。

    > 你的实现遇到了 GPF？尝试解决思考题 5？

    但是`waitpid` 需要返回特殊状态，以区分进程正在运行还是已经退出。这非常糟糕，当前进程的返回值也是一个 `isize` 类型的值，这意味着如果按照现在的设计，势必存在一些返回值和“正在运行”的状态冲突。不过在本次实验中，这并不会造成太大的问题。

    此问题的更好解决方案将在 Lab 5 中进行讨论。

## 运行 Shell

至此，你可以编写自己的 Shell 了！作为用户与操作系统的交互方式，它需要实现一些必须功能：

- 列出当前系统中的所有用户程序
- 列出当前正在运行的全部进程
- 运行一个用户程序

同时，它也可以实现一些辅助的能力：

- 列出帮助信息
- 清空屏幕
- ...

为了实现一些信息的查看，你也需要实现如下两个系统调用：

```rust
// None
Syscall::Stat => { /* ... */ },
// None
Syscall::ListApp => { /* ... */},
```

!!! tip "实验任务"

    尝试结合 lab 0 的内容，实现一个你自己的 Shell！

    它应当能够生成一个用户程序，等待它退出，并输出它的返回值。实现较为灵活，你可以自行调整用户态库和用户程序，以你喜欢的方式构建操作系统的用户态。

    除此之外，请实现 `help` 命令，并在其中输出帮助信息，并附加你的学号。

### 测试程序

作为之前所有程序功能的测试和检验，你需要在用户态下正确运行如下程序：

```rust
const MOD: u64 = 1000000007;

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1) % MOD
    }
}

fn main() -> isize {
    print!("Input n: ");

    let input = lib::stdin().read_line();

    // prase input as u64
    let n = input.parse::<u64>().unwrap();

    if n > 1000000 {
        println!("n must be less than 1000000");
        return 1;
    }

    // calculate factorial
    let result = factorial(n);

    // print system status
    sys_stat();

    // print result
    println!("The factorial of {} under modulo {} is {}.", n, MOD, result);

    0
}

entry!(main);
```

你可以进行一些交互测试，例如输入 `100` 或更大的数据（会产生更大的栈占用！）并查看输出结果。在最大规模（输入数据为 `999999` 时），预期输出为：

```
The factorial of 999999 under modulo 1000000007 is 128233642.
```

你可以使用此测例来进行对照参考。此用户程序约占用 3929 个页面，总计内存占用约 15.3MiB。

## 思考题

1. 是否可以在内核线程中使用系统调用？并借此来实现同样的进程退出能力？分析并尝试回答。

2. 为什么需要克隆内核页表？在系统调用的内核态下使用的是哪一张页表？用户态程序尝试访问内核空间会被正确拦截吗？尝试验证你的实现是否正确。

3. 为什么在使用 `still_alive` 函数判断进程是否存活时，需要关闭中断？在不关闭中断的情况下，会有什么问题？

4. 对于如下程序，使用 `gcc` 直接编译：

    ```c
    #include <stdio.h>

    int main() {
        printf("Hello, World!\n");
        return 0;
    }
    ```

    从本次实验及先前实验的所学内容出发，结合进程的创建、链接、执行、退出的生命周期，参考系统调用的调用过程（可以仅以 Linux 为例），解释程序的运行。

5. `x86_64::instructions::hlt` 做了什么？为什么这样使用？为什么不可以在用户态中的 `wait_pid` 实现中使用？

6. 有同学在某个回南天迷蒙的深夜遇到了奇怪的问题：

    只有当进行用户输入（触发了串口输入中断）的时候，会触发奇怪的 Page Fault，然而进程切换、内存分配甚至 `fork` 等系统调用都很正常。

    **经过<s>近三个小时的</s>排查，发现他将 TSS 中的 `privilege_stack_table` 相关设置注释掉了。**

    请查阅资料，了解特权级栈的作用，实验说明这一系列中断的触发过程，尝试解释这个现象。

    - 可以使用 `intdbg` 参数，或 `ysos.py -i` 进行数据捕获。
    - 留意 `0x0e` 缺页异常和缺页之前的中断的信息。
    - 注意到一个不应当存在的地址……？

    或许你可以重新复习一下 Lab 2 的相关内容：[double-fault-exceptions](https://os.phil-opp.com/double-fault-exceptions/)

## 加分项

1. 😋 尝试在 `ProcessData` 中记录代码段的占用情况，并统计当前进程所占用的页面数量，并在打印进程信息时，将进程的内存占用打印出来。

2. 😋 尝试在 `kernel/src/memory/frames.rs` 中实现帧分配器的回收功能 `FrameDeallocator`，作为一个最小化的实现，你可以在 `Allocator` 使用一个 `Vec` 存储被释放的页面，并在分配时从中取出。

3. 🤔 基于帧回收器的实现，在 `elf` 中实现 `unmap_range` 函数，从页表中取消映射一段连续的页面，并使用帧回收器进行回收。之后，在合适的地方，结合 `ProcessData` 中存储的页面信息，利用这个函数实现进程栈的回收。其他进程资源（如页表、代码段、数据段等）的回收将会在后续实验中实现，目前暂时不需要考虑。

4. 🤔 尝试利用 `uefi::runtime::get_time()` 和 `chrono` crate，获取当前时间，并将其暴露给用户态，以实现 `sleep` 函数。

    这里提供一个可能的 `sleep` 函数实现：

    ```rust
    pub fn sleep(millisecs: i64) {
        let start = sys_time();
        let dur = Duration::milliseconds(millisecs);
        let mut current = start;
        while current - start < dur {
            current = sys_time();
        }
    }
    ```

    > 当前实现是纯用户态、采用轮询的，这种实现是很低效的。
    >
    > 在现代操作系统中，进程会被挂起，并等待对应事件触发后重新被调度。
    >
    > 虽然不是最好，但是在目前的需求下，这已经足够了。

    在实现后，写一个或更改现有用户程序，验证你的实现是否正确，尝试输出当前时间并使用 `sleep` 函数进行等待。
