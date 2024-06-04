# 实验七：更好的内存管理

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 合并实验代码

!!! tip "如何使用本次参考代码"

    本次给出的参考代码为**增量补充**，即在上一次实验的基础上进行修改和补充。因此，你需要将本次参考代码与上一次实验的代码进行合并。

    合并后的代码并不能直接运行，你需要基于合并后的代码、按照文档进行修改补充，才能逐步实现本次实验的功能。

本次实验代码量较小，给出的代码集中于 `pkg/kernel/src/proc/vm` 目录下。

-   `heap.rs`：添加了 `Heap` 结构体，用于管理堆内存。
-   `mod.rs`：除栈外，添加了堆内存、ELF 文件映射的初始化和清理函数。

!!! note "关于 `ProcessVm` 的角色"

    在本实验设计中，`ProcessVm` 结构体用于记录用户程序的内存布局和页表，在调用下级函数前对页表、帧分配器进行获取，从而统一调用 `mapper` 或者 `get_frame_alloc_for_sure` 的时机。

## 帧分配器的内存回收

在 Lab 4 的加分项中，提到了尝试实现帧分配器的内存回收。在本次实验中将进一步完善这一功能。

在进行帧分配器初始化的过程中，内核从 bootloader 获取到了一个 `MemoryMap` 数组，其中包含了所有可用的物理内存区域，并且内核使用 `into_iter()` 将这一数据结构的所有权交给了一个迭代器，你可以在 `pkg/kernel/src/memory/frames.rs` 中了解到相关类型和实现。

迭代器是懒惰的，只有在需要时才会进行计算，因此在进行逐帧分配时，并没有额外的内存开销。但是，当需要进行内存回收时，就需要额外的数据结构来记录已经分配的帧，以便进行再次分配。

相对于真实的操作系统，本实验中的内存回收是很激进的：即能回收时就回收，不考虑回收对性能的影响。在实际的操作系统中，内存回收是一个复杂的问题，需要考虑到内存的碎片化、内存的使用情况、页面的大小等细节；进而使用标记清除、分段等策略来减少内存回收的频率和碎片化。

因此对于本实验的帧分配器来说，内存回收的操作是非常简单的，只需要**将已经分配的帧重新加入到可用帧的集合中即可**。

为了减少内存占用，这一操作通常使用位图来实现，即使用一个位图来记录每一帧的分配情况。由于 Rust 的标准库中并没有提供位图的实现，因此你可以简单地使用一个 `Vec<PhysFrame>` 作为已经回收的帧的集合。

!!! note "内存占用"

    使用 `Vec<PhysFrame>` 进行最简单的内存回收记录时，每一页需要使用 8 字节的内存来记录。相对于直接使用位图，这种方法会占用更多的内存（比位图多 64 倍）。

下面进行具体的实现：

```rust
pub struct BootInfoFrameAllocator {
    size: usize,
    frames: BootInfoFrameIter,
    used: usize,
    recycled: Vec<PhysFrame>,
}
```

之后实现分配和回收的方法：

```rust
unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        // FIXME: try pop from recycled frames

        // FIXME: if recycled is empty:
        //        try allocate from frames like before
    }
}

impl FrameDeallocator<Size4KiB> for BootInfoFrameAllocator {
    unsafe fn deallocate_frame(&mut self, _frame: PhysFrame) {
        // FIXME: push frame to recycled
    }
}
```

!!! tip "想要使用位图？"

    如果你想要使用位图来实现页面回收，可以尝试使用 [Roaring Bitmap](https://docs.rs/roaring/)，在笔者 2024 年 2 月的 PR 之后，它可以支持在 `no_std` 环境下使用。

    但它只支持 `u32` 类型的位图，**如何让它在合理范围内记录页面的分配？**

    > 或许你可以假定物理内存的最大大小不会超过一个合理的值……

## 用户程序的内存统计

在进行实际内存统计前，先来了解一下在 Linux 下的用户程序内存布局和管理：

### Linux 的进程内存

在 Linux 中，进程的内存区域可以由下属示意图来简单展示：

```txt
 (high address)
 +---------------------+ <------+ The top of Vmar, the highest address
 |                     |          Randomly padded pages
 +---------------------+ <------+ The base of the initial user stack
 | User stack          |
 |                     |
 +---------||----------+ <------+ The user stack limit / extended lower
 |         \/          |
 | ...                 |
 |                     |
 | MMAP Spaces         |
 |                     |
 | ...                 |
 |         /\          |
 +---------||----------+ <------+ The current program break
 | User heap           |
 |                     |
 +---------------------+ <------+ The original program break
 |                     |          Randomly padded pages
 +---------------------+ <------+ The end of the program's last segment
 |                     |
 | Loaded segments     |
 | .text, .data, .bss  |
 | , etc.              |
 |                     |
 +---------------------+ <------+ The bottom of Vmar at 0x10000
 |                     |          64 KiB unusable space
 +---------------------+
 (low address)
```

你可以在 Linux 中通过 `cat /proc/<pid>/maps` 查看进程的内存映射情况，笔者以 `cat /proc/self/maps` 为例：

```txt
6342f0d69000-6342f0d6b000 r--p 00000000 08:02 792775     /usr/bin/cat
6342f0d6b000-6342f0d70000 r-xp 00002000 08:02 792775     /usr/bin/cat
6342f0d70000-6342f0d72000 r--p 00007000 08:02 792775     /usr/bin/cat
6342f0d72000-6342f0d73000 r--p 00008000 08:02 792775     /usr/bin/cat
6342f0d73000-6342f0d74000 rw-p 00009000 08:02 792775     /usr/bin/cat
6342f1507000-6342f1528000 rw-p 00000000 00:00 0          [heap]
794da0000000-794da02eb000 r--p 00000000 08:02 790013     /usr/lib/locale/locale-archive
794da0400000-794da0428000 r--p 00000000 08:02 789469     /usr/lib/x86_64-linux-gnu/libc.so.6
794da0428000-794da05b0000 r-xp 00028000 08:02 789469     /usr/lib/x86_64-linux-gnu/libc.so.6
794da05b0000-794da05ff000 r--p 001b0000 08:02 789469     /usr/lib/x86_64-linux-gnu/libc.so.6
794da05ff000-794da0603000 r--p 001fe000 08:02 789469     /usr/lib/x86_64-linux-gnu/libc.so.6
794da0603000-794da0605000 rw-p 00202000 08:02 789469     /usr/lib/x86_64-linux-gnu/libc.so.6
794da0605000-794da0612000 rw-p 00000000 00:00 0
794da0665000-794da068a000 rw-p 00000000 00:00 0
794da0698000-794da069a000 rw-p 00000000 00:00 0
794da069a000-794da069b000 r--p 00000000 08:02 789457     /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
794da069b000-794da06c6000 r-xp 00001000 08:02 789457     /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
794da06c6000-794da06d0000 r--p 0002c000 08:02 789457     /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
794da06d0000-794da06d2000 r--p 00036000 08:02 789457     /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
794da06d2000-794da06d4000 rw-p 00038000 08:02 789457     /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
7ffc45730000-7ffc45751000 rw-p 00000000 00:00 0          [stack]
7ffc457d3000-7ffc457d7000 r--p 00000000 00:00 0          [vvar]
7ffc457d7000-7ffc457d9000 r-xp 00000000 00:00 0          [vdso]
ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0  [vsyscall]
```

其中所示的内存区域：

-   `/usr/bin/cat`：ELF 文件映射的内存区域，这里包含了 `cat` 程序的 `.text`、`.data`、`.bss` 等段。
-   `[heap]`：堆区的内存区域，这里包含了 `cat` 程序的堆内存，使用 `brk` 系统调用进行分配。
-   `/usr/lib/x86_64-linux-gnu/*.so.*`：动态链接库的内存区域，这里包含了程序使用的动态链接库的 `.text`、`.data`、`.bss` 等段。
-   `[stack]`：栈区的内存区域，这里包含了程序的栈内存，在达到栈区的最大大小前，栈区会自动增长。
-   `[vvar]`、`[vdso]`、`[vsyscall]`：内核的内存区域，这里包含了内核相关的一些数据结构，如 `vvar` 包含了一些内核和用户空间之间共享的变量；`vdso` 是虚拟的动态共享对象 (Virtual Dynamic Shared Object) 区域，用于在用户空间和内核空间之间提供一些系统调用的快速访问。

你也可以查看程序的内存使用情况，使用 `cat /proc/<pid>/status`，依旧以 `cat` 程序为例：

```txt
VmPeak:	    5828 kB
VmSize:	    5828 kB
VmLck:	       0 kB
VmPin:	       0 kB
VmHWM:	    1792 kB
VmRSS:	    1792 kB
RssAnon:	       0 kB
RssFile:	    1792 kB
RssShmem:	       0 kB
VmData:	     360 kB
VmStk:	     132 kB
VmExe:	      20 kB
VmLib:	    1748 kB
VmPTE:	      52 kB
VmSwap:	       0 kB
```

其中有几个需要注意的字段：

-   `VmPeak` / `VmSize`：进程的峰值虚拟内存大小和当前虚拟内存大小，指的是整个虚拟内存空间的大小。
-   `VmHWM` / `VmRSS`：进程的峰值物理内存大小和当前物理内存大小，指的是进程实际使用的物理内存大小。
-   `RssAnon` / `RssFile` / `RssShmem`：进程的匿名内存、文件映射内存和共享内存的大小。
-   `VmData` / `VmStk` / `VmExe`：进程的数据段、栈段、代码段的大小。
-   `VmLib`：进程的动态链接库的大小，对于 `cat` 程序来说，这里主要是 `libc` 的占用，但这部分内存可以被多个进程很好地共享。
-   `VmPTE`：进程的页表项的大小。

当使用 `ps aux` 查看进程时，你可以看到更多的信息，如进程的 CPU 占用、内存占用、进程的状态等。

```txt
USER         PID %CPU %MEM     VSZ    RSS TTY      STAT START   TIME COMMAND
root           1  0.0  0.3  167780  12876 ?        Ss   Apr22   1:57 /sbin/init
...
mysql       1820  0.1 10.5 1750680 365696 ?        Sl   Apr22  75:39 /url/bin/mysqld
```

其中 VSZ 和 RSS 就对应了上述的 `VmSize` 和 `VmRSS`，`%MEM` 表示了 RSS (Resident Set Size) 占总物理内存的百分比。

总之，在 Linux 中进程的虚拟内存大小会比实际使用的物理内存大小要大很多，进程大部分的虚拟内存空间（尤其是文件映射部分）通常被标记为不存在，只有在访问时才会被加载到物理内存中。结合动态链接库的共享，Linux 可以很好地将物理内存物尽其用。

### 内存统计的实现

在目前的实现（Lab 3）中，用户程序在进程结构体中记录的内存区域只有栈区，堆区由内核进行代劳，同时 ELF 文件映射的内存区域也从来没有被释放过，无法被其他程序复用。

而相较于 Linux，本实验并没有将内存管理抽象为具有上述复杂功能的结构：用户程序的内存占用严格等同于其虚拟内存大小，并且所有页面都会被加载到物理内存中，不存在文件映射等概念，只有堆内存和栈内存是可变的。

因此，其内存统计并没有那么多的细节，只需要统计用户程序的栈区和堆区的大小即可。在 `Stack` 和 `Heap` 中，已经实现了 `memory_usage` 函数来获取栈区和堆区的内存占用字节数。

```rust
impl Stack {
    pub fn memory_usage(&self) -> u64 {
        self.usage * crate::memory::PAGE_SIZE
    }
}

impl Heap {
    pub fn memory_usage(&self) -> u64 {
        self.end.load(Ordering::Relaxed) - self.base.as_u64()
    }
}
```

!!! note "堆区的内存管理将在本实验后部分实现，不过此处可以直接将 `Heap` 先行加入到进程结构体中"

那么根据上述讨论，对本实验的内存占用而言，只剩下了 ELF 文件映射的内存区域和页表的内存占用，为实现简单，本部分忽略页表的内存占用，只统计 ELF 文件映射的内存占用。

```rust
pub struct ProcessVm {
    // page table is shared by parent and child
    pub(super) page_table: PageTableContext,

    // stack is pre-process allocated
    pub(super) stack: Stack,

    // heap is allocated by brk syscall
    pub(super) heap: Heap,

    // code is hold by the first process
    // these fields will be empty for other processes
    pub(super) code: Vec<PageRangeInclusive>,
    pub(super) code_usage: u64,
}

impl ProcessVm {
    pub(super) fn memory_usage(&self) -> u64 {
        self.stack.memory_usage() + self.heap.memory_usage() + self.code_usage
    }
}
```

获取用户程序 ELF 文件映射的内存占用的最好方法是在加载 ELF 文件时记录内存占用，这需要对 `elf` 模块中的 `load_elf` 函数进行修改：

```rust
pub fn load_elf(
    elf: &ElfFile,
    physical_offset: u64,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    user_access: bool,
) -> Result<Vec<PageRangeInclusive>, MapToError<Size4KiB>> {
    trace!("Loading ELF file...{:?}", elf.input.as_ptr());

    // use iterator and functional programming to load segments
    // and collect the loaded pages into a vector
    elf.program_iter()
        .filter(|segment| segment.get_type().unwrap() == program::Type::Load)
        .map(|segment| {
            load_segment(
                elf,
                physical_offset,
                &segment,
                page_table,
                frame_allocator,
                user_access,
            )
        })
        .collect()
}

// load segments to new allocated frames
fn load_segment(
    elf: &ElfFile,
    physical_offset: u64,
    segment: &program::ProgramHeader,
    page_table: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    user_access: bool,
) -> Result<PageRangeInclusive, MapToError<Size4KiB>> {
    let virt_start_addr = VirtAddr::new(segment.virtual_addr());
    let start_page = Page::containing_address(virt_start_addr);

    // ...

    let end_page = Page::containing_address(virt_start_addr + mem_size - 1u64);
    Ok(Page::range_inclusive(start_page, end_page))
}
```

之后，在 `pkg/kernel/src/proc/vm` 中完善 `ProcessVm` 的 `load_elf_code` 函数，在加载 ELF 文件时记录内存占用。

为了便于测试和观察，在 `pkg/kernel/src/proc/manager.rs` 的 `print_process_list` 和 `Process` 的 `fmt` 实现中，添加打印内存占用的功能。

```rust
impl ProcessManager {
    pub fn print_process_list(&self) {
        let mut output =
            String::from("  PID | PPID | Process Name |  Ticks  |   Memory  | Status\n");

        // ...

        // NOTE: print memory page usage
        //      (you may implement following functions)
        let alloc = get_frame_alloc_for_sure();
        let frames_used = alloc.frames_used();
        let frames_recycled = alloc.frames_recycled();
        let frames_total = alloc.frames_total();

        let used = (frames_used - frames_recycled) * PAGE_SIZE as usize;
        let total = frames_total * PAGE_SIZE as usize;

        output += &format_usage("Memory", used, total);
        drop(alloc);

        // ...
    }
}

// A helper function to format memory usage
fn format_usage(name: &str, used: usize, total: usize) -> String {
    let (used_float, used_unit) = humanized_size(used as u64);
    let (total_float, total_unit) = humanized_size(total as u64);

    format!(
        "{:<6} : {:>6.*} {:>3} / {:>6.*} {:>3} ({:>5.2}%)\n",
        name,
        2,
        used_float,
        used_unit,
        2,
        total_float,
        total_unit,
        used as f32 / total as f32 * 100.0
    )
}
```

```rust
impl core::fmt::Display for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let inner = self.inner.read();
        let (size, unit) = humanized_size(inner.proc_vm.as_ref().map_or(0, |vm| vm.memory_usage()));
        write!(
            f,
            " #{:-3} | #{:-3} | {:12} | {:7} | {:>5.1} {} | {:?}",
            self.pid.0,
            inner.parent().map(|p| p.pid.0).unwrap_or(0),
            inner.name,
            inner.ticks_passed,
            size,
            unit,
            inner.status
        )?;
        Ok(())
    }
}
```

!!! success "阶段性成果"

    使用你实现的 Shell 打印进程列表和他们的内存占用。

    使用 `fact` 阶乘递归程序进行测试，在其中使用 `sys_stat` 系统调用打印进程信息，尝试观察到内存占用的变化。

## 用户程序的内存释放

在经过上述的讨论和实现后，目前进程的内存管理已经包含了栈区、堆区和 ELF 文件映射三部分，但是在进程退出时，这些内存区域并没有被释放，内存没有被回收，无法被其他程序复用。

不过，在实现了帧分配器的内存回收、进程的内存统计后，进程退出时的内存释放也将得以实现。

### 页表上下文的伏笔

!!! tip "关于本实验的内存模型……"

    还记得 `fork` 的实现吗？在 Linux 等真实世界的操作系统中，`fork` 不会真正复制物理内存，而是使用写时复制 (Copy-On-Write) 的技术，只有在子进程或父进程修改内存时才会进行复制。

    但在本实验中，出于并发等实验功能的实现简易性，`fork` 得到的进程与父进程**共享页面和页表**，因此在释放内存时要注意不要影响到其他进程。

`PageTableContext` 是自 Lab 3 就给出的页表上下文结构，如果你注意过它的定义，或许会发现自那时就埋下的伏笔——没印象也无妨，它的定义如下所示：

```rust
pub struct PageTableContext {
    pub reg: Arc<Cr3RegValue>,
}
```

> 笔者打赌绝大部分的读者可能会是第一次打开 `proc/paging.rs` 文件……

在目前的项目中，你应当能看到它实现的两个不同的方法：`clone_level_4` 和 `fork`：

```rust
pub fn clone_level_4(&self) -> Self {
    // 1. alloc new page table
    // ...

    // 2. copy current page table to new page table
    // ...

    // 3. create page table
    Self {
        reg: Arc::new(
            Cr3RegValue::new(page_table_addr, Cr3Flags::empty())
        ),
    }
}

pub fn fork(&self) -> Self {
    // forked process shares the page table
    Self {
        reg: self.reg.clone(),
    }
}
```

-   `clone_level_4` 用于复制当前页表（仅第四级页表），并将其作为一个新的页表上下文返回，用于 `spawn` 时复制内核页表；
-   `fork` 则是直接对 `Arc<Cr3RegValue>` 进行 `clone` 操作，使得新的进程与父进程共享页表。

也即，在目前的实现中，对于每一棵独立的“进程树”，它们的页表是独立的，但是在同一棵“进程树”中，它们的页表是共享的。

> 这里仅是一个简单的表示，本实验并没有真正去记录有关“树”的信息，只是为了方便理解。

既然 `Arc` 表示“原子引用计数”，也就意味着可以通过它来确定**”当前页表被多少个进程共享“**，从而在释放内存时，只有在最后一个进程退出时才释放共享的内存。

为 `PageTableContext` 添加一个 `using_count` 方法，用于获取当前页表被引用的次数：

```rust
pub fn using_count(&self) -> usize {
    Arc::strong_count(&self.reg)
}
```

### 内存释放的实现

出于模块化设计，先为 `Stack` 实现 `clean_up` 函数，由于栈是一块连续的内存区域，且进程间不共享栈区，因此在进程退出时直接释放栈区的页面即可。

```rust
impl Stack {
    pub fn clean_up(
        &mut self,
        // following types are defined in
        //   `pkg/kernel/src/proc/vm/mod.rs`
        mapper: MapperRef,
        dealloc: FrameAllocatorRef,
    ) -> Result<(), UnmapError> {
        if self.usage == 0 {
            warn!("Stack is empty, no need to clean up.");
            return Ok(());
        }

        // FIXME: unmap stack pages with `elf::unmap_pages`

        self.usage = 0;

        Ok(())
    }
}
```

接下来重点关注 `ProcessVm` 的相关实现，位于 `pkg/kernel/src/proc/vm/mod.rs` 中，首先为它添加 `clean_up` 函数：

```rust
impl ProcessVm {
    pub(super) fn clean_up(&mut self) -> Result<(), UnmapError> {
        let mapper = &mut self.page_table.mapper();
        let dealloc = &mut *get_frame_alloc_for_sure();

        // statistics for logging and debugging
        // NOTE: you may need to implement `frames_recycled` by yourself
        let start_count = dealloc.frames_recycled();

        // TODO...

        // statistics for logging and debugging
        let end_count = dealloc.frames_recycled();

        debug!(
            "Recycled {}({:.3} MiB) frames, {}({:.3} MiB) frames in total.",
            end_count - start_count,
            ((end_count - start_count) * 4) as f32 / 1024.0,
            end_count,
            (end_count * 4) as f32 / 1024.0
        );

        Ok(())
    }
}
```

在上述框架之上，按照顺序依次释放栈区、堆区和 ELF 文件映射的内存区域：

1. 释放栈区：调用 `Stack` 的 `clean_up` 函数；
2. 如果**当前页表被引用次数为 1**，则进行共享内存的释放，否则跳过至第 7 步；
3. 释放堆区：调用 `Heap` 的 `clean_up` 函数（后续实现）；
4. 释放 ELF 文件映射的内存区域：根据记录的 `code` 页面范围数组，依次调用 `elf::unmap_range` 函数，并进行页面回收。
5. 清理页表：调用 `mapper` 的 `clean_up` 函数，这将清空全部**无页面映射的**一至三级页表。
6. 清理四级页表：直接回收 `PageTableContext` 的 `reg.addr` 所指向的页面。
7. 统计内存回收情况，并打印调试信息。

对于第 5 和第 6 步，可以参考使用如下代码：

```rust
unsafe {
    // free P1-P3
    mapper.clean_up(dealloc);

    // free P4
    dealloc.deallocate_frame(self.page_table.reg.addr);
}
```

最后，遵守 Rust 的内存管理规则，需要在 `Process` 的 `Drop` 实现中调用 `ProcessVm` 的 `clean_up` 函数：

```rust
impl Drop for ProcessVm {
    fn drop(&mut self) {
        if let Err(err) = self.clean_up() {
            error!("Failed to clean up process memory: {:?}", err);
        }
    }
}
```

在实现了 `Drop` 之后，你可以在 `Process` 的 `kill` 函数中直接使用 `take` 来释放进程的内存：

```rust
// consume the Option<ProcessVm> and drop it
self.proc_vm.take();
```

!!! success "阶段性成果"

    使用你实现的 Shell 运行 `fact` 阶乘递归程序，观察进程的内存占用和**释放**情况。

    在 `fact` 程序中，尝试使用 `sys_stat` 系统调用打印进程信息，观察到**内存占用的变化**。

    > 你的页面被成功回收了吗？

## 内核的内存统计

至此，用户程序的内存管理已经得到了较好的实现，但是内核占用了多少内存呢？

类似于用户进程的加载过程，可以通过在内核加载时记录内存占用来实现内核的初步内存统计，即在 bootloader 中实现这一功能。

首先，在 `pkg/boot/src/lib.rs` 中，定义一个 `KernelPages` 类型，用于传递内核的内存占用信息，并将其添加到 `BootInfo` 结构体的定义中：

```rust
pub type KernelPages = ArrayVec<PageRangeInclusive, 8>;

pub struct BootInfo {
    // ...

    // Kernel pages
    pub kernel_pages: KernelPages,
}
```

并在 `pkg/boot/src/main.rs` 中，将 `load_elf` 函数返回的内存占用信息传递至 `BootInfo` 结构体中：

??? note "使用了其他函数加载内核？"

    如果你跟着实验指南一步一步实现，那么你的内核加载函数应当是 `load_elf`，它通过分配新的帧、映射它们、复制数据的顺序来进行加载。

    如果你使用了参考实现提供的代码，这里可能会有所不同：参考实现中使用 `map_elf` 来实现内核页面的映射**它不再新分配帧**，而是对 ELF 文件中的页面**直接映射**，因此你需要根据实际情况来获取内核被加载的页面信息。

    作为参考，可以使用如下代码直接从 `ElfFile` 中获取内核被加载的页面信息：

    ```rust
    pub fn get_page_usage(elf: &ElfFile) -> KernelPages {
        elf.program_iter()
            .filter(|segment| segment.get_type() == Ok(xmas_elf::program::Type::Load))
            .map(|segment| get_page_range(segment))
            .collect()
    }
    ```

成功加载映射信息后，将其作为 `ProcessManager` 的初始化参数，用于构建 `kernel` 进程：

```rust
/// init process manager
pub fn init(boot_info: &'static boot::BootInfo) {
    // FIXME: you may need to implement `init_kernel_vm` by yourself
    let proc_vm = ProcessVm::new(PageTableContext::new()).init_kernel_vm(&boot_info.kernel_pages);

    trace!("Init kernel vm: {:#?}", proc_vm);

    // kernel process
    let kproc = Process::new(String::from("kernel"), None, Some(proc_vm), None);

    kproc.write().resume();
    manager::init(kproc);

    info!("Process Manager Initialized.");
}
```

其中，为 `ProcessVm` 添加 `init_kernel_vm` 函数，用于初始化内核的内存布局：

```rust
pub fn init_kernel_vm(mut self, pages: &KernelPages) -> Self {
    // FIXME: load `self.code` and `self.code_usage` from `pages`

    // FIXME: init kernel stack (impl the const `kstack` function)
    //        `pub const fn kstack() -> Self`
    //         use consts to init stack, same with kernel config
    self.stack = Stack::kstack();

    self
}
```

在进行后续实验的过程中，将会继续对 `ksatck` 函数进行修改，这里可以直接使用配置文件中指定的常量来初始化，或者先行忽略。

!!! success "阶段性成果"

    试使用 `sys_stat` 系统调用打印进程信息，观察内核内存的占用情况。

## 内核栈的自动增长

在 Lab 3 中简单实现了用户进程的栈区自动增长，但是内核的栈区并没有进行相应的处理，这将导致内核栈溢出时无法进行自动增长，从而导致内核崩溃。

为了在之前的实验中避免这种情况，实验通过 bootloader 直接为内核分配了 512 \* 4 KiB = 2 MiB 的栈区来避免可能的栈溢出问题。但这明显是不合理的，因为内核的栈区并不需要这么大的空间。

与其分配一个固定大小的栈区，不如在缺页中断的基础上实现一个简单的栈区自动增长机制，当栈区溢出时，自动为其分配新的页面。

需要用到的配置项在 Lab 1 中已经给出，即 `kernel_stack_auto_grow`，对它的行为进行如下约定：

-   默认为 `0`，这时内核栈区所需的全部页面（页面数量为 `kernel_stack_size`）将会在内核加载时一次性分配。
-   当这一参数为非零值时，表示内核栈区的初始化页面数量，从栈顶开始向下分配这一数量的初始化页面，并交由内核进行自己的栈区管理。

```rust
let (stack_start, stack_size) = if config.kernel_stack_auto_grow > 0 {
    let init_size = config.kernel_stack_auto_grow;
    let bottom_offset = (config.kernel_stack_size - init_size) * 0x1000;
    let init_bottom = config.kernel_stack_address + bottom_offset;
    (init_bottom, init_size)
} else {
    (config.kernel_stack_address, config.kernel_stack_size)
};
```

与用户态栈类似，你可以在 `pkg/kernel/src/proc/vm/stack.rs` 中将这些信息定义为常量，并在 `Stack` 的 `kstack` 函数中使用这些常量来初始化内核栈区：

```rust
// [bot..0xffffff0100000000..top..0xffffff01ffffffff]
// kernel stack
pub const KSTACK_MAX: u64 = 0xffff_ff02_0000_0000;
pub const KSTACK_DEF_BOT: u64 = KSTACK_MAX - STACK_MAX_SIZE;
pub const KSTACK_DEF_PAGE: u64 = 8;
pub const KSTACK_DEF_SIZE: u64 = KSTACK_DEF_PAGE * crate::memory::PAGE_SIZE;

pub const KSTACK_INIT_BOT: u64 = KSTACK_MAX - KSTACK_DEF_SIZE;
pub const KSTACK_INIT_TOP: u64 = KSTACK_MAX - 8;
```

!!! warning "别忘了修改配置文件使其描述的区域一致！"

    对于上述的常量，你应当在配置文件中这样修改，其中 `kernel_stack_auto_grow` 的取值视实现可能有所不同：

    ```toml
    # The size of the kernel stack, given in number of 4KiB pages.
    kernel_stack_size=1048576

    # Define if the kernel stack will auto grow (handled by kernel).
    kernel_stack_auto_grow=8
    ```

最后，在缺页中断的处理过程中，对权限、区域进行判断。如果发生缺页中断的进程是内核进程则**不要设置用户权限标志位**，并进行日志记录：

```rust
info!("Page fault on kernel at {:#x}", addr);
```

最后，为了测试你的栈扩容成果，可以用如下代码在 `pkg/kernel/src/lib.rs` 中进行测试：

```rust
pub fn init(boot_info: &'static BootInfo) {
    // ...

    info!("Test stack grow.");

    grow_stack();

    info!("Stack grow test done.");
}

#[no_mangle]
#[inline(never)]
pub fn grow_stack() {
    const STACK_SIZE: usize = 1024 * 4;
    const STEP: usize = 64;

    let mut array = [0u64; STACK_SIZE];
    info!("Stack: {:?}", array.as_ptr());

    // test write
    for i in (0..STACK_SIZE).step_by(STEP) {
        array[i] = i as u64;
    }

    // test read
    for i in (0..STACK_SIZE).step_by(STEP) {
        assert_eq!(array[i], i as u64);
    }
}
```

!!! success "阶段性成果"

    尝试能使你的内核启动的最小的 `kernel_stack_auto_grow` 值，观察内核栈的自动增长情况。

    **并尝试回答思考题 3，它或许会对你的理解有所帮助。**

## 用户态堆

最后，为了提供给用户程序更多的内存管理能力，还需要实现一个系统调用：`sys_brk`，用于调整用户程序的堆区大小。

!!! note "关于 `brk` 系统调用……"

    `brk` 系统调用是一个古老的系统调用，本意为调整 Program Break（程序断点）指针的位置，该指针最初指进程的数据段末尾，但这一断点可以向上增长，进而留出灵活可控的空间作为“堆内存”。

    > 那句老话：“堆向高地址增长，栈向低地址增长”。你可以在本实验开头的 “Linux 进程内存” 部分中找到它。

    而 `brk` 系统调用则是用于调整这一断点的位置，从而调整堆区的大小。在开启地址随机化后，它在初始化时会被加上一个随机的偏移量，从而使得堆区的地址不再是固定的。

    在 C 中，提供了 `brk` 和 `sbrk` 两个函数来调用这一系统调用，在现代的 Linux 中，`brk` 系统调用的功能已经逐渐被更灵活的 `mmap` 系统调用所取代。

    但是在本实验中，为了简化内存管理的实现，仍然使用 `brk` 系统调用来调整用户程序的堆区大小，进而为后续可能的实验提供基础。

首先，参考给出代码中的 `pkg/kernel/src/proc/vm/heap.rs`：

```rust
// user process runtime heap
// 0x100000000 bytes -> 4GiB
// from 0x0000_2000_0000_0000 to 0x0000_2000_ffff_fff8
pub const HEAP_START: u64 = 0x2000_0000_0000;
pub const HEAP_PAGES: u64 = 0x100000;
pub const HEAP_SIZE: u64 = HEAP_PAGES * crate::memory::PAGE_SIZE;
pub const HEAP_END: u64 = HEAP_START + HEAP_SIZE - 8;

/// User process runtime heap
///
/// always page aligned, the range is [base, end)
pub struct Heap {
    /// the base address of the heap
    ///
    /// immutable after initialization
    base: VirtAddr,

    /// the current end address of the heap
    ///
    /// use atomic to allow multiple threads to access the heap
    end: Arc<AtomicU64>,
}
```

在 `Heap` 中，`base` 表示堆区的起始地址，`end` 表示堆区的结束地址，`end` 是一个 `Arc<AtomicU64>` 类型的原子变量，因此它在多个进程的操作中被并发访问。

> 也就是说，用户程序的堆区是在父子进程之间共享的，`fork` 时不需要复制堆区内容，只需要复制 `Heap` 结构体即可。

在本实验设计中，堆区的最大大小固定、起始地址固定，堆区的大小由 `end` 变量来控制，当用户程序调用 `brk` 系统调用时，内核会根据用户程序传入的参数来调整 `end` 的值，并进行相应的页面映射，从而调整堆区的大小。

> 如果你还是想和 Linux 对齐，`brk` 系统调用的调用号为 12。

下面对 `brk` 系统调用的参数和行为进行简单的约定。

在用户态中，考虑下列系统调用函数封装：`brk` 系统调用的参数是一个可为 `None` 的指针，表示用户程序希望调整的堆区结束地址，用户参数采用 `0` 表示 `None`，返回值采用 `-1` 表示操作失败。

```rust
#[inline(always)]
pub fn sys_brk(addr: Option<usize>) -> Option<usize> {
    const BRK_FAILED: usize = !0;
    match syscall!(Syscall::Brk, addr.unwrap_or(0)) {
        BRK_FAILED => None,
        ret => Some(ret),
    }
}
```

在内核中，`brk` 系统调用的处理函数如下：将用户传入的参数转换为内核的 `Option<VirtAddr>` 类型进行传递，并使用相同类型作为返回值。

```rust
// in `pkg/kernel/src/syscall/service.rs`
pub fn sys_brk(args: &SyscallArgs) -> usize {
    let new_heap_end = if args.arg0 == 0 {
        None
    } else {
        Some(VirtAddr::new(args.arg0 as u64))
    };
    match brk(addr) {
        Some(addr) => addr.as_u64() as usize,
        None => !0,
    }
}

// in `pkg/kernel/src/proc/mod.rs`
pub fn brk(addr: Option<VirtAddr>) -> Option<VirtAddr> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        // NOTE: `brk` does not need to get write lock
        get_process_manager().current().read().brk(addr)
    })
}
```

对于 `brk` 系统调用的具体实现，你需要在 `pkg/kernel/src/proc/vm/heap.rs` 中为 `Heap` 结构体实现 `brk` 函数：

-   如果参数为 `None`，则表示用户程序希望获取当前的堆区结束地址，即返回 `end` 的值；
-   如果用户程序传入的参数不为 `None`，则检查用户传入的地址是否合法，即在 `[HEAP_START, HEAP_END]` 区间内，如果不合法则返回 `None`。

对于有效输入的处理，需要满足如下约定：

-   初始化堆区时，`base` 和 `end` 的值均为 `HEAP_START`；
-   用户希望释放整个堆区：传入地址为 `base`，释放所有页面，`end` 重置为 `base`；
-   用户希望缩小堆区：传入地址比当前 `end` 小，对目的地址向上对齐到页边界，释放多余的页面；
-   用户希望扩大堆区：传入地址比当前 `end` 大，对目的地址向上对齐到页边界，分配新的页面。

对于一段典型的系统调用过程，可以参考如下代码：

```rust
let heap_start = sys_brk(None).unwrap();
let heap_end = heap_start + HEAP_SIZE;

let ret = sys_brk(Some(heap_end)).expect("Failed to allocate heap");

assert!(ret == heap_end, "Failed to allocate heap");
```

最后，别忘了为 `Heap` 实现 `clean_up` 函数，用于释放堆区的页面，对于连续的堆区页面释放可以参考 `Stack` 进行实现，这里不再赘述。

在实现了 `sys_brk` 系统调用后，你可以在用户程序中使用 `brk` 系统调用来调整堆区的大小，从而实现用户程序的内存管理。

如果直接替换现有的用户态堆分配，则很难找出可能存在的问题，因此下面给出一个测试和实现流程作为参考：

1. 新建一个用户程序，参考上述代码，尝试在其中使用 `brk` 系统调用来调整堆区的大小，并进行写入和读取操作；

2. 若上述操作没有问题，则可以在 `lib` 中实现可选的第二个内存分配器（参考给出代码 `pkg/lib/src/allocator/brk.rs`）；

    内存分配器的自主实现不是本次实验的内容，因此这里直接使用 `linked_list_allocator` 进行代劳。

    在后续的实验中，如果你想要自行实现内存管理算法，可以参考给出的方式添加 `feature` 对代码进行隔离，以便于测试和调试。

3. 尝试在进程中使用如下方式来暂时使用新的内存分配器：

    ```diff
    [dependencies]
    - lib = { package = "yslib", path = "../../lib" }

    + [dependencies.lib]
    + package = "yslib"
    + path = "../../lib"
    + default-features = false
    + features = ["brk_alloc"]
    ```

4. 在你测试通过后，可以将其作为默认的内存分配器：

    ```diff
    [features]
    - default = ["kernel_alloc"]
    + default = ["brk_alloc"]
    ```

如果想要实现一系列操作的自主测试，可以在自定义的用户程序中进行一系列的操作，或者直接将其实现为接受用户输入的 Shell 命令，进一步测试并记录你的 `brk` 系统调用的行为。

!!! success "阶段性成果"

    你应该能够使用新的内存分配器来让之前的每个用户程序正常执行了。

## 思考题

1. 当在 Linux 中运行程序的时候删除程序在文件系统中对应的文件，会发生什么？程序能否继续运行？遇到未被映射的内存会发生什么？

2. 为什么要通过 `Arc::strong_count` 来获取 `Arc` 的引用计数？查看它的定义，它和一般使用 `&self` 的方法有什么不同？出于什么考虑不能直接通过 `&self` 来进行这一操作？

3. bootloader 加载内核并为其分配初始栈区时，至少需要多少页内存才能保证内核正常运行？

    尝试逐渐增大内核的栈区大小，观察内核的运行情况，对于**不能正常启动的情况**，尝试分析可能的原因。

    _提示：内核实现缺页中断的处理时，依赖于哪些子系统？报错是什么？什么子系统可能会导致对应的问题？_

4. 尝试查找资料，了解 `mmap`、`munmap` 和 `mprotect` 系统调用的功能和用法，回答下列问题：

    - `mmap` 的主要功能是什么？它可以实现哪些常见的内存管理操作？

    - `munmap` 的主要功能是什么？什么时候需要使用 `munmap`？

    - `mprotect` 的主要功能是什么？使用 `mprotect` 可以实现哪些内存保护操作？
    - 编写 C 程序，使用 `mmap` 将一个文件映射到内存中，并读写该文件的内容。

        _思考：文件内容什么时候会被写入到磁盘？_

    - 综合考虑有关内存、文件、I/O 等方面的知识，讨论为什么 `mmap` 系统调用在现代操作系统中越来越受欢迎，它具有哪些优势？
