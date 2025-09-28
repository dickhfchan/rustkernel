#![no_std]
#![no_main]

extern crate alloc;

mod memory;
mod interrupts;
mod process;
mod ipc;
mod uart;
mod devicetree;
mod allocator;

use core::panic::PanicInfo;
use core::arch::global_asm;
use devicetree::parse_device_tree;

// Include the boot assembly
global_asm!(include_str!("boot.s"));

/// Main Rust entry point called from boot.s
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Initialize UART for early console output
    uart::init_uart();
    
    println!("RustKernel v0.1.0 - ARM64 Microkernel");
    println!("Boot: CPU primary core active");
    
    // Parse device tree (passed by bootloader in x0, but for QEMU we'll use known address)
    let fdt_addr = 0x40000000 as *const u8; // QEMU default FDT location
    if let Some(dt) = parse_device_tree(fdt_addr) {
        println!("Boot: Device tree parsed successfully");
        for region in dt.memory_regions() {
            if let Some(mem) = region {
                println!("Boot: Memory region: 0x{:016x} - 0x{:016x} ({} MB)", 
                    mem.start, mem.start + mem.size, mem.size / (1024 * 1024));
            }
        }
    } else {
        println!("Boot: Warning - Could not parse device tree, using defaults");
    }
    
    println!("Boot: Initializing kernel subsystems...");
    
    // Initialize heap allocator
    allocator::init_heap();
    println!("Boot: Heap allocator initialized");
    
    // Initialize core kernel subsystems
    memory::init();
    interrupts::init();
    ipc::init();
    process::init();
    
    println!("Boot: Kernel initialization complete");
    println!("Boot: Starting userspace services...");
    
    // Start core userspace services
    start_userspace();
    
    println!("Boot: Entering kernel idle loop");
    
    // Enter idle loop - kernel should only handle interrupts now
    kernel_idle();
}

fn start_userspace() {
    // TODO: Load and start memory manager service
    // TODO: Load and start process manager service
    println!("Userspace services started");
}

fn kernel_idle() -> ! {
    loop {
        // Wait for interrupts
        // TODO: Implement proper ARM64 WFI (Wait For Interrupt)
        core::hint::spin_loop();
    }
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC!");
    println!("Message: {}", info.message());
    if let Some(location) = info.location() {
        println!("Location: {}:{}", location.file(), location.line());
    }
    
    loop {
        core::hint::spin_loop();
    }
}
