#!/usr/bin/env python3

import os
import shutil
import subprocess
import argparse


parser = argparse.ArgumentParser(description='Build script for YSOS')
parser.add_argument('-d', '--debug', action='store_true',
                    help='Enable debug for qemu')
parser.add_argument('-i', '--intdbg', action='store_true',
                    help='Enable interrupt output for qemu')
parser.add_argument('-m', '--memory', default='96M',
                    help='Set memory size for qemu, default is 96M')
parser.add_argument('-o', '--output', default='-nographic',
                    help='Set output for qemu, default is -nographic')
parser.add_argument('-v', '--verbose', action='store_true',
                    help='Enable verbose output')
parser.add_argument('--dry-run', action='store_true', help='Enable dry run')
parser.add_argument('--bios', type=str,
                    default=os.path.join('assets', 'OVMF.fd'), help='Set BIOS path')
parser.add_argument('--boot', type=str, default='esp', help='Set boot path')

parser.add_argument('task', type=str, choices=[
                    'build', 'clean', 'launch', 'run'
                    ], default='build', help='Task to execute')

args = parser.parse_args()


def info(step: str, content: str):
    print(f'\033[1;32m[+] {step}:\033[0m {content}')


def error(step: str, content: str):
    print(f'\033[1;31m[E] {step}:\033[0m {content}')


def debug(step: str, content: str):
    if args.verbose or args.dry_run:
        print(f'\033[1;34m[?] {step}:\033[0m {content}')


def execute_command(cmd: list, workdir: str = None, shell: bool = False) -> int:
    debug('Executing', " ".join(cmd) + (f' in {workdir}' if workdir else ''))

    if args.dry_run:
        return 0

    prog = subprocess.Popen(cmd, cwd=workdir, shell=shell)
    prog.wait()

    if prog.returncode != 0:
        raise Exception(f"{cmd} failed with code {prog.returncode}")

    return prog.returncode


def qemu(output: str = '-nographic', memory: str = '96M', debug: bool = False, intdbg: bool = False):
    qemu_exe = shutil.which('qemu-system-x86_64')

    # add optional path C:\Program Files\qemu for Windows
    if qemu_exe is None and os.name == 'nt':
        qemu_exe = shutil.which('qemu-system-x86_64',
                                path='C:\\Program Files\\qemu')

    if qemu_exe is None:
        raise Exception('qemu-system-x86_64 not found in PATH')

    qemu_args = [qemu_exe, '-bios', args.bios, '-net', 'none', *output.split(),
                 '-m', memory, '-drive', 'format=raw,file=fat:esp', '-snapshot']

    if debug:
        qemu_args += ['-s', '-S']
    elif intdbg:
        qemu_args += ['-no-reboot', '-d', 'int,cpu_reset']

    execute_command(qemu_args)


def copy_to_esp(src: str, dst: str):
    dst = os.path.join(os.getcwd(), args.boot, dst)

    if args.dry_run:
        debug('Would copy', f'{src} -> {dst}')
        return

    # mkdir for dst if not exists
    dst_base = os.path.dirname(dst)
    if not os.path.exists(dst_base):
        os.makedirs(dst_base)

    # copy files
    if os.path.isfile(src):
        debug('Copying', f'{src} -> {dst}')
        shutil.copy(src, dst)
    else:
        raise Exception(f'{src} is not a file')


def build():
    cargo_exe = shutil.which('cargo')

    if cargo_exe is None:
        raise Exception('cargo not found in PATH')

    # build uefi boot loader
    bootloader = os.path.join(os.getcwd(), 'pkg', 'boot')
    info('Building', 'bootloader...')
    execute_command([cargo_exe, 'build', '--release'], bootloader)
    compile_output = os.path.join(os.getcwd(), 'target',
                                  'x86_64-unknown-uefi', 'release', 'ysos_boot.efi')
    copy_to_esp(compile_output, os.path.join('EFI', 'BOOT', 'BOOTX64.EFI'))


def clean():
    if os.path.exists(args.boot):
        shutil.rmtree(args.boot)

    cargo_exe = shutil.which('cargo')

    if cargo_exe is None:
        raise Exception('cargo not found in PATH')

    execute_command([cargo_exe, 'clean'])


def main():
    if args.task == 'build':
        build()
    elif args.task == 'clean':
        clean()
    elif args.task == 'launch':
        qemu(args.output, args.memory, args.debug, args.intdbg)
    elif args.task == 'run':
        build()
        qemu(args.output, args.memory, args.debug, args.intdbg)


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        error('Error', e)
        exit(1)
