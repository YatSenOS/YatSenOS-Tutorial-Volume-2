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

- `heap.rs`：添加了 `Heap` 结构体，用于管理堆内存。
- `mod.rs`：除栈外，添加了堆内存、ELF 文件映射的初始化和清理函数。

!!! note "关于 `ProcessVm` 的角色"

    在本实验设计中，`ProcessVm` 结构体用于记录用户程序的内存布局和页表，在调用下级函数前对页表、帧分配器进行获取，从而统一调用 `mapper` 或者 `get_frame_alloc_for_sure` 的时机。

## 帧分配器的内存回收

在 Lab 4 的加分项中，提到了尝试实现帧分配器的内存回收。在本次实验中将进一步完善这一功能。

在进行帧分配器初始化的过程中，内核从 bootloader 获取到了一个 `MemoryMap` 数组，其中包含了所有可用的物理内存区域，并且内核使用 `into_iter()` 将这一数据结构的所有权交给了一个迭代器，你可以在 `pkg/kernel/src/memory/frames.rs` 中了解到相关类型和实现。

迭代器是懒惰的，只有在需要时才会进行计算，因此在进行逐帧分配时，并没有额外的内存开销。但是，当需要进行内存回收时，就需要额外的数据结构来记录已经分配的帧，以便进行再次分配。

相对于真实的操作系统，本实验中的内存回收是很激进的：即能回收时就回收，不考虑回收对性能的影响。在实际的操作系统中，内存回收是一个复杂的问题，需要考虑到内存的碎片化、内存的使用情况、页面的大小等细节；进而使用标记清除、分段等策略来减缓内存回收等频率和碎片化。

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

- `/usr/bin/cat`：ELF 文件映射的内存区域，这里包含了 `cat` 程序的 `.text`、`.data`、`.bss` 等段。
- `[heap]`：堆区的内存区域，这里包含了 `cat` 程序的堆内存，使用 `brk` 系统调用进行分配。
- `/usr/lib/x86_64-linux-gnu/*.so.*`：动态链接库的内存区域，这里包含了程序使用的动态链接库的 `.text`、`.data`、`.bss` 等段。
- `[stack]`：栈区的内存区域，这里包含了程序的栈内存，在达到栈区的最大大小前，栈区会自动增长。
- `[vvar]`、`[vdso]`、`[vsyscall]`：内核的内存区域，这里包含了内核相关的一些数据结构，如 `vvar` 包含了一些内核和用户空间之间共享的变量；`vdso` 是虚拟的动态共享对象 (Virtual Dynamic Shared Object) 区域，用于在用户空间和内核空间之间提供一些系统调用的快速访问。

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

- `VmPeak` / `VmSize`：进程的峰值虚拟内存大小和当前虚拟内存大小，指的是整个虚拟内存空间的大小。
- `VmHWM` / `VmRSS`：进程的峰值物理内存大小和当前物理内存大小，指的是进程实际使用的物理内存大小。
- `RssAnon` / `RssFile` / `RssShmem`：进程的匿名内存、文件映射内存和共享内存的大小。
- `VmData` / `VmStk` / `VmExe`：进程的数据段、栈段、代码段的大小。
- `VmLib`：进程的动态链接库的大小，对于 `cat` 程序来说，这里主要是 `libc` 的占用，但这部分内存可以被多个进程很好地共享。
- `VmPTE`：进程的页表项的大小。

当使用 `ps aux` 查看进程时，你可以看到更多的信息，如进程的 CPU 占用、内存占用、进程的状态等。

```txt
USER         PID %CPU %MEM     VSZ    RSS TTY      STAT START   TIME COMMAND
root           1  0.0  0.3  167780  12876 ?        Ss   Apr22   1:57 /sbin/init
...
mysql       1820  0.1 10.5 1750680 365696 ?        Sl   Apr22  75:39 /url/bin/mysqld
```

其中 VSE 和 RSS 就对应了上述的 `VmSize` 和 `VmRSS`，`%MEM` 表示了 RSS (Resident Set Size) 占总物理内存的百分比。

总之，在 Linux 中进程的虚拟内存大小会比实际使用的物理内存大小要大很多，进程大部分的虚拟内存空间（尤其是文件映射部分）通常被标记为不存在，只有在访问时才会被加载到物理内存中。结合动态链接库的共享，Linux 可以很好得将物理内存物尽其用。

### 内存统计的实现

在目前的实现（Lab 3）中，用户程序在进程结构体中记录的内存区域只有栈区，堆区由内核进行代劳，同时 ELF 文件映射的内存区域也从来没有被释放过，无法被其他程序复用。

而相较于 Linux，本实验的并没有将内存管理抽象为具有上述复杂功能的结构：用户程序的内存占用严格等同于其虚拟内存大小，并且所有页面都会被加载到物理内存中，不存在文件映射等概念，只有堆内存和栈内存是可变的。

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
        let frames_recycled = alloc.recycled_count();
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

## 内核的内存统计

## 内核栈的自动增长

## 用户态堆

## 思考题

1. 当在 Linux 中运行程序的时候删除程序在文件系统中对应的文件，会发生什么？程序能否继续运行？遇到未被映射的内存会发生什么？

## 加分项
