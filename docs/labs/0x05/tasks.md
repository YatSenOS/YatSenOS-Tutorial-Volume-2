# 实验五：fork 的实现、并发与锁机制

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## fork 的实现

在操作系统设计中，进程的控制除了创建、终止等基本操作之外，还包括了进程的**复制**。

这种复制的操作可以用于创建**子进程**，被称为 `fork`，它可以使得用户进程具有控制多个进程的能力，从而实现并发执行。

YSOS 的 `fork` 系统调用设计如下描述：

!!! note "出于实验设计考量：<br/>本实现与 Linux 或 [POSIX](https://pubs.opengroup.org/onlinepubs/9699919799/) 中所定义的 `fork` 有所不同，也结合了 Linux 中 `vfork` 的行为。"

- `fork` 会创建一个新的进程，新进程称为子进程，原进程称为父进程。
- 子进程在系统调用后将得到 `0` 的返回值，而父进程将得到子进程的 PID。如果创建失败，父进程将得到 `-1` 的返回值。
- `fork` **不复制**父进程的内存空间，**不实现** Cow (Copy on Write) 机制，即父子进程将持有一定的共享内存：代码段、数据段、堆、bss 段等。
- `fork` 子进程与父进程共享内存空间（页表），但**子进程拥有自己独立的寄存器和栈空间。**
- **由于上述内存分配机制的限制，`fork` 系统调用必须在任何 Rust 内存分配（堆内存分配）之前进行。**

为了实现父子进程的资源共享，在先前的实验中，已经做了一些准备工作：

比如 `pkg/kernel/src/proc/paging.rs` 中，`PageTableContext` 中的 `Cr3RegValue` 被 `Arc` 保护了起来；在 `pkg/kernel/src/proc/data.rs` 中，也存在 `Arc` 包装的共享数据的内容。

??? note "忘了 `Arc` 是什么？"

    `Arc` 是 `alloc::sync` 中的一个原子引用计数智能指针，它允许多个线程同时拥有对同一数据的所有权，且不会造成数据竞争。

    `Arc` 的 `clone()` 方法会增加引用计数，`drop()` 方法会减少引用计数，当引用计数为 0 时，数据会被释放。`Arc` 本身是**不可变的**，但可以通过 `RwLock` 获取内部可变性，进而安全的修改一个被多个线程所持有的数据。

对于 Windows 等将进程抽象为资源容器的操作系统，这些需要共享的资源也就会被抽象为**进程对象**。在这种情况下，实验所设计的行为又更类似于 “新建一个执行线程” 的操作。

### 系统调用

有了上一次实验的经验，系统调用的新增、处理均已经有了一定的经验，此处不过多赘述。对 `fork` 系统调用有如下约定，别忘了在 `syscall_def` 中定义你的系统调用号：

```rust
// None -> pid: u16 or 0 or -1
Syscall::Fork => { /* ... */},
```

!!! tip "如果你和笔者一样有强迫症，Linux 相关功能的系统调用号是 `58`"

### 进程管理

!!! warning "关于具体的实现"

    实验至此，你也应当积累了一些自己的项目管理经验，对于上述的 `FIXME`，你应当有一些自己的想法，用合适的方式进行实现。

    后续的完善将会给出一些提示、建议和注意事项，相关代码结构并不需要**完全按照文档进行**。

    **请注意：每个 `FIXME` 并不代表此功能必须在对应的位置实现，你也应当自由管理相关函数的返回值、参数等。**

在处理好用户态库和系统调用的对接后，参考如下代码，完善你的 `fork`:

!!! note "往下翻翻，说明更多哦（为什么总有人不看完文档就开始写代码！）"

```rust
pub fn fork(context: &mut ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        // FIXME: save_current as parent
        // FIXME: fork to get child
        // FIXME: push to child & parent to ready queue
        // FIXME: switch to next process
    })
}
```

```rust
impl ProcessManager {
    pub fn fork(&self) {
        // FIXME: get current process
        // FIXME: fork to get child
        // FIXME: add child to process list

        // FOR DBG: maybe print the process ready queue?
    }
}
```

```rust
impl Process {
    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        // FIXME: lock inner as write
        // FIXME: inner fork with parent weak ref

        // FOR DBG: maybe print the child process info
        //          e.g. parent, name, pid, etc.

        // FIXME: make the arc of child
        // FIXME: add child to current process's children list
        // FIXME: mark the child as ready & return it
    }
}
```

```rust
pub struct ProcessInner {
    // ...
    parent: Option<Weak<Process>>,
    children: Vec<Arc<Process>>,
    // ...
}

impl ProcessInner {
    pub fn fork(&mut self, parent: Weak<Process>) -> ProcessInner {
        // FIXME: get current process's stack info

        // FIXME: clone the process data struct
        // FIXME: clone the page table context (see instructions)

        // FIXME: alloc & map new stack for child (see instructions)
        // FIXME: copy the *entire stack* from parent to child

        // FIXME: update child's stack frame with new *stack pointer*
        //          > keep lower bits of rsp, update the higher bits
        //          > also update the stack record in process data
        // FIXME: set the return value 0 for child with `context.set_rax`

        // FIXME: construct the child process inner

        // NOTE: return inner because there's no pid record in inner
    }
}
```

关于具体的代码实现，参考如下的提示和说明：

1. 将功能的具体实现委托至下一级进行，保持代码语义的简洁。

    - 系统调用静态函数，并将其委托给 `ProcessManager::fork`。
    - `ProcessManager::fork` 将具体实现委托给当前进程的 `Process::fork`。
    - `Process::fork` 将具体实现委托给 `ProcessInner::fork`。

    每一层代码只关心自己层级的逻辑和数据，不关心持有自身的锁或其他外部数据的状态，进而提高代码可维护性。

2. 使用先前实现的 `save_current` 和 `switch_next` 等函数，提高代码复用性。

    如果使用时遇到了问题，很可能是你的代码过于相互耦合，尝试将逻辑进行分离，保证函数功能的单一性。

3. 利用好函数的返回值等机制，注意相关操作的执行顺序。

4. 使用 `Arc::downgrade` 获取 `Weak` 引用，从而避免循环引用。

    父进程持有子进程的强引用，子进程持有父进程的弱引用，这样可以避免循环引用导致的内存泄漏。

5. 为了复制栈空间，你可以使用 `core::intrinsics::copy_nonoverlapping` 函数。

    这个函数会使用底层 LLVM 所提供的内存复制相关指令，具有较高的性能。需要调用侧保证源和目标的内存空间不会重叠。可以封装为如下函数进行使用：

    ```rust
    /// Clone a range of memory
    ///
    /// - `src_addr`: the address of the source memory
    /// - `dest_addr`: the address of the target memory
    /// - `size`: the count of pages to be cloned
    fn clone_range(src_addr: u64, dest_addr: u64, size: usize) {
        trace!("Clone range: {:#x} -> {:#x}", src_addr, dest_addr);
        unsafe {
            copy_nonoverlapping::<u8>(
                src_addr as *mut u8,
                dest_addr as *mut u8,
                size * Size4KiB::SIZE as usize,
            );
        }
    }
    ```

6. 记录父子进程共用的页表。

    可以使用 `Arc` 来提供引用计数，来确保进程逐个退出时，只有最后一个退出的进程会进行页表内容的释放。为此，你需要补充一些相关的函数调用：

    ```rust
    impl PageTableContext {
        // ...
        pub fn using_count(&self) -> usize {
            Arc::strong_count(&self.reg)
        }

        pub fn fork(&self) -> Self {
            // forked process shares the page table
            Self {
                reg: self.reg.clone(),
            }
        }
        // ...
    }
    ```

    也可以补充一些相关的调试信息：

    ```rust
    impl Debug for PageTableContext {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // ...
                .field("refs", &self.using_count())
            // ...
        }
    }
    ```

8. 为子进程分配合适的栈空间。

    通过子进程数量、页表引用计数、当前父进程的栈等信息，为子进程分配合适的栈空间。

    下面是一个比较常规的期望的栈空间分配结果：

    ```txt
    +---------------------+ <- 0x400000000000
    |    Parent Stack     |
    +---------------------+ <- 0x3FFF00000000
    |    Child 1 Stack    |
    +---------------------+ <- 0x3FFE00000000
    |    Child 2 Stack    |
    +---------------------+ <- 0x3FFD00000000
    |         ...         |
    +---------------------+
    ```

    这样的栈布局在复杂情况下可能会造成栈复用，在这种情况下进行 `map_range` 会失败，从而你可以继续寻找合适的偏移：

    ```rust
    while elf::map_range(/* page range to be mapped */).is_err()
    {
        trace!("Map thread stack to {:#x} failed.", new_stack_base);
        new_stack_base -= STACK_MAX_SIZE; // stack grow down
    }
    ```

    你也可以用 bitmap 等结构体记录栈的释放，或使用其他方式进行合理的分配。

    此处的实现很灵活，也无需完全按照上述栈规划进行，你可以自行对算法或分布进行设计。

    !!! note "更通用的实现"

        在更好的实现中，`fork` 并不复制整个栈，操作系统会启用 `fork` 后全部页面的写保护。

        在任意进程尝试写入时，再对整个页面进行复制。这种策略被称为写时复制（Copy on Write，COW），它可以大大减少内存的使用和开销，提高性能。


### 功能测试

在完成了 `fork` 的实现后，你需要通过如下功能测试来验证你的实现是否正确：

```rust
#![no_std]
#![no_main]

extern crate alloc;
extern crate lib;

use lib::*;

static mut M: u64 = 0xdeadbeef;

fn main() -> usize {
    let mut c = 32;

    let pid = sys_fork();

    if pid == 0 {
        println!("I am the child process");

        assert_eq!(c, 32);

        unsafe {
            println!("child read value of M: {:#x}", M);
            M = 0x2333;
            println!("child changed the value of M: {:#x}", M);
        }

        c += 32;
    } else {
        println!("I am the parent process");

        sys_stat();

        assert_eq!(c, 32);

        println!("Waiting for child to exit...");

        let ret = sys_wait_pid(pid);

        println!("Child exited with status {}", ret);

        assert_eq!(ret, 64);

        unsafe {
            println!("parent read value of M: {:#x}", M);
            assert_eq!(M, 0x2333);
        }

        c += 1024;

        assert_eq!(c, 1056);
    }

    c
}

entry!(main);
```

## 并发与锁机制

由于并发执行时，线程的调度顺序无法预知，进而造成的执行顺序不确定，**持有共享资源的进程之间的并发执行可能会导致数据的不一致**，最终导致相同的程序产生一系列不同的结果，这样的情况被称之为**竞态条件（race condition）**。

!!! tip "条件竞争……？"

    恶意程序利用类似的原理，通过不断地尝试，最终绕过检查，获得了一些不应该被访问的资源，这种对系统的攻击行为也被称为条件竞争。

    > 它们的英文翻译都是 Race Condition，但在不同的领域内常用不同的翻译。

    一个著名的例子是 Linux 内核权限提升漏洞 Dirty COW (CVE-2016-5195)，通过条件竞争使得普通用户可以写入原本只读的内存区域，从而提升权限。

考虑如下的代码：

```rust
static mut COUNTER: usize = 0;

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        handles.push(std::thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", unsafe { COUNTER });
}
```

!!! tip "可以直接使用 `rustc main.rs` 进行编译"

得到的结果如下：

```bash
$ for ((i = 0; i < 16; i++)); do ./main; done
Result: 9595
Result: 8838
Result: 8315
Result: 7602
Result: 9120
Result: 8485
Result: 8831
Result: 8717
Result: 8812
Result: 8955
Result: 9266
Result: 8168
Result: 9159
Result: 10000
Result: 9664
Result: 10000
```

可以看到，每次运行的结果都可能不一样，这是因为 `COUNTER += 1` 操作并不是原子的，它包含了读取、修改和写入三个步骤，而在多线程环境下，这三个步骤之间可能会被其他线程（通过操作系统的时钟中断或其他方式）打断，反汇编上述代码，可以看到 `COUNTER += 1` 的实际操作：

```nasm
mov rax, qword [obj.main::COUNTER::hfb966cd5c23908b7] # read COUNTER to rax
add rax, 1                                            # rax += 1
# ... overflow check by rustc ...
mov qword [obj.main::COUNTER::hfb966cd5c23908b7], rax # write rax to COUNTER
```

考虑如下的执行顺序（实际执行的时钟中断会慢得多，所以上述代码使用循环来凸显这一问题）：

```nasm
# Thread 1
mov rax, qword [obj.main::COUNTER::hfb966cd5c23908b7]
add rax, 1

# !!! Context Switch !!!

# Thread 2
mov rax, qword [obj.main::COUNTER::hfb966cd5c23908b7]

# !!! Context Switch !!!

# Thread 1
mov qword [obj.main::COUNTER::hfb966cd5c23908b7], rax

# !!! Context Switch !!!

# Thread 2
add rax, 1
mov qword [obj.main::COUNTER::hfb966cd5c23908b7], rax
```

在这样的执行顺序下，`COUNTER` 的值会比预期少，几个线程可能会同时读取到相同的值，然后同时写入相同的值，这样的行为就会导致 `+=` 的语意被破坏。

上面这种访问共享资源的代码片段被称为**临界区**，为了保证临界区的正确性，需要确保**每次只有一个线程可以进入临界区**，也即保证这部分指令序列是**互斥**的。

### 原子指令

一般而言，为了解决并发任务带来的问题，需要通过指令集中的原子操作来保证数据的一致性。在 Rust 中，这类原子指令被封装在 `core::sync::atomic` 模块中，作为架构无关的原子操作来提供并发安全性。

以 `AtomicUsize` 为例，它提供了一系列的原子操作，如 `fetch_add`、`fetch_update`、`compare_exchange` 等，这些操作都是原子的，不会被其他线程打断，对于之前的例子：

```rust
static COUNTER: AtomicUsize = AtomicUsize::new(0);
COUNTER.fetch_add(1, Ordering::SeqCst);
```

其中 `Ordering` 用户控制内存顺序，在单核情况下，`Ordering` 的选择并不会影响程序的行为，可以简单了解，并尝试回答思考题 4 的内容。

在编译器优化后将会被编译为：

```nasm
lock inc qword [obj.main::COUNTER::h2889e4585a2a2d30]
```

这就是一句原子的 `inc` 指令，中断或任务切换都不会打断这个指令的执行，从而保证了 `COUNTER` 的一致性。

在了解了原子指令的基本概念后，可以利用它来为用户态程序提供两种简单的同步操作：自旋锁 `SpinLock` 和信号量 `Semaphore`。其中自旋锁的实现并不需要内核态的支持，而信号量则会涉及到进程调度等操作，需要内核态的支持。

正因如此，在进行内核编写的过程中遇到的 `Mutex` 和 `RwLock` 等用于保障内核态数据一致性的锁机制**均是基于自旋锁实现的**，_你可能在之前的实验中遇到过系统因为自旋忙等待导致的异常情况_。

#### 自旋锁

自旋锁 `SpinLock` 是一种简单的锁机制，它通过不断地检查锁的状态来实现线程的阻塞，直到获取到锁为止。

在 `pkg/lib/src/sync.rs` 中，关注 `SpinLock` 的实现：

```rust
pub struct SpinLock {
    bolt: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            bolt: AtomicBool::new(false),
        }
    }

    pub fn acquire(&mut self) {
        // FIXME: acquire the lock, spin if the lock is not available
    }

    pub fn release(&mut self) {
        // FIXME: release the lock
    }
}

// Why? Check reflection question 5
unsafe impl Sync for SpinLock {}
```

在实现 `acquire` 和 `release` 时，你需要使用 `AtomicBool` 的原子操作来保证锁的正确性：

- `load` 函数用于读取当前值。
- `store` 函数用于设置新值。
- `compare_exchange` 函数用户原子得进行比较-交换，也即比较当前值是否为目标值，如果是则将其设置为新值，否则返回当前值。

在进行循环等待时，可以使用 `core::hint::spin_loop` 提高性能，在 x86_64 架构中，它实际上会编译为 `pause` 指令。

#### 信号量

得利于 Rust 良好的底层封装，自旋锁的实现非常简单。但是也存在一定的问题：

- 忙等待：自旋锁会一直占用 CPU 时间，直到获取到锁为止，这会导致 CPU 利用率的下降。
- 饥饿：如果一个线程一直占用锁，其他线程可能会一直无法获取到锁。
- 死锁：如果两个线程互相等待对方占有的锁，就会导致死锁。

信号量 `Semaphore` 是一种更为复杂的同步机制，它可以用于控制对共享资源的访问，也可以用于控制对临界区的访问。通过与进程调度相关的操作，信号量还可以用于控制进程的执行顺序、提高 CPU 利用率等。

信号量需要实现四种操作：

- `new`：根据所给出的 `key` 创建一个新的信号量。
- `remove`：根据所给出的 `key` 删除一个已经存在的信号量。
- `siganl`：也叫做 `V` 操作，也可以被 `release/up/verhogen` 表示，它用于释放一个资源，使得等待的进程可以继续执行。
- `wait`：也叫做 `P` 操作，也可以被 `acquire/down/proberen` 表示，它用于获取一个资源，如果资源不可用，则进程将会被阻塞。

为了实现与内核的交互，信号量的操作将被实现为一个系统调用，它将使用到三个系统调用参数：

```rust
// op: u8, key: u32, val: usize -> ret: any
Syscall::Sem => sys_sem(&args, context),
```

其中 `op` 为操作码，`key` 为信号量的键值，`val` 为信号量的值，`ret` 为返回值。根据先前的约定，`op` 被放置在 `rdi` 寄存器中，`key` 和 `val` 分别被放置在 `rsi` 和 `rdx` 寄存器中，可以通过 `args.arg0`、`args.arg1` 和 `args.arg2` 来进行访问。

信号量相关内容在 `pkg/kernel/src/proc/sync.rs` 中进行实现：

“资源” 被抽象为一个 `usize` 整数，它**并不需要使用 `AtomicUsize`**，为了存储等待的进程，需要在此整数外额外使用一个 `Vec` 来存储等待的进程。它们二者将会被一个自旋锁实现的互斥锁（在内核中直接使用 `spin::Mutex`）保护。

```rust
pub struct Semaphore {
    count: usize,
    wait_queue: VecDeque<ProcessId>,
}
```

信号量操作的结果使用 `SemaphoreResult` 表示：

```rust
pub enum SemaphoreResult {
    Ok,
    NotExist,
    Block(ProcessId),
    WakeUp(ProcessId),
}
```

- `Ok`：表示操作成功，且无需进行阻塞或唤醒。
- `NotExist`：表示信号量不存在。
- `Block(ProcessId)`：表示操作需要阻塞线程，一般是当前进程。
- `WakeUp(ProcessId)`：表示操作需要唤醒线程。

为了实现信号量的 KV 存储，使用 `SemaphoreSet` 定义信号量集合的操作：

```rust
pub struct SemaphoreSet {
    sems: BTreeMap<SemaphoreId, Mutex<Semaphore>>,
}
```

并在 `ProcessData` 中添加为线程共享资源：

```rust
pub struct ProcessData {
    // ...
    pub(super) semaphores: Arc<RwLock<SemaphoreSet>>,
    // ...
}
```

!!! note "关于这里的一堆锁……"

    在本实验实现的单核处理器下，`Semaphore` 的实现似乎并不需要内部的 `Mutex` 进行保护，只需要外部的 `RwLock` 进行保护即可。

    但在多核处理器下，`Semaphore` 的实现可能会涉及到多个核心的并发访问，因此需要使用 `Mutex` 来提供更细粒度的锁保护。在进行添加、删除操作时，对 `RwLock` 使用 `write` 获取写锁，而在进行 `signal`、`wait` 操作时，对 `RwLock` 使用 `read` 来获取更好的性能和控制。

    综上考量，这里就保留了 `Mutex` 的使用。

由于信号量会阻塞进程，所以需要在系统调用的处理中按照信号量的返回值进行相关进程操作，一个代码示例如下：

```rust
pub fn sem_wait(key: u32, context: &mut ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        let pid = processor::current_pid();
        let ret = manager.current().write().sem_wait(key, pid);
        match ret {
            SemaphoreResult::Ok => context.set_rax(0),
            SemaphoreResult::NotExist => context.set_rax(1),
            SemaphoreResult::Block(pid) => {
                // FIXME: save, block it, then switch to next
                //        maybe use `save_current` and `switch_next`
            }
            _ => unreachable!(),
        }
    })
}
```

请参考实验代码给出的相关注释内容，完成信号量的实现、不同操作的系统调用服务实现，最后完善作为系统调用的 `sys_sem`：

```rust
pub fn sys_sem(args: &SyscallArgs, context: &mut ProcessContext) {
    match args.arg0 {
        0 => context.set_rax(new_sem(args.arg1 as u32, args.arg2)),
        1 => context.set_rax(remove_sem(args.arg1 as u32)),
        2 => sem_siganl(args.arg1 as u32, context),
        3 => sem_wait(args.arg1 as u32, context),
        _ => context.set_rax(usize::MAX),
    }
}
```

??? tip "记得完善用户侧 `pkg/lib/src/sync.rs` 中对信号量的操作"

    参考别的用户态函数，如 `pkg/lib/src/io.rs` 的构建。

    使用 `op` 来分配信号量的用户态函数。

### 测试任务

在实现了 `SpinLock` 和 `Semaphore` 的基础上，你需要完成如下的用户程序任务来测试你的实现：

#### 多线程计数器

在所给代码的 `pkg/app/counter` 中实现了一个多线程计数器，多个线程对一个共享的计数器进行累加操作，最终输出计数器的值。

为了提供足够大的可能性来触发竞态条件，该程序使用了一些手段来刻意构造一个临界区，这部分代码不应被修改。

你需要通过上述**两种方式**，分别保护该临界区，使得计数器的值最终为 `800`。

!!! note "尝试修改代码，使用**两组线程**分别测试 `SpinLock` 和 `Semaphore`"

    一个参考代码行为如下：

    ```rust
    fn main() -> isize {
        let pid = sys_fork();

        if pid == 0 {
            test_semaphore();
        } else {
            test_spin();
            sys_wait_pid(pid);
        }

        0
    }
    ```

    你可以在 `test_spin` 和 `test_semaphore` 中分别继续 `fork` 更多的进程用来实际测试。

#### 消息队列

创建一个用户程序 `pkg/app/mq`，结合使用信号量，实现一个消息队列：

- 父进程使用 fork 创建额外 16 个进程，其中一半为生产者，一半为消费者。

- 生产者不断地向消息队列中写入消息，消费者不断地从消息队列中读取消息。

- 每个线程处理的消息总量共 10 条。

    即生产者会产生 10 个消息，每个消费者只消费 10 个消息。

- 在每个线程生产或消费的时候，输出相关的信息。

- 在生产者和消费者完成上述操作后，使用 `sys_exit(0)` 直接退出。

- 最终使用父进程等待全部的子进程退出后，输出消息队列的消息数量。

- 在父进程创建完成 16 个进程后，使用 `sys_stat` 输出当前的全部进程的信息。

你需要保证最终消息队列中的消息数量为 0，你可以开启内核更加详细的日志，并使用输出的相关信息尝试证明队列的正常工作：

- 在从队列取出消息时，消息为空吗？
- 在向队列写入消息时，队列是否满了？
- 在队列为空时，消费者是否被阻塞？
- 在队列满时，生产者是否被阻塞？

#### 哲学家的晚饭

假设有 5 个哲学家，他们的生活只是思考和吃饭。这些哲学家共用一个圆桌，每位都有一把椅子。在桌子中央有一碗米饭，在桌子上放着 5 根筷子。

当一位哲学家思考时，他与其他同事不交流。时而，他会感到饥饿，并试图拿起与他相近的两根筷子（筷子在他和他的左或右邻居之间）。

一个哲学家一次只能拿起一根筷子。显然，他不能从其他哲学家手里拿走筷子。当一个饥饿的哲学家同时拥有两根筷子时，他就能吃。在吃完后，他会放下两根筷子，并开始思考。

创建一个用户程序 `pkg/app/dinner`，使用课上学到的知识，实现并解决哲学家就餐问题：

- 创建一个程序，模拟五个哲学家的行为。

- 每个哲学家都是一个独立的线程，可以同时进行思考和就餐。

- 使用互斥锁来保护每个筷子，确保同一时间只有一个哲学家可以拿起一根筷子。

- 使用等待操作调整哲学家的思考和就餐时间，以增加并发性和实际性。

    - 如果你实现了 `sys_time` 系统调用（Lab 4），可以使用它来构造 `sleep` 操作。
    - 如果你并没有实现它，可以参考多线程计数器中的 `delay` 函数进行实现。

- 当哲学家成功就餐时，输出相关信息，如哲学家编号、就餐时间等。

- 向程序中引入一些随机性，例如在尝试拿筷子时引入一定的延迟，模拟竞争条件和资源争用。

- 可以设置等待时间或循环次数，以确保程序能够运行足够长的时间，并尝试观察到不同的情况，如死锁和饥饿。

??? tip "在用户态中引入伪随机数"

    Rust 提供了一些伪随机数生成器，你可以使用 `rand` 库来引入一些随机性，以模拟不同的情况。

    ```toml
    [dependencies]
    rand = { version = "0.8", default-features = false }
    rand_chacha = { version = "0.3", default-features = false }
    ```

    在无标准库的环境下，你需要为伪随机数生成器提供种子。

    如果你实现了 `sys_time` 系统调用，这会是一个很方便的种子。

    如果你没有实现，不妨试试使用 `sys_getpid` 或者 `fork` 顺序等数据作为种子来生成随机数。

    以 `ChaCha20Rng` 伪随机数生成器为例，使用相关方法获取随机数：

    ```rust
    use rand::prelude::*;
    use rand_chacha::ChaCha20Rng;

    fn main() {
        // ...
        let time = lib::sys_time();
        let mut rng = ChaCha20Rng::seed_from_u64(time.timestamp() as u64);
        println!("Random number: {}", rng.gen::<u64>());
        // ...
    }
    ```

    相关文档请查阅：[The Rust Rand Book](https://rust-random.github.io/book/)

通过观察程序的输出和行为，请尝试构造并截图记录以下现象：

- 某些哲学家能够成功就餐，即同时拿到左右两侧的筷子。
- 尝试构造死锁情况，即所有哲学家都无法同时拿到他们需要的筷子。
- 尝试构造饥饿情况，即某些哲学家无法获得足够的机会就餐。

尝试解决上述可能存在的问题，并介绍你的解决思路。

??? tip "可能的解决思路……"

    分布式系统中，常见的解决思路是引入一个**“服务生”**来协调哲学家的就餐。

    这个服务生会**控制筷子的分配**，从而**避免死锁和饥饿的情况**。

## 思考题

1. 在 Lab 2 中设计输入缓冲区时，如果不使用无锁队列实现，而选择使用 `Mutex` 对一个同步队列进行保护，在编写相关函数时需要注意什么问题？考虑在进行 `pop` 操作过程中遇到串口输入中断的情形，尝试描述遇到问题的场景，并提出解决方案。

2. 在进行 `fork` 的复制内存的过程中，系统的当前页表、进程页表、子进程页表、内核页表等之间的关系是怎样的？在进行内存复制时，需要注意哪些问题？

3. 为什么在实验的实现中，`fork` 系统调用必须在任何 Rust 内存分配（堆内存分配）之前进行？如果在堆内存分配之后进行 `fork`，会有什么问题？

4. 进行原子操作时候的 `Ordering` 参数是什么？此处 Rust 声明的内容与 [C++20 规范](https://en.cppreference.com/w/cpp/atomic/memory_order) 中的一致，尝试搜索并简单了解相关内容，简单介绍该枚举的每个值对应于什么含义。

5. 在实现 `SpinLock` 的时候，为什么需要实现 `Sync` trait？类似的 `Send` trait 又是什么含义？

6. `core::hint::spin_loop` 使用的 `pause` 指令和 Lab 4 中的 `x86_64::instructions::hlt` 指令有什么区别？这里为什么不能使用 `hlt` 指令？

## 加分项

1. 🤔 参考信号量相关系统调用的实现，尝试修改 `waitpid` 系统调用，在进程等待另一个进程退出时进行阻塞，并在目标进程退出后携带返回值唤醒进程。

2. 🤔 尝试实现如下用户程序任务，完成用户程序 `fish`：

    - 创建三个子进程，让它们分别能输出且只能输出 `>`，`<` 和 `_`。
    - 使用学到的方法对这些子进程进行同步，使得打印出的序列总是 `<><_` 和 `><>_` 的组合。

    在完成这一任务的基础上，其他细节可以自行决定如何实现，包括输出长度等。

3. 🤔 尝试和前文不同的其他方法解决哲学家就餐问题，并验证你的方法能够正确解决它，简要介绍你的方法，并给出程序代码和测试结果。
