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

!!! tip "如何使用本次参考代码"

    为了帮助大家进行项目代码的结构组织，本次实验给出的参考代码中包含了**完整的文件结构**。

    这意味着你可以**直接使用本次实验的[参考代码](https://github.com/YatSenOS/YatSenOS-Tutorial-Volume-2/tree/main/src/0x01/)** （路径为 `YatSenOS-Tutorial-Volume-2/src/0x01`）进行实验，而不需要从头开始编写项目结构。

    请将代码**复制到你自己的仓库中并替换重复文件以进行实验**，而不是在本仓库中进行。

    请注意本次实验中的 `Makefile` 和 `ysos.py` 均有更新，并注意保留 `assets/OVMF.fd` 文件。

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
    - 找出内核的入口点，它是被如何控制的？结合源码、链接、加载的过程，谈谈你的理解。
    - 请找出编译产物的 segments 的数量，并且**用表格的形式**说明每一个 segments 的权限、是否对齐等信息。

## 在 UEFI 中加载内核

经过上述的配置，内核将会被编译为一个 ELF 文件，下一步需要在 UEFI 程序中加载这个文件、准备好内核的运行环境，最后跳转到内核进行执行。这一过程中，这个 UEFI 程序所扮演的角色就是 bootloader。

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

1. 加载配置文件：加载配置文件，解析其中的内核栈大小、内核栈地址等内容。
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

为了进行后续读取并映射内核 ELF 文件的操作，你需要使用 `Cr0` 寄存器禁用根页表的写保护，以便进行后续的内存映射操作。

### 映射内核文件

在成功加载内核，并禁用根页表写保护后，需要将内核的代码段、数据段、BSS 段等映射到虚拟地址空间中。你可以参考和使用 `pkg/elf/src/lib.rs` 中的相关函数进行映射工作。

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

依据[调试教程](../../wiki/debug.md)的相关内容，搭建基于命令行的 GDB 调试环境。

作为实验的推荐调试环境，你需要配置好 `gef` 插件以进行更加灵活的二进制调试。同时利用 VSCode 进行调试也是一个不错的选择，鼓励你进行尝试，它将会作为实验的加分项目之一。

最后，你需要检验是否成功加载了内核：

- 使用 `make build DBG_INFO=true` 或 `python ysos.py build -p debug` 编译内核，确保编译时开启了调试信息。
- 使用 `make debug` 或 `python ysos.py launch -d` 启动 QEMU 并进入调试模式，这时候 QEMU 将会等待 GDB 的连接。
- 在另一个终端中，使用 `gdb -q` 命令进入 GDB 调试环境。

    !!! note "使用 `.gdbinit` 方便你的调试过程"

        以下是一个 `.gdbinit` 的例子，你可以将其放置在你的工作目录下，这样每次进入 GDB 调试环境时，它都会自动加载。请注意部分指令是 `gef` 所提供的，详情请见调试文档。

        ```bash
        file esp/KERNEL.ELF
        gef-remote localhost 1234
        tmux-setup
        b ysos_kernel::init
        ```

- 使用 `c` 命令继续执行，你将会看到 QEMU 窗口中的输出，同时 GDB 将会在断点处停下。
- 查看断点处的汇编和符号是否正确，使用 `vmmap` 和 `readelf` 等指令查看内核的加载情况。

!!! tip "遇到了奇怪的问题？尝试更改 `log::set_max_level(log::LevelFilter::Info);` 来调整日志输出的等级，以便你能够观察到更多的日志输出。"

!!! question "实验任务"

    完成上述代码任务，回答如下的问题：

    - `set_entry` 函数做了什么？为什么它是 unsafe 的？
    - `jump_to_entry` 函数做了什么？它将传递给内核参数留在了哪里？借助调试器进行说明。
    - `entry_point!` 宏做了什么？内核为什么需要使用它声明自己的入口点？
    - 如何为内核提供直接访问物理内存的能力？你知道几种方式？代码中所采用的是哪一种？
    - 为什么 ELF 文件中不描述栈的相关内容？栈是如何被初始化的？它可以被任意放置吗？

    根据上述调试过程，回答以下问题，并给出你的回答与必要的截图：

    - 请解释指令 `layout asm` 的功能。倘若想找到当前运行内核所对应的 Rust 源码，应该使用什么 GDB 指令？
    - 假如在编译时没有启用 `DBG_INFO=true`，调试过程会有什么不同？
    - 你如何选择了你的调试环境？截图说明你在调试界面（TUI 或 GUI）上可以获取到哪些信息？

## UART 与日志输出

### 串口驱动

在 [UART 串口通信](../../wiki/uart.md) 部分中介绍了 UART 的基本原理，以及相关的基础知识。在这一部分实验中，你将会实现一个简单的串口驱动，并将其用于内核的日志输出。

由于这是第一次进行驱动的编写，你可以在 `pkg/kernel/src/drivers` 目录下看到一个基本的代码框架，你需要完成其中的 `uart16550` 驱动。

在 `pkg/kernel/src/drivers/serial.rs` 中存放了串口初始化的相关代码，你所实现的 `SerialPort` 结构体将会在这里被调用：

```rust
use super::uart16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8; // COM1

once_mutex!(pub SERIAL: SerialPort);

pub fn init() {
    init_SERIAL(SerialPort::new(SERIAL_IO_PORT));
    get_serial_for_sure().init();

    println!("{}", crate::get_ascii_header());
    println!("[+] Serial Initialized.");
}

guard_access_fn!(pub get_serial(SERIAL: SerialPort));
```

!!! tip "更好的写法？"

    你可以尝试使用常量泛型（Const Generics）来实现对于不同端口的定义：

    ```rs
    pub struct SerialPort<const BASE_ADDR: u16> {
        // ...
    }

    impl<const BASE_ADDR: u16> SerialPort<BASE_ADDR> {
        pub const unsafe fn new() -> Self {
            Self {
                // use BASE_ADDR here
            }
        }
    }
    ```

    这样定义之后，你还需要适当修改上述 `serial.rs` 文件中的代码，以便使用这一泛型结构体。

#### 被保护的全局静态对象

在 Rust 中对全局变量的写入是一个 unsafe 操作，因为这是**线程不安全的**，如果直接使用全局静态变量，编译器会进行报错。但是对于 **“串口设备”** 这一类 **静态的全局对象** 我们确实需要进行一些数据存储，为了内存安全，就会不可避免的引入了**互斥锁**来进行保护。

!!! question "在 `pkg/boot/lib.rs` 中的 `ENTRY` 是如何被处理的？"

在内核框架中，我们提供了两个宏来帮助你实现这一功能：

```rust
once_mutex!(pub SERIAL: SerialPort);
guard_access_fn!(pub get_serial(SERIAL: SerialPort));
```

!!! note "你可以在 `pkg/kernel/src/utils/macros.rs` 中找到这些宏的定义。"

这两段代码将会被展开为：

```rust
pub static SERIAL: spin::Once<spin::Mutex<SerialPort>> = spin::Once::new();

#[allow(non_snake_case)]
pub fn init_SERIAL(val_SERIAL: SerialPort) {
    SERIAL.call_once(|| spin::Mutex::new(val_SERIAL));
}

#[inline(never)]
#[allow(non_snake_case, dead_code)]
pub fn get_serial<'a>() -> Option<spin::MutexGuard<'a, SerialPort>> {
    SERIAL.get().and_then(spin::Mutex::try_lock)
}

#[inline(never)]
#[allow(non_snake_case, dead_code)]
pub fn get_serial_for_sure<'a>() -> spin::MutexGuard<'a, SerialPort> {
    SERIAL.get().and_then(spin::Mutex::try_lock)
        .expect("SERIAL has not been initialized or lockable")
}
```

这里使用了 `spin::Mutex` 来对 `SerialPort` 进行了保护，同时提供了 `get_serial` 和 `get_serial_for_sure` 两个函数来尝试获取互斥锁所有权，并返回 `SerialPort` 的引用。

`spin::Once` 声明了一个 `Once` 类型的静态变量，它的 `call_once` 方法接受一个闭包作为参数，这个闭包将会在第一次调用 `call_once` 时被执行，之后的调用将会被忽略。**这确保了 `SERIAL` 只会被初始化一次。**

`spin::Mutex` 是一个基于自旋锁实现的互斥锁，它的 `try_lock` 方法尝试获取互斥锁的所有权，如果获取成功，则返回一个 `MutexGuard`，这个 `MutexGuard` 将会在离开作用域时自动释放互斥锁。

部分有关于“自选锁”和“互斥锁”的实现代码如下所示：

```rust
pub fn lock(&self) -> SpinMutexGuard<T> {
    loop {
        if let Some(guard) = self.try_lock() {
            break guard;
        }

        while self.is_locked() {
            R::relax();
        }
    }
}

pub fn try_lock(&self) -> Option<SpinMutexGuard<T>> {
    if self
        .lock
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        Some(SpinMutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        })
    } else {
        None
    }
}
```

上述代码通过 `AtomicBool` 类型的 `compare_exchange` 函数中的**比较-交换原子指令**尝试获取互斥锁的所有权，并在没有成功获取时进行自旋等待。

!!! tip "本次实验中，这部分不要求进行了解，只需要知道它们的基本原理和使用方法即可。"

你可以在 `spin` crate 的[文档](https://docs.rs/spin/latest/spin)中找到更多信息。

#### 串口驱动的设计

在考虑 IO 设备驱动的设计时，你需要考虑如下问题：

1. 为了描述驱动的**状态**，需要存储哪些数据？
2. 需要如何与**硬件**进行交互？
3. 与硬件交互的过程中，需要考虑哪些**并发**问题？
4. 驱动需要向内核提供哪些**接口**？

以 `uart16550` 为例，在 `x86_64` 架构下，你可以在 [Serial_Ports - OSDev](https://wiki.osdev.org/Serial_Ports) 中找到它的相关资料，包括寄存器标志位的含义、寄存器的地址等等。

为了与串口设备进行交互，你需要存储一个设备端口的基地址，对于 COM1 端口，它的基地址为 `0x3F8`。

在这一基地址的基础上，你可以通过偏移量来访问串口设备的寄存器，例如 `0x3F8 + 0` 将会访问串口设备的数据寄存器，`0x3F8 + 1` 将会访问串口设备的中断使能寄存器等等。*上方链接中的资料中有详细的寄存器地址和偏移量的对应关系。*

为了与这些寄存器进行交互，你可以使用 `x86_64` crate 中的 `Port`，以下是一个简单的例子：

```rust
let data = Port::new(base);
data.write(0x0A);
let ret = data.read();
```

对于只读和只写的寄存器，你可以使用 `PortWriteOnly` 和 `PortReadOnly` 来从类型系统上防止误操作的发生。

- 偏移量为 `1` 的寄存器是中断使能寄存器，可以使用 `PortWriteOnly::new(base + 1)` 操作。
- 偏移量为 `5` 的寄存器是线控寄存器，可以使用 `PortReadOnly::new(base + 5)` 操作。

对于串口设备，其寄存器均为 8 位，你可以使用 `u8` 类型来进行读写操作。

#### 串口驱动的实现

串口设备的驱动实现主要由初始化、发送数据、接收数据三部分组成。

参考 [Serial_Ports - OSDev](https://wiki.osdev.org/Serial_Ports) 中提供的如下示例代码，编写这部分驱动的 Rust 实现：

> 其中的 `outb` 和 `inb` 函数的含义是向指定的端口写入一个字节、从指定的端口读取一个字节。

##### 初始化

!!! note "请在初始化的每一步中使用注释说明你的操作。若存在其他代码参考，请使用注释说明。"

```c
#define PORT 0x3f8          // COM1

static int init_serial() {
   outb(PORT + 1, 0x00);    // Disable all interrupts
   outb(PORT + 3, 0x80);    // Enable DLAB (set baud rate divisor)
   outb(PORT + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
   outb(PORT + 1, 0x00);    //                  (hi byte)
   outb(PORT + 3, 0x03);    // 8 bits, no parity, one stop bit
   outb(PORT + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
   outb(PORT + 4, 0x0B);    // IRQs enabled, RTS/DSR set
   outb(PORT + 4, 0x1E);    // Set in loopback mode, test the serial chip
   outb(PORT + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)

   // Check if serial is faulty (i.e: not same byte as sent)
   if(inb(PORT + 0) != 0xAE) {
      return 1;
   }

   // If serial is not faulty set it in normal operation mode
   // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
   outb(PORT + 4, 0x0F);
   return 0;
}
```

##### 发送数据

```c
int is_transmit_empty() {
   return inb(PORT + 5) & 0x20;
}

void write_serial(char a) {
   while (is_transmit_empty() == 0);

   outb(PORT,a);
}
```

##### 接收数据

```c
int serial_received() {
   return inb(PORT + 5) & 1;
}

char read_serial() {
   while (serial_received() == 0);

   return inb(PORT);
}
```

#### 串口驱动的测试

在 `pkg/kernel/src/utils/macros.rs` 中，你可以找到 `print!` 和 `println!` 宏面向串口输出的实现。

在调用 `drivers::serial::init()` 后，如果能够正常看到 `[+] Serial Initialized.` 的输出，说明串口驱动已经成功初始化。

### 日志输出

为了获取更好的日志管理，我们将使用 `log` crate 来进行日志输出，并将其输出接入到前文所实现的串口驱动中。

你可以在 `pkg/kernel/src/utils/logger.rs` 中找到日志输出的相关代码，你需要完成其中的 `init` 函数和 `log` 函数。

> `Logger` 是一个 Zero Sized Types (ZSTs)，在编译之后不会占用任何空间，它更像是一种类型标记，方便我们进行更灵活的操作。

在 `init` 函数中，你需要完成对 `log` crate 的初始化，通过合适的日志级别过滤器来控制日志输出的等级。

在 `log` 函数中，你需要完成对日志输出的实现，你可以选择直接使用 `print!` 和 `println!` 宏进行输出，也可以通过判断日志级别进行不同的格式化、处理。

你可以使用 `self.enabled(record.metadata())` 来判断当前日志级别是否需要输出；也可以用 `record.file_static()` 和 `record.line()` 来获取产生这条日志输出的源文件位置；同时也可以进行不同的格式化、甚至将日志进行命令行着色，从而提高日志的可读性。

以下是进行最常规输出的例子：

```rust
println!("{}", record.args());
```

## 思考题

1. 在 `pkg/kernel` 的 `Cargo.toml` 中，指定了依赖中 `boot` 包为 `default-features = false`，这是为了避免什么问题？请结合 `pkg/boot` 的 `Cargo.toml` 谈谈你的理解。

2. 在 `pkg/boot/src/main.rs` 中参考相关代码，聊聊 `max_phys_addr` 是如何计算的，为什么要这么做？

3. 串口驱动是在进入内核后启用的，那么在进入内核之前，显示的内容是如何输出的？

4. 在 QEMU 中，我们通过指定 `-nographic` 参数来禁用图形界面，这样 QEMU 会默认将串口输出重定向到主机的标准输出。

    - 假如我们将 `Makefile` 中取消该选项，QEMU 的输出窗口会发生什么变化？请观察指令 `make run QEMU_OUTPUT=` 的输出，结合截图分析对应现象。
    - 在移除 `-nographic` 的情况下，如何依然将串口重定向到主机的标准输入输出？请尝试自行构造命令行参数，并查阅 QEMU 的文档，进行实验。
    - 如果你使用 `ysos.py` 来启动 qemu，可以尝试修改 `-o` 选项来实现上述功能。

    !!! note "现象观察提示"

        若此时启动 QEMU 的输出提示是 `vnc server running on ::1:5900`，则说明 QEMU 的图形界面被启用并通过端口 5900 输出。你可以考虑使用 `VNC Viewer` 来观察 QEMU 界面。

        **这一步骤不做要求，如果自身环境实现遇到困难，可以尝试与其他同学合作进行观察。**


## 加分项

1. 😋 线控寄存器的每一比特都有特定的含义，尝试使用 `bitflags` 宏来定义这些标志位，并在 `uart16550` 驱动中使用它们。

2. 😋 尝试在进入内核并初始化串口驱动后，使用 escape sequence 来清屏，并编辑 `get_ascii_header()` 中的字符串常量，输出你的学号信息。

3. 🤔 尝试修改 `logger` 的初始化函数，使得日志等级能够根据编译时的环境变量 `LOG_LEVEL` 来决定编译产物的日志等级。

    !!! note "提示"

        你可以使用 `match option_env!("LOG_LEVEL")` 来判断环境变量的值，它将会在编译时被替换为环境变量的值。

3. 🤔 尝试使用调试器，在内核初始化之后中断，查看、记录并解释如下的信息：

    - 内核的栈指针、栈帧指针、指令指针等寄存器的值。
    - 内核的代码段、数据段、BSS 段等在内存中的位置。

4. 🤔 “开发者是愿意用安全换取灵活的”，所以，我要把代码加载到栈上去，可当我妄图在栈上执行代码的时候，却得到了 `Segment fault`，你能解决这个问题吗？

    请尝试利用 `gcc` 在 Linux 平台上编译一个简单的 C 语言程序，将其编译为 ELF 格式的文件，并尝试在栈上执行它，使它输出 `Hello, world!`。

    !!! question "通过了解 ELF 文件格式、编译链接、内存分页等知识，尝试解决这个问题。"
