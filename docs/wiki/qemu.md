# QEMU 使用参考

QEMU 是一个开源的虚拟机软件，它可以模拟多种硬件平台，如 x86、ARM、MIPS 等，可以运行在多种操作系统上，如 Linux、Windows、macOS 等。

可以使用类似于如下的命令行运行 QEMU：

```sh
$ qemu-system-x86_64 -bios ./ovmf.fd -net none \
    -m 96M -drive format=raw,file=fat:rw:./esp -nographic
```

其中 `-bios` 指定了 UEFI 的固件，`-net none` 指定了网络设备，`-m` 指定了内存大小，`-drive` 指定了硬盘，`-nographic` 指定了不使用图形界面，转而将串口 IO 重定向到标准输入输出。

`-drive` 所指定的 `format=raw,file=fat:rw:./esp` 让虚拟机把宿主机上的一个普通**目录** `./esp` 伪装成一个 FAT 格式的虚拟硬盘挂载上去。这一技术被称为 vvfat (Virtual FAT)。使用 Kylin OS 这样的系统作为开发环境时，可能会遇到 QEMU 报错 `Unknown protocol 'fat'`，这是因为 Kylin OS 这样重视安全性的环境的 QEMU 不支持 vvfat 这一存在安全隐患的功能，不能挂载目录而只能挂载常规镜像文件。本实验提供的 Python 脚本 `ysos.py` 支持自动化从 `esp` 目录构建镜像文件 `esp.img` 用于运行和调试内核，需要在执行脚本的指令中添加参数 `--vvfat_disabled`，例如：

```sh
$ python ysos.py launch --vvfat_disabled
```

上述运行脚本的指令所做的事情等价于：

```sh
$ dd if=/dev/zero of=esp.img bs=1M count=64
$ mformat -i esp.img -t 64 -h 32 -s 64 ::
$ mcopy -i esp.img -s ./esp/* ::/
$ qemu-system-x86_64 -bios ./ovmf.fd -net none \
    -m 96M -drive format=raw,file=esp.img -nographic
```

其中 `mformat` 和 `mcopy` 可以通过包管理器安装 `mtools` 获取。

为了退出 QEMU，可以使用 <kbd>Ctrl</kbd> + <kbd>A</kbd> 后输入 <kbd>X</kbd> 退出。

在调试时，可以使用 `-s` 参数来启动 GDB 调试服务，是 `-gdb tcp:1234` 的简写，并使用 `-S` 参数来暂停 CPU 的执行，等待 GDB 连接。

当遇到 Triple Fault 时，可以使用 `-no-reboot` 参数来阻止 QEMU 重启。并使用 `-d int,cpu_reset` 参数来打印中断和 CPU 重置的调试信息，这部分对于中断调试很有帮助。

可以参考 [官方文档](https://www.qemu.org/docs/master/system/index.html) 获取更多的 QEMU 使用信息。
