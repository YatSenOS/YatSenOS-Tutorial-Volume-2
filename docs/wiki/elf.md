# ELF 文件格式

!!! note "请确保你已经了解了什么是**虚拟内存**，可以参考 [分页内存简述](./paging.md)"

ELF（Executable and Linkable Format）是 Unix 系统实验室在与 Sun Microsystems 合作开发 SVR4（UNIX System V Release 4.0）时设计的。因此，ELF 首次出现在基于 SVR4 的 Solaris 2.0（又称 SunOS 5.0）中。该格式是在 System V ABI 中指定的。这是一种用途非常广泛的文件格式，既可用作可执行文件，也可用作共享库文件。

## ELF 文件概述

ELF 文件大体上由文件头和数据组成，它还可以加上额外的调试信息。

> 事实上大部分的文件都是类似的结构

一般来说，ELF 有以下几个部分

- ELF 文件头
- Section header table，为 relocatable files 所必须，loadable files 可选，链接器需要 Section Table 进行链接
- Program header table，为 loadable files 所必需，但 relocatable files 可选，Program header table 描述了所有可加载的 segments 和其他数据结构，这或许会是我们遇见最多的
- 有文件头还得有内容，即 section 和 segment，这包括了各种可加载的数据，字符串表，符号表等等。每个 segment 里可以包含多个 sections。

### ELF Header

ELF header 会出现在每个 ELF 文件的开头，它的定义如下

```c
// https://elixir.bootlin.com/linux/v6.7-rc5/source/include/uapi/linux/elf.h#L241
#define EI_NIDENT	16

typedef struct elf64_hdr {
  unsigned char	e_ident[EI_NIDENT];	/* ELF "magic number" */
  Elf64_Half e_type;
  Elf64_Half e_machine;
  Elf64_Word e_version;
  Elf64_Addr e_entry;		/* Entry point virtual address */
  Elf64_Off e_phoff;		/* Program header table file offset */
  Elf64_Off e_shoff;		/* Section header table file offset */
  Elf64_Word e_flags;
  Elf64_Half e_ehsize;
  Elf64_Half e_phentsize;
  Elf64_Half e_phnum;
  Elf64_Half e_shentsize;
  Elf64_Half e_shnum;
  Elf64_Half e_shstrndx;
} Elf64_Ehdr;
```

其中的数据类型在之后也会遇到

```c
/* 64-bit ELF base types. */
typedef __u64	Elf64_Addr;
typedef __u16	Elf64_Half;
typedef __s16	Elf64_SHalf;
typedef __u64	Elf64_Off;
typedef __s32	Elf64_Sword;
typedef __u32	Elf64_Word;
typedef __u64	Elf64_Xword;
typedef __s64	Elf64_Sxword;
```

- `e_ident` ，即 ELF identification，描述了“这是一个 ELF 文件”

  ```shell
  ➜  xiao hexdump -C ./this_is_an_elf_file | head -1
  00000000  7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00  |.ELF............|
  ```

  这 16 个 bytes 表示了不同的意思，接下来通过写一个简单的 ELF parser 来描述这一段内容吧！

  ```c
  #include <stdio.h>
  #include <stdint.h>
  #include <stdlib.h>
  #include <assert.h>
  #include <fcntl.h>
  #include <unistd.h>
  #include <elf.h>
  
  #define	EI_MAG0		0		/* e_ident[] indexes */
  #define	EI_MAG1		1
  #define	EI_MAG2		2
  #define	EI_MAG3		3
  #define	EI_CLASS	4
  #define	EI_DATA		5
  #define	EI_VERSION	6
  #define	EI_OSABI	7
  #define	EI_PAD		8

  #define	ELFMAG0		0x7f		/* EI_MAG */
  #define	ELFMAG1		'E'
  #define	ELFMAG2		'L'
  #define	ELFMAG3		'F'
  #define	ELFMAG		"\177ELF"
  #define	SELFMAG		4

  int main() {
    int fd = open("./this_is_an_elf_file", 0, 0);
    uint8_t ident[0x10] = { 0 };
    read(fd, &ident, 0x10);

    // the first 4 bytes
    uint8_t *magic = (uint8_t *)ident;

    // identify the ELF file
    assert(
      magic[0] == ELFMAG0 &&
      magic[1] == ELFMAG1 &&
      magic[2] == ELFMAG2 &&
      magic[3] == ELFMAG3
    );

    // ELF class
    if (ident[EI_CLASS] == ELFCLASS64) {
      printf("[*] 64 bit files\n");
    } else if (ident[EI_CLASS] == ELFCLASS32) {
      printf("[*] 32 bit files\n");
    }

    // ELF encoding
    if (ident[EI_DATA] == ELFDATA2LSB) {
      printf("[*] little endian ELF\n");
    } else if (ident[EI_DATA] == ELFDATA2MSB) {
      printf("[*] big endian ELF\n");
    }

    // ELF OS ABI
    if (ident[EI_OSABI] == ELFOSABI_SYSV) {
      printf("[*] System V ABI\n");
    } else if (ident[EI_OSABI] == ELFOSABI_HPUX) {
      printf("[*] HP-UX operating system ABI\n");
    } else if (ident[EI_OSABI] == ELFOSABI_STANDALONE) {
      printf("[*] Standalone (embedded) application\n");
    }

    printf("[*] API version: %d\n", ident[EI_VERSION]);

    return 0;
  }
  ```

- `e_type` 描述 ELF 的类型，包括：

  - `ET_NONE` 没有类型也是类型
  - `ET_REL` Relocatable file
  - `ET_EXEC` Executable file
  - `ET_DYN` Shared object file
  - `ET_CORE` Core file, Coredump 也是 ELF 类型

- `e_machine` 描述目标平台

- `e_version` 描述版本

- `e_entry` 储存 ELF 文件的入口虚拟地址

- `e_phoff` 储存 ELF Program header 的 offset，也就是说，Program header 储存在距离文件开头 `e_phoff`的位置

- `e_shoff` 储存 ELF Section header 的 offset

- `e_flags` 处理器特定的 flags

- `e_ehsize` ELF 文件头的大小

- `e_phentsize` ELF Program header entry 的大小

- `e_phnum` ELF Program header 的数量

- `e_shentsize` 类似 `e_phentsize`但是是 Section

- `e_shnum` 同上类推

- `e_shstrndx` Section 中字符串表的 index

### Section Header

Section 保存了 ELF 文件中的各种信息，Section header 的定义如下

```c
typedef struct elf64_shdr {
  Elf64_Word sh_name;		/* Section name, index in string tbl */
  Elf64_Word sh_type;		/* Type of section */
  Elf64_Xword sh_flags;		/* Miscellaneous section attributes */
  Elf64_Addr sh_addr;		/* Section virtual addr at execution */
  Elf64_Off sh_offset;		/* Section file offset */
  Elf64_Xword sh_size;		/* Size of section in bytes */
  Elf64_Word sh_link;		/* Index of another section */
  Elf64_Word sh_info;		/* Additional section information */
  Elf64_Xword sh_addralign;	/* Section alignment */
  Elf64_Xword sh_entsize;	/* Entry size if section holds table */
} Elf64_Shdr;

```

- `sh_flags` 描述了 Section 的一些属性，包括 `SHF_WRITE`，`SHF_ALLOC`，`SHF_EXECINSTR` 等等

- `sh_type`描述了 Section 的类型，包括了储存 dynamic linking table 的 `SHT_DYNAMIC` ，存放 linker symbol table 的 `SHT_SYMTAB`，由程序定义的 `SHT_PROGBITS`等等

  使用 `readelf -S`可以观察程序的 section headers

  ```log
  root@da070736a297:/# readelf -S /bin/sh
  There are 28 section headers, starting at offset 0x1d358:

  Section Headers:
    [Nr] Name              Type             Address           Offset
         Size              EntSize          Flags  Link  Info  Align
    [ 0]                   NULL             0000000000000000  00000000
         0000000000000000  0000000000000000           0     0     0
    [ 1] .interp           PROGBITS         0000000000000238  00000238
         000000000000001c  0000000000000000   A       0     0     1
    [ 2] .note.ABI-tag     NOTE             0000000000000254  00000254
         0000000000000020  0000000000000000   A       0     0     4
  ```

### Program Header

在 ELF 程序中，多个 sections 可以存放在一个 segments 中用以加载。Program header table 就储存了用来描述 segment 的 Program header，其定义如下

```c
typedef struct elf64_phdr {
  Elf64_Word p_type;
  Elf64_Word p_flags;
  Elf64_Off p_offset;		/* Segment file offset */
  Elf64_Addr p_vaddr;		/* Segment virtual address */
  Elf64_Addr p_paddr;		/* Segment physical address */
  Elf64_Xword p_filesz;		/* Segment size in file */
  Elf64_Xword p_memsz;		/* Segment size in memory */
  Elf64_Xword p_align;		/* Segment alignment, file & memory */
} Elf64_Phdr;

```

- `p_type`表示 segment 的类型，包括有 `PT_LOAD`，`PT_DYNAMIC`，`PT_INTERP`等等。

- `p_flags` 包括有 `PF_X`，`PF_W`，`PF_R`等等，通过不同的 bit 表达不同的信息，可以相互组合。这决定了 segment 映射时的权限。

  ```log
  root@da070736a297:/# readelf -l /bin/sh

  Elf file type is DYN (Shared object file)
  Entry point 0x4a20
  There are 9 program headers, starting at offset 64

  Program Headers:
    Type           Offset             VirtAddr           PhysAddr
                   FileSiz            MemSiz              Flags  Align
    PHDR           0x0000000000000040 0x0000000000000040 0x0000000000000040
                   0x00000000000001f8 0x00000000000001f8  R      0x8
    INTERP         0x0000000000000238 0x0000000000000238 0x0000000000000238
                   0x000000000000001c 0x000000000000001c  R      0x1
        [Requesting program interpreter: /lib64/ld-linux-x86-64.so.2]
    LOAD           0x0000000000000000 0x0000000000000000 0x0000000000000000
                   0x000000000001b268 0x000000000001b268  R E    0x200000
    LOAD           0x000000000001bf50 0x000000000021bf50 0x000000000021bf50
                   0x00000000000012d0 0x0000000000003f00  RW     0x200000
    DYNAMIC        0x000000000001cb28 0x000000000021cb28 0x000000000021cb28
                   0x00000000000001f0 0x00000000000001f0  RW     0x8
    NOTE           0x0000000000000254 0x0000000000000254 0x0000000000000254
                   0x0000000000000044 0x0000000000000044  R      0x4
    GNU_EH_FRAME   0x00000000000179e4 0x00000000000179e4 0x00000000000179e4
                   0x00000000000007dc 0x00000000000007dc  R      0x4
    GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                   0x0000000000000000 0x0000000000000000  RW     0x10
    GNU_RELRO      0x000000000001bf50 0x000000000021bf50 0x000000000021bf50
                   0x00000000000010b0 0x00000000000010b0  R      0x1

   Section to Segment mapping:
    Segment Sections...
     00
     01     .interp
     02     .interp .note.ABI-tag .note.gnu.build-id .gnu.hash .dynsym .dynstr .gnu.version .gnu.version_r .rela.dyn .rela.plt .init .plt .plt.got .text .fini .rodata .eh_frame_hdr .eh_frame
     03     .init_array .fini_array .data.rel.ro .dynamic .got .data .bss
     04     .dynamic
     05     .note.ABI-tag .note.gnu.build-id
     06     .eh_frame_hdr
     07
     08     .init_array .fini_array .data.rel.ro .dynamic .got
  ```

## 控制 ELF 的结构

以下的程序会把 `a` 和 `function()` 放入对应的 section 中：

```c
int __attribute__((section(".myvariable"))) a = 0;
int __attribute__((section(".myfunction"))) main() { return 0; }
```

你可以尝试使用 `readelf`观察编译后的程序。

同时，也可以使用 Linker Script 来控制 ELF 结构：

```ld
SECTIONS
{
    . = 0xc0ffee00;
    .text : { *(.text) }
    . = ALIGN(0x1000);
    .data : { *(.data) }
    .bss : { *(.bss) }
}
```

使用以下命令编译

```bash
gcc main.c -c -o main.o && ld main.o -T ./script.ld -o main
```

观察结果，你也可以使用 `readelf`，这里使用 `gdb`插件 `gef`的`vmmap`命令来观察，也可以直接观察 `/proc/pid/`

```log
gef➤  vmmap
[ Legend:  Code | Heap | Stack ]
Start              End                Offset             Perm Path
0x00000000c0ffe000 0x00000000c0fff000 0x0000000000000000 r-x /home/xiao/Desktop/ctf/main
0x00000000c0fff000 0x00000000c1000000 0x0000000000001000 rw- /home/xiao/Desktop/ctf/main
0x00000000c1000000 0x00000000c1001000 0x0000000000000000 rw- [heap]
0x00007ffff7ff9000 0x00007ffff7ffd000 0x0000000000000000 r-- [vvar]
0x00007ffff7ffd000 0x00007ffff7fff000 0x0000000000000000 r-x [vdso]
0x00007ffffffdd000 0x00007ffffffff000 0x0000000000000000 rw- [stack]
```

```log
$ readelf -Sl ./main
There are 9 section headers, starting at offset 0x1100:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [ 0]                   NULL             0000000000000000  00000000
       0000000000000000  0000000000000000           0     0     0
  [ 1] .text             PROGBITS         00000000c0ffee00  00000e00
       000000000000000b  0000000000000000  AX       0     0     1
  [ 2] .eh_frame         PROGBITS         00000000c0ffee10  00000e10
       0000000000000038  0000000000000000   A       0     0     8
  [ 3] .data             PROGBITS         00000000c0fff000  00001000
       000000000000000e  0000000000000000  WA       0     0     8
  [ 4] .bss              NOBITS           00000000c0fff010  0000100e
       0000000000000004  0000000000000000  WA       0     0     4
  [ 5] .comment          PROGBITS         0000000000000000  0000100e
       000000000000001e  0000000000000001  MS       0     0     1
  [ 6] .symtab           SYMTAB           0000000000000000  00001030
       0000000000000078  0000000000000018           7     2     8
  [ 7] .strtab           STRTAB           0000000000000000  000010a8
       0000000000000012  0000000000000000           0     0     1
  [ 8] .shstrtab         STRTAB           0000000000000000  000010ba
       000000000000003f  0000000000000000           0     0     1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  D (mbind), l (large), p (processor specific)

Elf file type is EXEC (Executable file)
Entry point 0xc0ffee00
There are 4 program headers, starting at offset 64

Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  LOAD           0x0000000000000000 0x00000000c0ffe000 0x00000000c0ffd000
                 0x0000000000000120 0x0000000000000120  R      0x1000
  LOAD           0x0000000000000e00 0x00000000c0ffee00 0x00000000c0ffee00
                 0x0000000000000048 0x0000000000000048  R E    0x1000
  LOAD           0x0000000000001000 0x00000000c0fff000 0x00000000c0fff000
                 0x000000000000000e 0x0000000000000014  RW     0x1000
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10

 Section to Segment mapping:
  Segment Sections...
   00
   01     .text .eh_frame
   02     .data .bss
   03
```
