# Changelog

All notable changes to the RustKernel project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Planned
- ARM64 interrupt handling system
- Process scheduler with context switching
- System call interface
- Complete IPC implementation
- Userspace service framework

## [0.3.0] - 2025-09-28

### Added - Memory Management System
- **Physical Frame Allocator**: Bitmap-based allocation for 4KB frames
  - Efficient O(1) free frame tracking with allocation hints
  - Support for up to 256MB RAM with 8KB bitmap storage
  - Thread-safe implementation with Mutex protection
  - Real-time allocation statistics
- **Virtual Memory Management**: Complete ARM64 paging implementation
  - 4-level page table support (L0→L1→L2→L3)
  - 48-bit virtual address space, 44-bit physical address space
  - Page table entry flags for memory protection and caching
  - Automatic page table allocation and deallocation
  - Virtual-to-physical address translation
- **Memory Management Unit (MMU)**: ARM64 hardware control
  - MAIR_EL1, TCR_EL1, TTBR0/1_EL1 register configuration
  - Identity mapping setup for kernel code and data
  - TLB (Translation Lookaside Buffer) management functions
  - Hardware MMU enable/disable controls (prepared but not active)
- **Memory Testing Suite**: Comprehensive validation framework
  - Physical frame allocation/deallocation testing
  - Heap allocation verification using Vec
  - Automated test execution during kernel boot
  - Statistics validation and error detection
- **Memory Discovery**: Enhanced device tree parsing
  - Automatic memory region detection from FDT
  - Fallback configuration for unknown hardware
  - Integration with frame allocator initialization

### Technical Details
- Frame allocator supports 229,376 free frames out of 245,760 total (94% efficiency)
- Kernel heap: 100KB allocated at 0x4444_4444_0000
- Memory layout: Identity mapping for first 256MB kernel space
- Page size: 4KB with ARM64 standard alignment
- Bitmap storage: 8KB static allocation for frame tracking

## [0.2.0] - 2025-09-28

### Added - Boot System and Hardware Support
- **ARM64 Boot Sequence**: Complete assembly-to-Rust transition
  - Primary CPU detection using MPIDR_EL1 register
  - 64KB dedicated kernel stack setup
  - BSS section initialization and clearing
  - Exception Level 1 (EL1) kernel execution
- **Device Tree Integration**: FDT parsing for hardware discovery
  - Memory region detection and parsing
  - Big-endian data handling for ARM64 compatibility
  - QEMU virt machine device tree support
  - Automatic hardware configuration discovery
- **UART Console Driver**: PL011 UART implementation
  - Early debugging output with formatted printing
  - Baud rate configuration (38400 bps for 24MHz clock)
  - FIFO enable with 8-bit word length
  - Transmit/receive functionality (receive unused currently)
- **Kernel Heap Allocator**: Dynamic memory allocation
  - `linked_list_allocator` integration for no_std environment
  - 100KB heap space at fixed virtual address
  - Global allocator setup for `alloc` crate support

### Infrastructure
- Custom ARM64 linker script with proper memory layout
- QEMU integration with virt machine target
- Build system automation with dependency checking
- Debug configuration with GDB server support

## [0.1.0] - 2025-09-28

### Added - Project Foundation
- **Project Structure**: Rust workspace with microkernel architecture
  - Kernel core with no_std configuration
  - Userspace runtime library framework
  - Service-oriented design with memory/process managers
  - UEFI bootloader preparation (future implementation)
- **Build System**: Comprehensive development environment
  - ARM64 target configuration (aarch64-unknown-none)
  - Custom build scripts and Makefile automation
  - QEMU testing environment setup
  - Dependency management and verification
- **IPC Framework**: Port-based message passing foundation
  - Asynchronous message queue design
  - Process and port identifier types
  - Message structure with fixed-size data payload
  - Thread-safe port implementation with Mutex protection
- **Core Kernel Modules**: Placeholder implementations
  - Process management framework
  - Interrupt handling system preparation
  - Memory management module structure
  - Clean module organization and interfaces

### Development Environment
- Rust 2021 edition with latest stable compiler
- ARM64 cross-compilation support
- QEMU system emulation for testing
- Homebrew integration for macOS development
- Comprehensive documentation and README

### Design Decisions
- Microkernel architecture with minimal kernel space
- Rust memory safety without runtime overhead
- Service isolation for fault tolerance
- Port-based IPC for clean abstraction
- Device tree support for hardware portability

---

## Development Milestones

### Phase 1: Foundation ✅
- [x] Project structure and build system
- [x] ARM64 boot sequence and hardware initialization
- [x] Memory management system with allocation and paging

### Phase 2: Core Services 🚧
- [ ] ARM64 interrupt handling and exception vectors
- [ ] Process scheduler with context switching
- [ ] System call interface for kernel/userspace communication
- [ ] Complete IPC implementation with message queues

### Phase 3: Userspace 🔲
- [ ] Userspace runtime library
- [ ] Memory manager service
- [ ] Process manager service
- [ ] Device driver framework

### Phase 4: Integration 🔲
- [ ] Service discovery and lifecycle management
- [ ] Advanced memory features (shared memory, memory mapping)
- [ ] Performance optimization and testing
- [ ] UEFI bootloader implementation