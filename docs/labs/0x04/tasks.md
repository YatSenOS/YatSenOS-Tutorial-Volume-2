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

## 用户程序的编译

对于不同的运行环境，即使指令集相同，一个可执行的程序仍然有一定的差异。

与内核的编译类似，在 `pkg/app/config/x86_64-unknown-ysos.json` 中，定义了用户程序的编译目标，并定义了相关的 LD 链接脚本。

在 `Cargo.toml` 中，使用通配符引用了 `pkg/app` 中的所有用户程序。相关的编译过程在先前给出的编译脚本中均已定义，可以直接编译。

通常而言，用户程序并不直接自行处理系统调用，而是由用户态库提供的函数进行调用。

在编写 C 语言时 `printf`、`scanf` 等函数并不是直接调用系统调用，以 gcc on Linux 的一般行为为例，这些函数被定义在 `glibc` 中，而 `glibc` 会处理系统调用。相对应的，在 Windows 上，也会存在 `msvcrt` (Microsoft Visual C Run-time) 等库。

为了让用户态程序更好地与 YSOS 进行交流，处理程序的生命周期等，需要提供用户态库，以便用户程序调用。

## 用户程序的加载

load_elf，Process

!!! note "阶段性目标？"

    由于并没有实现任何系统调用服务（包括程序的退出、输入输出、内存分配等），因此你在加载用户程序后，基本无法进行任何操作。

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
