OVMF := assets/OVMF.fd
ESP := esp
BUILD_ARGS :=
QEMU_ARGS := -m 64M
QEMU_OUTPUT := -nographic
MODE ?= release
CUR_PATH := $(shell pwd)
DBG_INFO := false

ifeq (${MODE}, release)
	BUILD_ARGS := --release
endif

.PHONY: build run debug clean launch intdbg \
	target/x86_64-unknown-uefi/$(MODE)/ysos_boot.efi

run: build launch

launch:
	@qemu-system-x86_64 \
		-bios ${OVMF} \
		-net none \
		$(QEMU_ARGS) \
		$(QEMU_OUTPUT) \
		-drive format=raw,file=fat:${ESP} \
		-snapshot

intdbg:
	@qemu-system-x86_64 \
		-bios ${OVMF} \
		-net none \
		$(QEMU_ARGS) \
		$(QEMU_OUTPUT) \
		-drive format=raw,file=fat:${ESP} \
		-snapshot \
		-no-reboot -d int,cpu_reset

debug:
	@qemu-system-x86_64 \
		-bios ${OVMF} \
		-net none \
		$(QEMU_ARGS) \
		$(QEMU_OUTPUT) \
		-drive format=raw,file=fat:${ESP} \
		-snapshot \
		-s -S

clean:
	@cargo clean

build: $(ESP)

$(ESP): $(ESP)/EFI/BOOT/BOOTX64.EFI

$(ESP)/EFI/BOOT/BOOTX64.EFI: target/x86_64-unknown-uefi/$(MODE)/ysos_boot.efi
	@mkdir -p $(@D)
	cp $< $@

target/x86_64-unknown-uefi/$(MODE)/ysos_boot.efi: pkg/boot
	cd pkg/boot && cargo build $(BUILD_ARGS)
