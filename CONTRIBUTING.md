# Contributing to RustKernel

Thank you for your interest in contributing to RustKernel! This is an educational microkernel project focused on learning ARM64 systems programming with Rust.

## Development Setup

### Prerequisites

1. **Rust toolchain** with nightly features:
   ```bash
   rustup target add aarch64-unknown-none
   rustup component add rust-src
   ```

2. **QEMU** for ARM64 emulation:
   ```bash
   # macOS
   brew install qemu
   
   # Ubuntu/Debian  
   apt-get install qemu-system-arm
   ```

3. **Build tools**:
   ```bash
   make check-deps  # Verify all dependencies
   ```

### Building and Testing

```bash
# Build the kernel
make build

# Run in QEMU
make run

# Debug with GDB
make debug

# Clean build artifacts
make clean
```

## Project Architecture

RustKernel follows a microkernel design with these key principles:

- **Minimal kernel**: Only essential functions in kernel space
- **Service-oriented**: OS services run as userspace processes  
- **Memory safety**: Rust ownership prevents memory corruption
- **Hardware isolation**: ARM64 memory protection and privilege levels

## Code Organization

```
kernel/src/
├── main.rs           # Kernel entry point
├── boot.s            # ARM64 assembly boot code
├── memory/           # Memory management subsystem
│   ├── frame_allocator.rs  # Physical memory allocation
│   ├── paging.rs     # Virtual memory management
│   ├── mmu.rs        # ARM64 MMU control
│   └── test.rs       # Memory testing suite
├── uart.rs           # Console driver
├── devicetree.rs     # Hardware discovery
├── interrupts.rs     # Exception handling
├── process.rs        # Process management
└── ipc.rs            # Inter-process communication
```

## Coding Standards

### Rust Guidelines
- Use `#![no_std]` for kernel code
- Prefer `Result<T, E>` over panics for error handling
- Document public APIs with `///` comments
- Use `unsafe` blocks only when necessary and document why
- Follow Rust naming conventions (snake_case, CamelCase, etc.)

### ARM64 Specifics
- Use proper memory barriers (`dsb`, `isb`) for hardware operations
- Align data structures to hardware requirements
- Use volatile operations for memory-mapped I/O
- Comment assembly code thoroughly

### Testing
- Add tests for new memory management features
- Verify allocation/deallocation correctness  
- Include boundary condition testing
- Document expected behavior

## Contribution Process

### 1. Fork and Clone
```bash
git clone https://github.com/yourusername/rustkernel.git
cd rustkernel
```

### 2. Create Feature Branch
```bash
git checkout -b feature/interrupt-handling
```

### 3. Implement Changes
- Follow the coding standards above
- Add tests for new functionality
- Update documentation as needed
- Ensure `make build` passes without warnings

### 4. Commit Changes
Use conventional commit messages:
```bash
git commit -m "feat: implement ARM64 exception vectors

- Add exception vector table for EL1
- Implement synchronous exception handlers
- Add timer interrupt support
- Include comprehensive testing"
```

### 5. Submit Pull Request
- Describe the changes and motivation
- Reference any related issues
- Include testing instructions
- Request review from maintainers

## Development Areas

### Current Priorities
1. **Interrupt Handling**: ARM64 exception vectors and GIC support
2. **Process Scheduler**: Context switching and round-robin scheduling
3. **System Calls**: Kernel/userspace communication interface
4. **IPC Implementation**: Complete message passing system

### Future Areas
- Device driver framework
- File system support
- Network stack
- Performance optimization

## Testing

### Memory Management Tests
The project includes automated testing for:
- Physical frame allocation/deallocation
- Heap allocation verification
- Statistics validation
- Memory leak detection

### Adding New Tests
```rust
// In kernel/src/memory/test.rs
pub fn test_new_feature() {
    crate::println!("Testing new feature...");
    
    // Test implementation
    assert!(condition, "Test failure message");
    
    crate::println!("✓ New feature test passed");
}
```

### Running Tests
Tests run automatically during kernel boot. Check console output for results.

## Documentation

### Code Documentation
- Document all public APIs
- Explain non-obvious implementation decisions
- Include usage examples for complex functions
- Reference ARM64 manual sections where relevant

### Architecture Documentation
- Update README.md for major changes
- Add entries to CHANGELOG.md
- Include diagrams for complex subsystems
- Document design decisions and trade-offs

## Questions and Support

### Getting Help
- Check existing documentation first
- Search closed issues for similar problems
- Ask questions in GitHub Discussions
- Reference ARM64 Architecture Reference Manual

### Reporting Issues
Include:
- Rust version and target information
- QEMU version and command line
- Complete error messages and logs
- Steps to reproduce the issue

## License

By contributing to RustKernel, you agree that your contributions will be licensed under the MIT License.

## Acknowledgments

This project is inspired by:
- [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann
- [The ARM64 Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest)
- [QEMU ARM System Emulation](https://qemu.readthedocs.io/en/latest/system/target-arm.html)

Thank you for contributing to RustKernel! 🦀🔧