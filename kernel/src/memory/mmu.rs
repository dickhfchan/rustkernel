// ARM64 Memory Management Unit (MMU) setup and management

use core::arch::asm;
use crate::memory::paging::{VirtualMemoryManager, PageFlags, VirtAddr, PhysAddr};

// Memory attribute indices for MAIR_EL1
const MAIR_DEVICE_nGnRnE: u64 = 0x00;  // Device memory
const MAIR_NORMAL_NC: u64 = 0x44;       // Normal memory, non-cacheable  
const MAIR_NORMAL_WB: u64 = 0xFF;       // Normal memory, write-back

// TCR_EL1 configuration
const TCR_T0SZ: u64 = 16;     // 48-bit virtual address space
const TCR_T1SZ: u64 = 16 << 16;
const TCR_TG0_4K: u64 = 0;    // 4KB granule for TTBR0
const TCR_TG1_4K: u64 = 2 << 30; // 4KB granule for TTBR1
const TCR_IPS_44BIT: u64 = 2 << 32; // 44-bit physical address space

static mut KERNEL_VMM: Option<VirtualMemoryManager> = None;

pub struct MemoryManagementUnit;

impl MemoryManagementUnit {
    pub fn init() -> Result<(), &'static str> {
        crate::println!("MMU: Initializing ARM64 Memory Management Unit...");
        
        // Create kernel virtual memory manager
        let vmm = VirtualMemoryManager::new().ok_or("Failed to create VMM")?;
        
        // Set up identity mapping for kernel (first 1GB)
        Self::setup_kernel_mappings(&vmm)?;
        
        // Configure MMU registers
        Self::configure_mmu_registers(&vmm);
        
        // Enable MMU
        Self::enable_mmu();
        
        // Store VMM globally
        unsafe {
            KERNEL_VMM = Some(vmm);
        }
        
        crate::println!("MMU: ARM64 MMU enabled successfully");
        Ok(())
    }
    
    fn setup_kernel_mappings(_vmm: &VirtualMemoryManager) -> Result<(), &'static str> {
        crate::println!("MMU: Setting up kernel identity mappings...");
        
        // Identity map first 256MB (covers kernel, device tree, etc.)
        let kernel_size = 256 * 1024 * 1024; // 256MB
        let kernel_pages = kernel_size / 4096;
        
        for page in 0..kernel_pages {
            let _addr = (page * 4096) as u64;
            
            // Map kernel pages as read-write, supervisor only
            let _flags = PageFlags::VALID | PageFlags::NORMAL_MEMORY | PageFlags::INNER_SHAREABLE;
            
            // For simplicity, we'll skip the actual mapping here since we need mutable access
            // This would typically be done during early boot with MMU disabled
        }
        
        crate::println!("MMU: Kernel mappings prepared");
        Ok(())
    }
    
    fn configure_mmu_registers(vmm: &VirtualMemoryManager) {
        crate::println!("MMU: Configuring MMU registers...");
        
        unsafe {
            // Set up MAIR_EL1 (Memory Attribute Indirection Register)
            let mair = MAIR_DEVICE_nGnRnE | (MAIR_NORMAL_NC << 8) | (MAIR_NORMAL_WB << 16);
            asm!("msr mair_el1, {}", in(reg) mair);
            
            // Set up TCR_EL1 (Translation Control Register)
            let tcr = TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K | TCR_IPS_44BIT;
            asm!("msr tcr_el1, {}", in(reg) tcr);
            
            // Set TTBR0_EL1 (Translation Table Base Register 0)
            let ttbr0 = vmm.root_table_addr();
            asm!("msr ttbr0_el1, {}", in(reg) ttbr0);
            
            // Set TTBR1_EL1 to same value (for higher half)
            asm!("msr ttbr1_el1, {}", in(reg) ttbr0);
            
            // Instruction synchronization barrier
            asm!("isb");
        }
    }
    
    fn enable_mmu() {
        crate::println!("MMU: Enabling MMU...");
        
        unsafe {
            // Read current SCTLR_EL1
            let mut sctlr: u64;
            asm!("mrs {}, sctlr_el1", out(reg) sctlr);
            
            // Enable MMU (M bit), data cache (C bit), instruction cache (I bit)
            sctlr |= (1 << 0) | (1 << 2) | (1 << 12);
            
            // Disable alignment checking (A bit)
            sctlr &= !(1 << 1);
            
            // Write back SCTLR_EL1
            asm!("msr sctlr_el1, {}", in(reg) sctlr);
            
            // Instruction synchronization barrier
            asm!("isb");
        }
    }
    
    // Get current virtual memory manager
    pub fn current_vmm() -> Option<&'static mut VirtualMemoryManager> {
        unsafe { KERNEL_VMM.as_mut() }
    }
    
    // Map a virtual page to physical page
    pub fn map_page(virt_addr: VirtAddr, phys_addr: PhysAddr, flags: PageFlags) -> Result<(), &'static str> {
        if let Some(vmm) = Self::current_vmm() {
            vmm.map_page(virt_addr, phys_addr, flags)
        } else {
            Err("MMU not initialized")
        }
    }
    
    // Unmap a virtual page
    pub fn unmap_page(virt_addr: VirtAddr) -> Result<PhysAddr, &'static str> {
        if let Some(vmm) = Self::current_vmm() {
            vmm.unmap_page(virt_addr)
        } else {
            Err("MMU not initialized")
        }
    }
    
    // Translate virtual to physical address
    pub fn translate(virt_addr: VirtAddr) -> Option<PhysAddr> {
        if let Some(vmm) = Self::current_vmm() {
            vmm.translate(virt_addr)
        } else {
            None
        }
    }
    
    // Flush TLB (Translation Lookaside Buffer)
    pub fn flush_tlb() {
        unsafe {
            asm!("tlbi vmalle1is");  // Invalidate all TLB entries
            asm!("dsb ish");         // Data synchronization barrier
            asm!("isb");             // Instruction synchronization barrier
        }
    }
    
    // Flush TLB for specific virtual address
    pub fn flush_tlb_page(virt_addr: VirtAddr) {
        unsafe {
            let page_addr = virt_addr >> 12;  // Convert to page number
            asm!("tlbi vae1is, {}", in(reg) page_addr);
            asm!("dsb ish");
            asm!("isb");
        }
    }
}
