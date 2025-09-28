# RustKernel - ARM64 Microkernel

A minimal microkernel operating system written in Rust for ARM64 architecture, featuring memory safety, hardware isolation, and service-oriented design.

## Architecture Overview

- **Target Architecture**: ARM64 (AArch64) with QEMU virt machine support
- **Boot System**: UEFI-compatible with device tree parsing
- **IPC Design**: Port-based asynchronous message passing
- **Memory Management**: 4-level page tables with physical frame allocation
- **Kernel Philosophy**: Microkernel with userspace services

## Project Structure

```
rustkernel/
├── kernel/                    # Minimal kernel core
│   ├── src/
│   │   ├── main.rs           # Kernel entry point
│   │   ├── boot.s            # ARM64 assembly boot code
│   │   ├── uart.rs           # PL011 UART driver
│   │   ├── devicetree.rs     # FDT parsing
│   │   ├── allocator.rs      # Kernel heap allocator
│   │   ├── memory/           # Memory management subsystem
│   │   │   ├── frame_allocator.rs # Physical memory
│   │   │   ├── paging.rs     # Virtual memory
│   │   │   ├── mmu.rs        # ARM64 MMU control
│   │   │   └── test.rs       # Memory testing
│   │   ├── interrupts.rs     # Exception handling
│   │   ├── process.rs        # Process management
│   │   └── ipc.rs            # Inter-process communication
│   ├── linker.ld             # Custom linker script
│   └── Cargo.toml           # Kernel dependencies
├── userland/                 # Userspace components
│   ├── runtime/              # Userspace runtime library
│   └── services/             # System services
│       ├── memory-manager/   # Memory service
│       └── process-manager/  # Process service
├── bootloader/               # UEFI bootloader (future)
├── .cargo/config.toml        # Rust build configuration
├── Makefile                  # Build automation
└── Cargo.toml               # Workspace configuration
```

## Core Design Principles

1. **Minimal Kernel**: Only essential functions (IPC, memory, scheduling) in kernel space
2. **Service-Oriented**: OS services (filesystem, network, drivers) run as userspace processes
3. **Asynchronous Message Passing**: Port-based IPC with message queues
4. **Memory Safety**: Rust's ownership system prevents memory corruption
5. **Hardware Isolation**: ARM64 memory protection and privilege levels
6. **Fault Tolerance**: Services can crash and restart independently

## Implemented Features

### ✅ Boot System
- ARM64 assembly boot sequence with CPU detection
- Device tree (FDT) parsing for hardware discovery
- UART console driver (PL011) for early debugging
- Stack setup and BSS initialization
- Clean transition from assembly to Rust

### ✅ Memory Management
- **Physical Frame Allocator**: Bitmap-based with O(1) free tracking
- **Virtual Memory**: Complete ARM64 4-level page table implementation
- **Heap Allocator**: Dynamic allocation using `linked_list_allocator`
- **Memory Discovery**: Automatic detection via device tree parsing
- **Testing Suite**: Comprehensive allocation/deallocation validation
- **Statistics**: Real-time memory usage tracking

### ✅ System Infrastructure
- Workspace-based project organization
- Custom ARM64 linker script
- QEMU integration for testing
- Comprehensive build system with dependency management

### ✅ Interrupt Handling
- **ARM64 Exception Vectors**: Complete 16-entry exception vector table
- **Timer Support**: 100Hz ARM Generic Timer for scheduling foundation
- **System Call Infrastructure**: SVC instruction handling and processing
- **Exception Classification**: ESR_EL1 syndrome register decoding
- **Comprehensive Testing**: Automated validation of all interrupt types

### 🚧 In Progress
- Port-based IPC message system (foundation complete)
- Process management framework (stubs implemented)

## Building and Running

### Prerequisites

```bash
# Install required tools and targets
make install-deps

# Verify dependencies
make check-deps
```

### Development Workflow

```bash
# Build the kernel
make build

# Run in QEMU (use Ctrl+A, X to exit)
make run

# Debug with GDB
make debug

# Clean build artifacts
make clean
```

### Expected Output

```
RustKernel v0.1.0 - ARM64 Microkernel
Boot: CPU primary core active
Boot: Device tree parsed successfully
Boot: Memory region: 0x0000000040000000 - 0x0000000080000000 (1024 MB)
Boot: Heap allocator initialized
Initializing memory management...
FrameAllocator: 245760 frames total, 229376 frames free
Memory: Physical frame allocator ready (229376 free / 245760 total frames)
Memory Test: ✓ Heap allocation working correctly
Memory Test: ✓ Frame allocation working correctly
Memory: Memory management system initialized
Interrupts: Initializing ARM64 interrupt handling...
Interrupts: Exception vector table at 0x0000000040088000
Interrupts: Generic timer configured for 100Hz
Interrupts: ARM64 interrupt handling initialized
Interrupt Test: ✓ Interrupts disabled
Interrupt Test: ✓ System call handling working
Interrupt Test: ✓ Timer interrupts working (0→6)
Interrupt Test: === Interrupt Statistics ===
Interrupt Test: Timer ticks: 6
```

## Technical Specifications

### Memory Architecture
- **Page Size**: 4KB with 4-level page tables
- **Virtual Address Space**: 48-bit (256TB)
- **Physical Address Space**: 44-bit (16TB)
- **Kernel Heap**: 100KB allocated at boot
- **Frame Allocation**: Bitmap-based with 8KB storage

### ARM64 Features Used
- Exception Level 1 (EL1) for kernel execution
- Translation Table Base Registers (TTBR0/1_EL1)
- Memory Attribute Indirection Register (MAIR_EL1)
- Translation Control Register (TCR_EL1)
- System Control Register (SCTLR_EL1)

## Development Status

| Component | Status | Description |
|-----------|--------|--------------|
| ✅ Project Structure | Complete | Workspace, build system, dependencies |
| ✅ Boot System | Complete | ARM64 boot, device tree, UART console |
| ✅ Memory Management | Complete | Physical/virtual memory, heap allocation |
| ✅ Interrupt Handling | Complete | Exception vectors, timer, system calls |
| 🚧 IPC System | Framework | Port-based messaging foundation |
| 🔲 Process Management | Planned | Scheduling, context switching |
| 🔲 System Calls | Planned | Complete kernel/userspace interface |
| 🔲 Userspace Services | Planned | Memory and process managers |
| 🔲 Device Drivers | Planned | Userspace driver framework |
| 🔲 UEFI Bootloader | Future | Self-contained boot solution |

## Next Milestones

1. **Interrupt Handling**: ARM64 exception vectors and timer interrupts
2. **Process Scheduler**: Round-robin scheduling with context switching
3. **System Call Interface**: Kernel/userspace communication
4. **IPC Implementation**: Complete message passing system
5. **Userspace Services**: Memory and process manager services

## Contributing

This is an educational microkernel project. The codebase is designed to be:
- **Readable**: Well-commented with clear abstractions
- **Safe**: Rust's memory safety without sacrificing performance
- **Modular**: Clean separation between kernel and userspace
- **Testable**: Comprehensive testing at each layer

## Resources

- [ARM64 Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest)
- [Device Tree Specification](https://devicetree-specification.readthedocs.io/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
