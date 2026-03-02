#!/usr/bin/env python3

import os
import shutil
import subprocess
import argparse
import glob


parser = argparse.ArgumentParser(description="Build script for YSOS")
parser.add_argument("-d", "--debug", action="store_true", help="Enable debug for qemu")
parser.add_argument(
    "-i", "--intdbg", action="store_true", help="Enable interrupt output for qemu"
)
parser.add_argument(
    "-m", "--memory", default="96M", help="Set memory size for qemu, default is 96M"
)
parser.add_argument(
    "-o",
    "--output",
    default="-nographic",
    help="Set output for qemu, default is -nographic",
)
parser.add_argument(
    "-p",
    "--profile",
    type=str,
    choices=["release", "debug"],
    default="release",
    help="Set build profile for kernel",
)
parser.add_argument(
    "-v", "--verbose", action="store_true", help="Enable verbose output"
)
parser.add_argument("--dry-run", action="store_true", help="Enable dry run")
parser.add_argument(
    "--bios", type=str, default=os.path.join("assets", "OVMF.fd"), help="Set BIOS path"
)
parser.add_argument("--boot", type=str, default="esp", help="Set boot path")
parser.add_argument(
    "--debug-listen",
    type=str,
    default="0.0.0.0:1234",
    help="Set listen address for gdbserver",
)

parser.add_argument(
    "task",
    type=str,
    choices=["build", "clean", "launch", "run", "clippy"],
    default="build",
    help="Task to execute",
)

parser.add_argument(
    "--vvfat_disabled",
    action="store_true",
    help="QEMU doesn't support vvfat",
)

args = parser.parse_args()


def info(step: str, content: str):
    print(f"\033[1;32m[+] {step}:\033[0m \033[1m{content}\033[0m")


def error(step: str, content: str):
    print(f"\033[1;31m[E] {step}:\033[0m \033[1m{content}\033[0m")


def debug(step: str, content: str):
    if args.verbose or args.dry_run:
        print(f"\033[1;34m[?] {step}:\033[0m \033[1m{content}\033[0m")


def get_exe(name: str):
    res = shutil.which(name)
    if res is None:
        raise Exception(f"{name} not found in PATH")
    return res


def get_apps():
    app_path = os.path.join(os.getcwd(), "crates", "app")

    if not os.path.exists(app_path):
        return []

    apps = [
        name
        for name in os.listdir(app_path)
        if os.path.isdir(os.path.join(app_path, name))
        and name not in ["config", ".cargo"]
    ]

    return apps


def execute_command(cmd: list, workdir: str | None = None, shell: bool = False) -> int:
    debug("Executing", " ".join(cmd) + (f" in {workdir}" if workdir else ""))

    if args.dry_run:
        return 0

    prog = subprocess.Popen(cmd, cwd=workdir, shell=shell)
    prog.wait()

    if prog.returncode != 0:
        raise Exception(f"{cmd} failed with code {prog.returncode}")

    return prog.returncode


def qemu(
    output: str = "-nographic",
    memory: str = "96M",
    debug: bool = False,
    intdbg: bool = False,
    vvfat_disabled: bool = False,
):
    qemu_exe = shutil.which("qemu-system-x86_64")

    # add optional path C:\Program Files\qemu for Windows
    if qemu_exe is None and os.name == "nt":
        qemu_exe = shutil.which("qemu-system-x86_64", path="C:\\Program Files\\qemu")

    if qemu_exe is None:
        raise Exception("qemu-system-x86_64 not found in PATH")

    if vvfat_disabled:
        esp = "esp.img"

        execute_command(
            [get_exe("dd"), "if=/dev/zero", f"of={esp}", "bs=1M", "count=64"]
        )

        execute_command(
            [get_exe("mformat"), "-i", esp, "-t", "64", "-h", "32", "-s", "64", "::"]
        )

        execute_command(
            [
                get_exe("mcopy"),
                "-i",
                esp,
                "-s",
                *glob.glob(os.path.join("esp", "*")),
                "::/",
            ]
        )
    else:
        esp = "fat:esp"

    qemu_args = [
        qemu_exe,
        "-bios",
        args.bios,
        "-net",
        "none",
        *output.split(),
        "-m",
        memory,
        "-drive",
        f"format=raw,file={esp}",
        "-snapshot",
    ]

    if debug:
        qemu_args += ["-gdb", f"tcp:{args.debug_listen}", "-S"]
    elif intdbg:
        qemu_args += ["-no-reboot", "-d", "int,cpu_reset"]

    execute_command(qemu_args)


def copy_to_esp(src: str, dst: str):
    dst = os.path.join(os.getcwd(), args.boot, dst)

    if args.dry_run:
        debug("Would copy", f"{src} -> {dst}")
        return

    # mkdir for dst if not exists
    dst_base = os.path.dirname(dst)
    if not os.path.exists(dst_base):
        os.makedirs(dst_base)

    # copy files
    if os.path.isfile(src):
        debug("Copying", f"{src} -> {dst}")
        shutil.copy(src, dst)
    else:
        raise Exception(f"{src} is not a file")


def build():
    cargo_exe = get_exe("cargo")

    # build uefi boot loader
    bootloader = os.path.join(os.getcwd(), "crates", "boot")
    info("Building", "bootloader...")
    execute_command([cargo_exe, "build", "--release"], bootloader)
    compile_output = os.path.join(
        os.getcwd(), "target", "x86_64-unknown-uefi", "release", "ysos_boot.efi"
    )
    copy_to_esp(compile_output, os.path.join("EFI", "BOOT", "BOOTX64.EFI"))

    # copy kernel config
    config_path = os.path.join(os.getcwd(), "crates", "kernel", "config", "boot.conf")
    if os.path.exists(config_path):
        copy_to_esp(config_path, os.path.join("EFI", "BOOT", "boot.conf"))

    # build kernel
    kernel = os.path.join(os.getcwd(), "crates", "kernel")
    info("Building", "kernel...")
    profile = (
        "--release" if args.profile == "release" else "--profile=release-with-debug"
    )
    execute_command([cargo_exe, "build", profile, "-Zjson-target-spec"], kernel)
    profile_dir = "release" if args.profile == "release" else "release-with-debug"
    compile_output = os.path.join(
        os.getcwd(), "target", "x86_64-unknown-none", profile_dir, "ysos_kernel"
    )
    copy_to_esp(compile_output, "KERNEL.ELF")

    # build apps
    apps = get_apps()
    for app in apps:
        app_path = os.path.join(os.getcwd(), "crates", "app", app)
        app_name = None

        # read Cargo.toml to get the package name
        with open(os.path.join(app_path, "Cargo.toml"), "r") as f:
            for line in f.readlines():
                if "name" in line:
                    app_name = line.split('"')[1]
                    break

        if not app_name:
            raise Exception(f"Failed to get app name for {app}")

        info("Building", f"app {app}...")
        execute_command([cargo_exe, "build", profile, "-Zjson-target-spec"], app_path)
        compile_output = os.path.join(
            os.getcwd(), "target", "x86_64-unknown-ysos", profile_dir, app_name
        )
        copy_to_esp(compile_output, os.path.join("APP", app))


def clippy():
    cargo_exe = get_exe("cargo")

    info("Running", "cargo fmt on root...")
    execute_command([cargo_exe, "+nightly", "fmt", "--all"])

    kernel = os.path.join(os.getcwd(), "crates", "kernel")
    info("Running", "clippy on kernel...")
    execute_command([cargo_exe, "clippy", "-Zjson-target-spec"], kernel)

    apps = get_apps()
    for app in apps:
        app_path = os.path.join(os.getcwd(), "crates", "app", app)
        info("Running", f"clippy on app {app}...")
        execute_command([cargo_exe, "clippy", "-Zjson-target-spec"], app_path)


def clean():
    if os.path.exists(args.boot):
        shutil.rmtree(args.boot)

    execute_command([get_exe("cargo"), "clean"])


def main():
    if args.task == "build":
        build()
    elif args.task == "clean":
        clean()
    elif args.task == "launch":
        qemu(args.output, args.memory, args.debug, args.intdbg, args.vvfat_disabled)
    elif args.task == "run":
        build()
        qemu(args.output, args.memory, args.debug, args.intdbg, args.vvfat_disabled)
    elif args.task == "clippy":
        clippy()


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        error("Error", str(e))
        exit(1)
