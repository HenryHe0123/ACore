TARGET := riscv64gc-unknown-none-elf
OS_ELF := target/$(TARGET)/release/os
OS_BIN := $(OS_ELF).bin

run:
	cargo build --release
	rust-objcopy --strip-all $(OS_ELF) -O binary $(OS_BIN)
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=$(OS_BIN),addr=0x80200000

clean:
	cargo clean

.PHONY: run clean