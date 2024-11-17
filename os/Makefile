# Makefile for building and running a RISC-V OS with QEMU

# 编译目标架构
TARGET = riscv64gc-unknown-none-elf
# 构建模式
MODE = release
# 输出目录
OUT_DIR = ./target/$(TARGET)/$(MODE)
# 内核文件
KERNEL_ELF = $(OUT_DIR)/os
# 二进制文件
KERNEL_BIN = $(OUT_DIR)/os.bin
# QEMU BIOS 文件
BOOTLOADER = ../bootloader/rustsbi-qemu.bin

# QEMU 配置
QEMU = qemu-system-riscv64
QEMU_MACHINE = virt
KERNEL_ENTRY_PA = 0x80200000
QEMU_ARGS = -machine $(QEMU_MACHINE) -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

# 默认目标
all: build run

# 构建内核并生成二进制文件
build: env
	cargo build --target $(TARGET) --release
	rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview
	rustup target add riscv64gc-unknown-none-elf

# 运行 QEMU
run: build
	$(QEMU) $(QEMU_ARGS)

# 清理构建文件
clean:
	cargo clean

# PHONY targets to avoid conflict with files of the same name
.PHONY: all build run clean
