# QEMU 使用参考

QEMU 是一个开源的虚拟机软件，它可以模拟多种硬件平台，如 x86、ARM、MIPS 等，可以运行在多种操作系统上，如 Linux、Windows、macOS 等。

可以使用类似于如下的命令行运行 QEMU：

```sh
$ qemu-system-x86_64 -bios ./ovmf.fd -net none \
    -m 96M -drive format=raw,file=fat:rw:./esp -nographic
```

其中 `-bios` 指定了 UEFI 的固件，`-net none` 指定了网络设备，`-m` 指定了内存大小，`-drive` 指定了硬盘，`-nographic` 指定了不使用图形界面，转而将串口 IO 重定向到标准输入输出。

为了退出 QEMU，可以使用 `Ctrl + A` 再输入 `X`。

在调试时，可以使用 `-s` 参数来启动 GDB 调试服务，是 `-gdb tcp:1234` 的简写，并使用 `-S` 参数来暂停 CPU 的执行，等待 GDB 连接。

当遇到 Triple Fault 时，可以使用 `-no-reboot` 参数来阻止 QEMU 重启。并使用 `-d int,cpu_reset` 参数来打印中断和 CPU 重置的调试信息，这部分对于中断调试很有帮助。

可以参考 [官方文档](https://www.qemu.org/docs/master/system/index.html) 获取更多的 QEMU 使用信息。
