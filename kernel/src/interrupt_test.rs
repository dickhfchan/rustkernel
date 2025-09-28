// Interrupt handling testing utilities

use crate::interrupts::{get_interrupt_stats, test_system_call, disable_interrupts, enable_interrupts};

pub fn test_interrupt_system() {
    crate::println!("Interrupt Test: Starting interrupt system tests...");
    
    // Test interrupt enable/disable
    test_interrupt_control();
    
    // Test system call handling
    test_syscall_handling();
    
    // Test timer interrupts
    test_timer_functionality();
    
    // Display interrupt statistics
    display_interrupt_stats();
    
    crate::println!("Interrupt Test: All interrupt tests completed");
}

fn test_interrupt_control() {
    crate::println!("Interrupt Test: Testing interrupt control...");
    
    // Test disabling and enabling interrupts
    disable_interrupts();
    crate::println!("Interrupt Test: ✓ Interrupts disabled");
    
    enable_interrupts();
    crate::println!("Interrupt Test: ✓ Interrupts enabled");
    
    crate::println!("Interrupt Test: Interrupt control test completed");
}

fn test_syscall_handling() {
    crate::println!("Interrupt Test: Testing system call handling...");
    
    let (_, sync_before, _, _, _) = get_interrupt_stats();
    
    // Trigger a test system call
    test_system_call();
    
    let (_, sync_after, _, _, _) = get_interrupt_stats();
    
    if sync_after > sync_before {
        crate::println!("Interrupt Test: ✓ System call handling working");
    } else {
        crate::println!("Interrupt Test: ✗ System call not detected");
    }
    
    crate::println!("Interrupt Test: System call test completed");
}

fn test_timer_functionality() {
    crate::println!("Interrupt Test: Testing timer functionality...");
    
    let (_, _, _, _, timer_before) = get_interrupt_stats();
    
    // Wait for timer interrupts (simple delay)
    for _ in 0..1000000 {
        core::hint::spin_loop();
    }
    
    let (_, _, _, _, timer_after) = get_interrupt_stats();
    
    if timer_after > timer_before {
        crate::println!("Interrupt Test: ✓ Timer interrupts working ({}→{})", 
                       timer_before, timer_after);
    } else {
        crate::println!("Interrupt Test: ✗ No timer interrupts detected");
    }
    
    crate::println!("Interrupt Test: Timer test completed");
}

fn display_interrupt_stats() {
    let (irq, sync, fiq, serror, timer) = get_interrupt_stats();
    
    crate::println!("Interrupt Test: === Interrupt Statistics ===");
    crate::println!("Interrupt Test: IRQ interrupts: {}", irq);
    crate::println!("Interrupt Test: Sync exceptions: {}", sync);
    crate::println!("Interrupt Test: FIQ interrupts: {}", fiq);
    crate::println!("Interrupt Test: System errors: {}", serror);
    crate::println!("Interrupt Test: Timer ticks: {}", timer);
    crate::println!("Interrupt Test: ==============================");
}
