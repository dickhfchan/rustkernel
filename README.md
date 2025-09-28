# RustKernel - ARM64 Microkernel

A minimal microkernel operating system written in Rust for ARM64 architecture.

## Architecture

- **Target**: ARM64 (AArch64)
- **Boot**: UEFI
- **IPC**: Port-based asynchronous messaging
- **Design**: Microkernel with userspace services

## Project Structure

```
rustkernel/
├── kernel/           # Minimal kernel (IPC, memory, scheduling)
├── userland/
│   ├── runtime/      # Userspace runtime library
│   └── services/     # Core OS services
│       ├── memory-manager/
│       └── process-manager/
├── bootloader/       # UEFI bootloader
└── Makefile         # Build system
```

## Core Principles

1. **Minimal Kernel**: Only essential functions in kernel space
2. **Service-Oriented**: OS services run as userspace processes
3. **Message Passing**: Asynchronous IPC via ports
4. **Isolation**: Services can crash/restart independently
5. **Memory Safety**: Rust throughout the system

## Building

### Prerequisites

```bash
make install-deps  # Install Rust target and QEMU
```

### Build and Run

```bash
make build        # Build kernel
make run          # Run in QEMU
make debug        # Run with GDB server
```

## Current Status

- [x] Project structure
- [x] Basic kernel framework
- [x] IPC message system design
- [ ] Memory management
- [ ] Interrupt handling
- [ ] Process management  
- [ ] Userspace services
- [ ] UEFI bootloader

## Next Steps

1. Implement ARM64 memory management
2. Set up interrupt handling
3. Build process scheduler
4. Create userspace runtime
5. Implement core services# rustkernel
