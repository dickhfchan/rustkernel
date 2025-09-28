// ARM64 interrupt handling and exception management

use core::arch::asm;
use spin::Mutex;

// Exception context saved by assembly handler
#[repr(C)]
pub struct ExceptionContext {
    // Saved by exception_entry macro
    pub spsr_el1: u64,
    pub elr_el1: u64,
    pub x30: u64,   // Link register
    pub x29: u64,   // Frame pointer
    pub x28: u64,
    pub x27: u64,
    pub x26: u64,
    pub x25: u64,
    pub x24: u64,
    pub x23: u64,
    pub x22: u64,
    pub x21: u64,
    pub x20: u64,
    pub x19: u64,
    pub x18: u64,
    pub x17: u64,
    pub x16: u64,
    pub x15: u64,
    pub x14: u64,
    pub x13: u64,
    pub x12: u64,
    pub x11: u64,
    pub x10: u64,
    pub x9: u64,
    pub x8: u64,
    pub x7: u64,
    pub x6: u64,
    pub x5: u64,
    pub x4: u64,
    pub x3: u64,
    pub x2: u64,
    pub x1: u64,
    pub x0: u64,
}

// Exception syndrome register (ESR_EL1) decoding
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ExceptionClass {
    UnknownReason = 0b000000,
    WfiWfe = 0b000001,
    DataAbortLowerEl = 0b100100,
    DataAbortCurrentEl = 0b100101,
    InstructionAbortLowerEl = 0b100000,
    InstructionAbortCurrentEl = 0b100001,
    SvcAarch64 = 0b010101,
    SvcAarch32 = 0b010001,
    Other(u8),
}

impl From<u8> for ExceptionClass {
    fn from(ec: u8) -> Self {
        match ec {
            0b000000 => ExceptionClass::UnknownReason,
            0b000001 => ExceptionClass::WfiWfe,
            0b100100 => ExceptionClass::DataAbortLowerEl,
            0b100101 => ExceptionClass::DataAbortCurrentEl,
            0b100000 => ExceptionClass::InstructionAbortLowerEl,
            0b100001 => ExceptionClass::InstructionAbortCurrentEl,
            0b010101 => ExceptionClass::SvcAarch64,
            0b010001 => ExceptionClass::SvcAarch32,
            other => ExceptionClass::Other(other),
        }
    }
}

// Interrupt statistics
static INTERRUPT_STATS: Mutex<InterruptStats> = Mutex::new(InterruptStats::new());

struct InterruptStats {
    irq_count: u64,
    sync_exceptions: u64,
    fiq_count: u64,
    serror_count: u64,
    timer_ticks: u64,
}

impl InterruptStats {
    const fn new() -> Self {
        Self {
            irq_count: 0,
            sync_exceptions: 0,
            fiq_count: 0,
            serror_count: 0,
            timer_ticks: 0,
        }
    }
}

// External symbols from assembly
extern "C" {
    static exception_vector_table: u8;
}

// Assembly calls these Rust functions
#[no_mangle]
extern "C" fn handle_sync_exception(ctx: *const ExceptionContext) {
    let ctx = unsafe { &*ctx };
    
    INTERRUPT_STATS.lock().sync_exceptions += 1;
    
    // Read exception syndrome register
    let esr: u64;
    unsafe {
        asm!("mrs {}, esr_el1", out(reg) esr);
    }
    
    let exception_class = ExceptionClass::from(((esr >> 26) & 0x3F) as u8);
    let iss = esr & 0x1FFFFFF;  // Instruction Specific Syndrome
    
    match exception_class {
        ExceptionClass::SvcAarch64 => {
            handle_system_call(ctx, iss);
        }
        ExceptionClass::DataAbortCurrentEl | ExceptionClass::DataAbortLowerEl => {
            handle_data_abort(ctx, esr);
        }
        ExceptionClass::InstructionAbortCurrentEl | ExceptionClass::InstructionAbortLowerEl => {
            handle_instruction_abort(ctx, esr);
        }
        ExceptionClass::WfiWfe => {
            // WFI/WFE instructions - just continue
            crate::println!("Interrupts: WFI/WFE instruction handled");
        }
        _ => {
            crate::println!("Interrupts: Unhandled sync exception: {:?}, ISS: 0x{:x}", 
                           exception_class, iss);
            crate::println!("Interrupts: PC: 0x{:016x}, SP: 0x{:016x}", ctx.elr_el1, ctx as *const _ as u64);
        }
    }
}

#[no_mangle]
extern "C" fn handle_irq_exception(_ctx: *const ExceptionContext) {
    INTERRUPT_STATS.lock().irq_count += 1;
    
    // Handle timer interrupt if enabled
    if is_timer_pending() {
        handle_timer_interrupt();
    }
    
    // Handle other IRQ sources
    // TODO: Add GIC interrupt handling
}

#[no_mangle]
extern "C" fn handle_fiq_exception(_ctx: *const ExceptionContext) {
    INTERRUPT_STATS.lock().fiq_count += 1;
    crate::println!("Interrupts: FIQ received");
}

#[no_mangle]
extern "C" fn handle_serror_exception(ctx: *const ExceptionContext) {
    let ctx = unsafe { &*ctx };
    INTERRUPT_STATS.lock().serror_count += 1;
    crate::println!("Interrupts: System Error at PC: 0x{:016x}", ctx.elr_el1);
}

#[no_mangle]
extern "C" fn rust_handle_invalid_exception(exception_type: u64) -> ! {
    crate::println!("FATAL: Invalid exception type {} occurred!", exception_type);
    crate::println!("This should never happen in normal operation.");
    
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}

fn handle_system_call(ctx: &ExceptionContext, syscall_num: u64) {
    crate::println!("Interrupts: System call {} from PC: 0x{:016x}", 
                   syscall_num, ctx.elr_el1);
    // TODO: Implement system call dispatching
}

fn handle_data_abort(ctx: &ExceptionContext, esr: u64) {
    let far: u64;
    unsafe {
        asm!("mrs {}, far_el1", out(reg) far);
    }
    
    crate::println!("Interrupts: Data abort at address 0x{:016x}, PC: 0x{:016x}", 
                   far, ctx.elr_el1);
    crate::println!("Interrupts: ESR: 0x{:016x}", esr);
}

fn handle_instruction_abort(ctx: &ExceptionContext, esr: u64) {
    crate::println!("Interrupts: Instruction abort at PC: 0x{:016x}", ctx.elr_el1);
    crate::println!("Interrupts: ESR: 0x{:016x}", esr);
}

// ARM Generic Timer support
const TIMER_FREQ_HZ: u64 = 100;  // 100 Hz timer (10ms interval)

fn is_timer_pending() -> bool {
    let cntp_ctl: u64;
    unsafe {
        asm!("mrs {}, cntp_ctl_el0", out(reg) cntp_ctl);
    }
    (cntp_ctl & 0x4) != 0  // ISTATUS bit
}

fn handle_timer_interrupt() {
    INTERRUPT_STATS.lock().timer_ticks += 1;
    
    // Clear timer interrupt by setting IMASK
    unsafe {
        asm!("mrs x0, cntp_ctl_el0");
        asm!("orr x0, x0, #2");        // Set IMASK bit
        asm!("msr cntp_ctl_el0, x0");
        asm!("mrs x0, cntp_ctl_el0");
        asm!("bic x0, x0, #2");        // Clear IMASK bit (bit clear)
        asm!("msr cntp_ctl_el0, x0");
    }
    
    // Set next timer interrupt
    setup_timer_interrupt();
    
    let stats = INTERRUPT_STATS.lock();
    if stats.timer_ticks % 100 == 0 {  // Every second
        crate::println!("Interrupts: Timer tick #{} ({}s uptime)", 
                       stats.timer_ticks, stats.timer_ticks / TIMER_FREQ_HZ);
    }
}

fn setup_timer_interrupt() {
    unsafe {
        // Get current counter value
        let current_count: u64;
        asm!("mrs {}, cntpct_el0", out(reg) current_count);
        
        // Get timer frequency
        let freq: u64;
        asm!("mrs {}, cntfrq_el0", out(reg) freq);
        
        // Set compare value for next interrupt (10ms from now)
        let next_interrupt = current_count + (freq / TIMER_FREQ_HZ);
        asm!("msr cntp_cval_el0, {}", in(reg) next_interrupt);
        
        // Enable timer
        asm!("mov x0, #1");           // Enable bit
        asm!("msr cntp_ctl_el0, x0");
    }
}

pub fn init() {
    crate::println!("Interrupts: Initializing ARM64 interrupt handling...");
    
    // Set up exception vector table
    unsafe {
        let vector_addr = &exception_vector_table as *const u8 as u64;
        crate::println!("Interrupts: Exception vector table at 0x{:016x}", vector_addr);
        asm!("msr vbar_el1, {}", in(reg) vector_addr);
    }
    
    // Configure timer
    setup_timer_interrupt();
    crate::println!("Interrupts: Generic timer configured for {}Hz", TIMER_FREQ_HZ);
    
    // Enable interrupts
    unsafe {
        // Clear interrupt mask bits in DAIF
        asm!("msr daifclr, #0xF");  // Enable all interrupt types
    }
    
    crate::println!("Interrupts: ARM64 interrupt handling initialized");
    crate::println!("Interrupts: All interrupt types enabled (IRQ, FIQ, SError)");
}

pub fn disable_interrupts() {
    unsafe {
        asm!("msr daifset, #0xF");  // Disable all interrupts
    }
}

pub fn enable_interrupts() {
    unsafe {
        asm!("msr daifclr, #0xF");  // Enable all interrupts
    }
}

pub fn get_interrupt_stats() -> (u64, u64, u64, u64, u64) {
    let stats = INTERRUPT_STATS.lock();
    (stats.irq_count, stats.sync_exceptions, stats.fiq_count, 
     stats.serror_count, stats.timer_ticks)
}

// Test function to trigger a system call
pub fn test_system_call() {
    crate::println!("Interrupts: Testing system call...");
    unsafe {
        asm!("svc #42");  // System call with immediate value 42
    }
}
