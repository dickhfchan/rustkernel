.section ".text.boot"

.global _start
.extern rust_main
.extern __bss_start
.extern __bss_end

// ARM64 boot entry point
_start:
    // Disable interrupts
    msr daifset, #0xf
    
    // Check if we're running on the primary CPU (MPIDR_EL1)
    mrs x0, mpidr_el1
    and x0, x0, #0xFF
    cbnz x0, halt         // If not CPU 0, halt
    
    // Set up stack pointer
    ldr x0, =_stack_top
    mov sp, x0
    
    // Clear BSS section
    ldr x0, =__bss_start
    ldr x1, =__bss_end
    
clear_bss_loop:
    cmp x0, x1
    b.ge clear_bss_done
    str xzr, [x0], #8
    b clear_bss_loop
    
clear_bss_done:
    // Jump to Rust main function
    bl rust_main
    
halt:
    // Infinite loop if we ever return or on secondary CPUs
    wfe
    b halt

.section ".bss"
.align 12
_stack_bottom:
    .space 0x10000  // 64KB stack
_stack_top: