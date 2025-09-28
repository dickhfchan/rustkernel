# RustKernel ARM64 Microkernel Build System

KERNEL_BIN = target/aarch64-unknown-none/debug/rustkernel
QEMU_ARGS = -machine virt -cpu cortex-a72 -smp 2 -m 1G -nographic

.PHONY: build clean run debug

build:
	cargo build -p rustkernel

release:
	cargo build -p rustkernel --release

run: build
	qemu-system-aarch64 $(QEMU_ARGS) -kernel $(KERNEL_BIN)

debug: build
	qemu-system-aarch64 $(QEMU_ARGS) -kernel $(KERNEL_BIN) -s -S

clean:
	cargo clean

# Install required tools
install-deps:
	rustup target add aarch64-unknown-none
	rustup component add rust-src
	brew install qemu # For macOS

# Check if we have all required tools
check-deps:
	@which qemu-system-aarch64 || (echo "qemu-system-aarch64 not found. Run 'make install-deps'" && exit 1)
	@rustup target list --installed | grep aarch64-unknown-none || (echo "ARM64 target not installed. Run 'make install-deps'" && exit 1)