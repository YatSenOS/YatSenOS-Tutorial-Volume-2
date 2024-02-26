# 实验五：fork 的实现、并发与锁机制

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## fork 的实现

在 UNIX 的操作系统设计中，进程的控制除了创建、终止等基本操作之外，还包括了进程的**复制**。

这种复制的操作可以用于创建**子进程**，被称为 `fork`，它可以使得用户进程具有控制多个进程的能力，从而实现并发执行。

YSOS 的 `fork` 系统调用设计如下描述：

!! note "为了实验内容考量，本实现与 Linux 中真实的 `fork` 有所不同，也采取了一些 `vfork` 的做法。"

- `fork` 会创建一个新的进程，新进程称为子进程，原进程称为父进程。
- 子进程在系统调用后将得到 `0` 的返回值，而父进程将得到子进程的 PID。如果创建失败，父进程将得到 `-1` 的返回值。
- `fork` **不复制**父进程的内存空间，**不实现** Cow (Copy on Write) 机制，即父子进程的将持有一定的共享内存（代码段、数据段、堆、bss 段等）。
- `fork` 子进程与父进程共享内存空间（页表），但**子进程拥有自己独立的寄存器和栈空间。**
- **由于内存分配机制的限制，`fork` 系统调用必须在任何 Rust 内存分配（堆内存分配）之前进行。**

为了实现父子进程的资源共享，在先前的实验中，已经做了一些准备工作。比如 `pkg/kernel/src/proc/paging.rs` 中，`PageTableContext` 中的 `Cr3RegValue` 被 `Arc` 保护了起来；在 `pkg/kernel/src/proc/data.rs` 中，也存在 `Arc` 包装的共享数据的内容。

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
// ...
impl ProcessManager {
    pub fn fork(&self) {
        // FIXME: get current process
        // FIXME: fork to get child
        // FIXME: add child to process list

        // FOR DBG: maybe print the process ready queue?
    }
}
// ...
pub struct ProcessInner {
    // ...
    parent: Option<Weak<Process>>,
    children: Vec<Arc<Process>>,
    // ...
}
// ...
impl Process {
    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        // FIXME: lock inner as write
        // FIXME: inner fork with parent weak ref

        // FOR DBG: maybe print the child process info (parent, name, pid, etc.)

        // FIXME: make the arc of child
        // FIXME: add child to current process's children list
        // FIXME: mark the child as ready & return it
    }
}
// ...
impl ProcessInner {
    pub fn fork(&mut self, parent: Weak<Process>) -> ProcessInner {
        // FIXME: get current process's stack info

        // FIXME: clone the process data struct
        // FIXME: clone the page table context (see instructions)

        // FIXME: alloc & map new stack for child (see instructions)
        // FIXME: copy the *entire stack* from parent to child

        // FIXME: update child's stack frame with new *stack pointer*
        //          > keep lower bits of rsp, set the higher bits to the new base
        //          > also update the stack record in process data
        // FIXME: set the return value 0 for child with `context.set_rax`

        // FIXME: construct the child process inner

        // NOTE: return inner because there's no pid record in inner
    }
}
```

关于具体的代码实现，参考如下的提示和说明：

1. 将功能的具体实现委托至下一级进行，保持代码语义的简洁。

    > - 系统调用静态函数，并将其委托给 `ProcessManager::fork`。
    > - `ProcessManager::fork` 将具体实现委托给当前进程的 `Process::fork`。
    > - `Process::fork` 将具体实现委托给 `ProcessInner::fork`。
    >
    > 每一层代码只关心自己层级的逻辑和数据，这样能更好的

2. 使用先前实现的 `save_current` 和 `switch_next` 等函数，提高代码复用性。

    > 如果失败了，很可能是你的代码过于耦合，尝试将逻辑进行分离，保证函数功能的单一性。

3. 利用好函数的返回值等机制，注意相关操作的执行顺序。

4. 使用 `Arc::downgrade` 获取 `Weak` 引用，从而避免循环引用。

    > 父进程持有子进程的强引用，子进程持有父进程的弱引用，这样可以避免循环引用导致的内存泄漏。

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

6. 与 Lab 3 中的多内核线程的栈分配类似，为子进程分配栈空间。

    与 Lab 3 中使用 PID 来计算栈的基址不同，此处使用页表的被引用数量 `strong_count` 来计算栈的基址。

    为此，你需要补充一些相关的函数调用：

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
    impl core::fmt::Debug for PageTableContext {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            // ...
                .field("refs", &self.using_count())
            // ...
        }
    }
    ```

    最终，预期的栈空间分布如下：

    ```txt
    +---------------------------------+ <- 0x400000000000
    |     PageTable 1st Ref Stack     |
    +---------------------------------+ <- 0x3FFF00000000
    |     PageTable 2nd Ref Stack     |
    +---------------------------------+ <- 0x3FFE00000000
    |     PageTable 3rd Ref Stack     |
    +---------------------------------+ <- 0x3FFD00000000
    |               ...               |
    +---------------------------------+
    ```

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

    // we won't copy the heap in `fork`
    // do not alloc heap before it
    // which may cause unexpected behavior (e.g. double free)
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

## 思考题

1. 在 Lab 2 中设计输入缓冲区时，如果不使用无锁队列实现，而选择使用 `Mutex` 对一个同步队列进行保护，在编写相关函数时需要注意什么问题？考虑在进行 `pop` 操作过程中遇到串口输入中断的情形，尝试描述遇到问题的场景，并提出解决方案。

2. 在进行 `fork` 的复制内存的过程中，系统的当前页表、进程页表、子进程页表、内核页表等之间的关系是怎样的？在进行内存复制时，需要注意哪些问题？
