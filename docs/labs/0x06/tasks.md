# 实验六：硬盘驱动与文件系统

!!! danger "在执行每一条命令前，请你对将要进行的操作进行思考"

    **为了你的数据安全和不必要的麻烦，请谨慎使用 `sudo`，并确保你了解每一条指令的含义。**

    **1. 实验文档给出的命令不需要全部执行**

    **2. 不是所有的命令都可以无条件执行**

    **3. 不要直接复制粘贴命令执行**

## 合并实验代码

!!! tip "如何使用本次参考代码"

    本次给出的参考代码为**增量补充**，即在上一次实验的基础上进行修改和补充。因此，你需要将本次参考代码与上一次实验的代码进行合并。

    合并后的代码并不能直接运行，你需要基于合并后的代码、按照文档进行修改补充，才能逐步实现本次实验的功能。

本次实验中提供的代码量较大，但主要部分是类型抽象和相关定义，基本没有函数逻辑，因此不要求理解后开始实验任务，用到某些类型的时候转至定义查询即可。

![](../assets/fs.png)

在所给出的代码中，主要需要补全的内容存放在 `pkg/kernel/src/drivers/ata/bus.rs` 和 `pkg/storage/src/fs/fat16/impls.rs` 中，对应任务为 ATA 磁盘驱动和 FAT 16 文件系统。

在 `pkg/storage/src/common` 中，提供了众多有关存储的底层结构：

- `block.rs`: 提供了数据块的抽象，用于存储数据，内部为指定大小的 `u8` 数组。
- `device.rs`: 目前只提供了块设备的抽象，提供分块读取数据的接口。
- `error.rs`: 定义了文件系统、磁盘、文件名可能遇到的一系列错误，并定义了以 `FsError` 为错误类型的 `Result`。
- `filesystem.rs`: 定义了文件系统的抽象，提供了文件系统的基本操作接口。
- `io.rs`: 定义了 `Read`、`Write` 和 `Seek` 的行为，不过在本次实验中只需要实现 `Read`。
- `metadata.rs`：定义了统一的文件元信息，包含文件名、修改时间、大小等信息。

同时，有了接口定义了统一的行为之后，可以利用他们来实现具有更丰富功能的结构体：

- `filehandle.rs`: 定义了文件句柄，它持有一个实现了 `FileIO` trait 的字段，并维护了文件的元数据。
- `mount.rs`: 定义了挂载点，它持有一个实现了 `Filesystem` trait 的字段，并维护了一个固定的挂载点路径，它会将挂载点路径下的文件操作请求转发给内部的文件系统。

在 `pkg/storage/src/partition/mod.rs` 中，定义了 `Partition` 结构体，和 `PartitionTable` trait，用于统一块设备的分区表行为。

在其他目录下，是需要同学们实现的 MBR 分区表和 FAT 16 文件系统。

在 `pkg/kernel/src/drivers/ata` 中，定义了 ATA 磁盘驱动的相关结构体和接口。

在 `pkg/kernel/src/drivers/filesystem` 中，定义了根文件系统的挂载和初始化等操作。

!!! warning "实验说明"

    作为一套相对独立的模块，存储结构、文件系统相关的内容可以被单独作为一个库进行编译实现。这样方便进行代码复用，并且赋予了对其进行独立测试的能力。

    同时，本次实验专注于实现**文件系统的只读操作**，重点是如何正确解析一个现实存在的文件系统，从而赋予内核直接从磁盘读取文件的能力。

    而文件系统的设计、写入、组织的内容，留作 Lab 8 中的扩展实验进行。

## MBR 分区表

作为熟悉代码结构的起步内容，我们先来实现 MBR 分区表的解析。

MBR（Master Boot Record）是一种磁盘分区表的标准，它位于磁盘的第一个扇区，占用 512 字节。在 MBR 中，有 4 个主分区表项，每个占用 16 字节，用于描述磁盘的分区信息。

之所以称为 “Boot Record”，是因为在 MBR 的定义中中还包含了引导程序的代码，在 Legacy BIOS 系统中，计算机会首先加载 MBR 中的引导程序，然后由引导程序加载操作系统。本实验使用 UEFI 进行引导工作，实际上并不会使用 MBR 中的引导程序。

MBR 分区表的结构如下，可以参考 [MBR - OSDev](https://osdev.org/MBR) 和 [Master Boot Record - wikipedia](https://en.wikipedia.org/wiki/Master_boot_record)：

<table border="2" cellpadding="4" cellspacing="0" class="wikitable"><tbody><tr><th> Offset</th><th> Size (bytes)</th><th> Description</th></tr><tr><td> 0x000</td><td> 440</td><td> MBR <b>Bootstrap</b> (flat binary executable code)</td></tr><tr><td> 0x1B8</td><td> 4</td><td> Optional "Unique Disk ID / Signature"</td></tr><tr><td> 0x1BC</td><td> 2</td><td> Optional, reserved 0x0000</td></tr><tr><td> 0x1BE</td><td> 16</td><td> First partition table entry</td></tr><tr><td> 0x1CE</td><td> 16</td><td> Second partition table entry</td></tr><tr><td> 0x1DE</td><td> 16</td><td> Third partition table entry</td></tr><tr><td> 0x1EE</td><td> 16</td><td> Fourth partition table entry</td></tr><tr><td> 0x1FE</td><td> 2</td><td> (0x55, 0xAA) "Valid bootsector" signature bytes</td></tr></tbody></table>

其中，每个分区表项的结构如下，可以参考 [wikipedia](https://en.wikipedia.org/wiki/Master_boot_record#PTE) 获取更详细的定义：

<table border="2" cellpadding="4" cellspacing="0" class="wikitable"><tbody><tr><th>Offset</th><th>Size (bytes)</th><th>Description</th></tr><tr><td>0x00</td><td>1</td><td>Status (bit 7 set = active or bootable)</td></tr><tr><td>0x01</td><td>3</td><td>CHS Address of partition start</td></tr><tr><td>0x04</td><td>1</td><td>Partition type</td></tr><tr><td>0x05</td><td>3</td><td>CHS address of last partition sector</td></tr><tr><td>0x08</td><td>4</td><td>LBA of partition start</td></tr><tr><td>0x0C</td><td>4</td><td>Number of sectors in partition</td></tr></tbody></table>

在分区表的解析实现中，只需要关心分区表项的解析，不需要关心其他的字段。因此，需要你在 `partition/mbr/mod.rs` 的 `parse` 函数中，根据 MBR 的结构定义，按照对应的偏移量，提取四个 `MbrPartition` 并进行存储。

对于分区表项，需要你在 `partition/mbr/entry.rs` 中，补全对应的结构体定义。

笔者为大家提供了一个便捷的宏：`define_field`，它的定义可以在 `common/macros.rs` 中找到，并且为各位补有文档注释，以做说明如何使用。

同时，这里以 `MbrPartition` 的定义为例子，再做一些解释：

```rust
impl MbrPartition {
    // ...
    define_field!(u8, 0x00, status);
    // ...
}
```

这里的 `define_field!` 宏，接受三个参数，分别是字段的类型、字段的偏移量和字段的名称。它会自动为你生成一个 `status()` 的函数，用于获取字段的值。

字段的类型可以是 `u8`、`u16`、`u32`，分别对应 1、2、4 字节的整数；同时还有 `[u8; n]` 的类型，用于表示固定长度的字节数组，同时也会提供一个对应的从 `&[u8]` 转换为 `&str` 的函数。

你可以在下方的 `Debug` trait 的实现中看到这些函数的使用，你需要补全其中展示的全部函数，并尝试通过文件附带的单元测试。

对于 `0x01-0x03` 和 `0x05-0x07` 两组三字节的内容分别表示了开始和结束的 CHS 地址，包含三组内容：磁头号、扇区号和柱面号，分别占用 8、6 和 10 比特，因此无法使用 `define_field` 进行简单定义，需要你自行实现 `head`、`sector` 和 `cylinder` 所对应的函数，对 `data` 进行解析。

对于后续的磁盘访问，更多通过 LBA 字段进行寻址，实际上并不会用到 CHS 的相关内容。

!!! note "运行单元测试"

    在 Lab 0 中已经简单设计了如何运行单元测试。你可以在 `partition/mbr/entry.rs` 中找到 `tests` 模块，其中包含了测试用例，你可以通过 `cargo test` 来运行它们。

    为了能够单独运行 `mbr` 模块的测试，你可以先注释掉 `lib.rs` 中对其他模块的引用，并处理在 `partition` 中相关需要补全的代码。

    如果想要在测试时看到测试输出，可以使用 `cargo test -- --nocapture` 运行测试，需要注意的是，你应当在 `pkg/storage` 目录下执行，或使用 `--package ysos_storage` 参数指定包名。

## 磁盘驱动

## FAT16 文件系统

## 思考题

1. 为什么在 `pkg/storage/lib.rs` 中声明了 `#![cfg_attr(not(test), no_std)]`，它有什么作用？哪些因素导致了 `kernel` 中进行单元测试是一个相对困难的事情？

## 加分项
