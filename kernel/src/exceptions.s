// ARM64 Exception Vector Table
// Each exception type has 4 possible sources based on execution state and level

.section ".text.exceptions", "ax"
.align 11  // Vector table must be aligned to 2KB boundary

.global exception_vector_table
exception_vector_table:
    // Current EL with SP0
    .align 7
    b sync_invalid_el1t         // Synchronous exception
    .align 7  
    b irq_invalid_el1t          // IRQ
    .align 7
    b fiq_invalid_el1t          // FIQ  
    .align 7
    b serror_invalid_el1t       // System Error

    // Current EL with SPx
    .align 7
    b sync_current_el1h         // Synchronous exception
    .align 7
    b irq_current_el1h          // IRQ
    .align 7
    b fiq_current_el1h          // FIQ
    .align 7
    b serror_current_el1h       // System Error

    // Lower EL using AArch64
    .align 7
    b sync_lower_el_aarch64     // Synchronous exception
    .align 7
    b irq_lower_el_aarch64      // IRQ
    .align 7
    b fiq_lower_el_aarch64      // FIQ
    .align 7
    b serror_lower_el_aarch64   // System Error

    // Lower EL using AArch32
    .align 7
    b sync_lower_el_aarch32     // Synchronous exception
    .align 7
    b irq_lower_el_aarch32      // IRQ
    .align 7
    b fiq_lower_el_aarch32      // FIQ
    .align 7
    b serror_lower_el_aarch32   // System Error

// Exception handler macro to save/restore context
.macro exception_entry
    // Save general purpose registers
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x19, [sp, #-16]!
    stp x20, x21, [sp, #-16]!
    stp x22, x23, [sp, #-16]!
    stp x24, x25, [sp, #-16]!
    stp x26, x27, [sp, #-16]!
    stp x28, x29, [sp, #-16]!
    
    // Save link register and exception return address
    mrs x0, elr_el1
    mrs x1, spsr_el1
    stp x30, x0, [sp, #-16]!
    str x1, [sp, #-8]!
.endm

.macro exception_exit
    // Restore SPSR and ELR
    ldr x1, [sp], #8
    ldp x30, x0, [sp], #16
    msr spsr_el1, x1
    msr elr_el1, x0
    
    // Restore general purpose registers
    ldp x28, x29, [sp], #16
    ldp x26, x27, [sp], #16
    ldp x24, x25, [sp], #16
    ldp x22, x23, [sp], #16
    ldp x20, x21, [sp], #16
    ldp x18, x19, [sp], #16
    ldp x16, x17, [sp], #16
    ldp x14, x15, [sp], #16
    ldp x12, x13, [sp], #16
    ldp x10, x11, [sp], #16
    ldp x8, x9, [sp], #16
    ldp x6, x7, [sp], #16
    ldp x4, x5, [sp], #16
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
    
    eret
.endm

// Invalid exception handlers (should not occur)
sync_invalid_el1t:
    mov x0, #0
    b handle_invalid_exception

irq_invalid_el1t:
    mov x0, #1  
    b handle_invalid_exception

fiq_invalid_el1t:
    mov x0, #2
    b handle_invalid_exception

serror_invalid_el1t:
    mov x0, #3
    b handle_invalid_exception

// Current EL (EL1) exception handlers
sync_current_el1h:
    exception_entry
    mov x0, sp
    bl handle_sync_exception
    exception_exit

irq_current_el1h:
    exception_entry
    mov x0, sp
    bl handle_irq_exception
    exception_exit

fiq_current_el1h:
    exception_entry
    mov x0, sp
    bl handle_fiq_exception  
    exception_exit

serror_current_el1h:
    exception_entry
    mov x0, sp
    bl handle_serror_exception
    exception_exit

// Lower EL (EL0) AArch64 exception handlers
sync_lower_el_aarch64:
    exception_entry
    mov x0, sp
    bl handle_sync_exception
    exception_exit

irq_lower_el_aarch64:
    exception_entry
    mov x0, sp
    bl handle_irq_exception
    exception_exit

fiq_lower_el_aarch64:
    exception_entry
    mov x0, sp
    bl handle_fiq_exception
    exception_exit

serror_lower_el_aarch64:
    exception_entry
    mov x0, sp
    bl handle_serror_exception
    exception_exit

// Lower EL (EL0) AArch32 exception handlers  
sync_lower_el_aarch32:
    exception_entry
    mov x0, sp
    bl handle_sync_exception
    exception_exit

irq_lower_el_aarch32:
    exception_entry
    mov x0, sp
    bl handle_irq_exception
    exception_exit

fiq_lower_el_aarch32:
    exception_entry
    mov x0, sp
    bl handle_fiq_exception
    exception_exit

serror_lower_el_aarch32:
    exception_entry
    mov x0, sp
    bl handle_serror_exception
    exception_exit

// Invalid exception handler
handle_invalid_exception:
    // x0 contains exception type
    bl rust_handle_invalid_exception
    // Should not return
    b .