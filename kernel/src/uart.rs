// ARM64 PL011 UART driver for QEMU virt machine

use core::fmt::{Arguments, Write};
use core::ptr::{read_volatile, write_volatile};

// QEMU virt machine UART base address
const UART_BASE: *mut u32 = 0x09000000 as *mut u32;

// UART register offsets
const UART_DR: isize = 0x00;     // Data Register
const UART_FR: isize = 0x06;     // Flag Register
const UART_IBRD: isize = 0x09;   // Integer Baud Rate Divisor
const UART_FBRD: isize = 0x0A;   // Fractional Baud Rate Divisor
const UART_LCRH: isize = 0x0B;   // Line Control Register
const UART_CR: isize = 0x0C;     // Control Register

// Flag register bits
const UART_FR_TXFF: u32 = 1 << 5; // Transmit FIFO full
const UART_FR_RXFE: u32 = 1 << 4; // Receive FIFO empty

// Control register bits
const UART_CR_UARTEN: u32 = 1 << 0; // UART enable
const UART_CR_TXE: u32 = 1 << 8;    // Transmit enable
const UART_CR_RXE: u32 = 1 << 9;    // Receive enable

// Line control register bits
const UART_LCRH_WLEN_8: u32 = 3 << 5; // 8-bit words
const UART_LCRH_FEN: u32 = 1 << 4;    // FIFO enable

pub struct Uart {
    base: *mut u32,
}

impl Uart {
    pub const fn new() -> Self {
        Self {
            base: UART_BASE,
        }
    }
    
    pub fn init(&self) {
        unsafe {
            // Disable UART
            write_volatile(self.base.offset(UART_CR), 0);
            
            // Set baud rate (38400 for 24MHz clock)
            write_volatile(self.base.offset(UART_IBRD), 39);
            write_volatile(self.base.offset(UART_FBRD), 0);
            
            // Configure line: 8N1, enable FIFO
            write_volatile(self.base.offset(UART_LCRH), 
                UART_LCRH_WLEN_8 | UART_LCRH_FEN);
            
            // Enable UART, transmit, and receive
            write_volatile(self.base.offset(UART_CR), 
                UART_CR_UARTEN | UART_CR_TXE | UART_CR_RXE);
        }
    }
    
    pub fn put_char(&self, c: u8) {
        unsafe {
            // Wait until transmit FIFO is not full
            while read_volatile(self.base.offset(UART_FR)) & UART_FR_TXFF != 0 {}
            
            // Write character to data register
            write_volatile(self.base.offset(UART_DR), c as u32);
        }
    }
    
    pub fn get_char(&self) -> Option<u8> {
        unsafe {
            // Check if receive FIFO is empty
            if read_volatile(self.base.offset(UART_FR)) & UART_FR_RXFE != 0 {
                None
            } else {
                Some(read_volatile(self.base.offset(UART_DR)) as u8)
            }
        }
    }
    
    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.put_char(b'\r');
            }
            self.put_char(byte);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.puts(s);
        Ok(())
    }
}

// Global UART instance
static mut UART: Uart = Uart::new();

pub fn init_uart() {
    unsafe {
        UART.init();
    }
}

pub fn print_args(args: Arguments) {
    unsafe {
        let _ = UART.write_fmt(args);
    }
}

// Export macros for early printing
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::uart::print_args(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}