TARGET := riscv64gc-unknown-none-elf
OS_ELF := target/$(TARGET)/release/os
OS_BIN := $(OS_ELF).bin

BOOTLOADER := ../bootloader/rustsbi-qemu.bin
ENTRY_ADDR := 0x80200000
# BOOTLOADER := none
# ENTRY_ADDR := 0x80000000

clean:
	@cargo clean

build: clean
	@cargo build --release
	@rust-objcopy --strip-all $(OS_ELF) -O binary $(OS_BIN)

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(OS_BIN),addr=$(ENTRY_ADDR)

debug: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(OS_BIN),addr=$(ENTRY_ADDR) \
		-s -S

.PHONY: build run debug clean