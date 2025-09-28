// ARM64 paging implementation using 4-level page tables

use bitflags::bitflags;
use crate::memory::frame_allocator::allocate_frame;

// Virtual address type
pub type VirtAddr = u64;

// Physical address type  
pub type PhysAddr = u64;

// ARM64 page table entry
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

bitflags! {
    /// ARM64 page table entry flags
    pub struct PageFlags: u64 {
        const VALID = 1 << 0;
        const TABLE = 1 << 1;  // For intermediate levels
        const PAGE = 0 << 1;   // For final level (page)
        const USER = 1 << 6;   // User accessible
        const READ_WRITE = 0 << 7;  // Read-write (default)
        const READ_ONLY = 1 << 7;   // Read-only
        const INNER_SHAREABLE = 3 << 8;
        const OUTER_SHAREABLE = 2 << 8;
        const NON_SHAREABLE = 0 << 8;
        const NORMAL_MEMORY = 0 << 2;  // Normal memory
        const DEVICE_MEMORY = 1 << 2;  // Device memory
    }
}

impl PageTableEntry {
    pub fn new(addr: PhysAddr, flags: PageFlags) -> Self {
        // Ensure address is page-aligned
        let aligned_addr = addr & !0xFFF;
        Self(aligned_addr | flags.bits())
    }
    
    pub fn empty() -> Self {
        Self(0)
    }
    
    pub fn is_valid(&self) -> bool {
        (self.0 & PageFlags::VALID.bits()) != 0
    }
    
    pub fn physical_addr(&self) -> PhysAddr {
        self.0 & 0x0000FFFFFFFFF000  // Extract physical address
    }
    
    pub fn flags(&self) -> PageFlags {
        PageFlags::from_bits_truncate(self.0)
    }
    
    pub fn set_addr(&mut self, addr: PhysAddr, flags: PageFlags) {
        let aligned_addr = addr & !0xFFF;
        self.0 = aligned_addr | flags.bits();
    }
}

// ARM64 page table (512 entries for 4KB pages)
#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: [PageTableEntry::empty(); 512],
        }
    }
    
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = PageTableEntry::empty();
        }
    }
    
    pub fn get_entry(&self, index: usize) -> Option<&PageTableEntry> {
        self.entries.get(index)
    }
    
    pub fn get_entry_mut(&mut self, index: usize) -> Option<&mut PageTableEntry> {
        self.entries.get_mut(index)
    }
    
    // Get page table entry for next level
    pub fn get_next_table(&self, index: usize) -> Option<&'static mut PageTable> {
        if let Some(entry) = self.get_entry(index) {
            if entry.is_valid() {
                let addr = entry.physical_addr();
                return Some(unsafe { &mut *(addr as *mut PageTable) });
            }
        }
        None
    }
    
    // Create next level page table
    pub fn create_next_table(&mut self, index: usize) -> Option<&'static mut PageTable> {
        if let Some(frame) = allocate_frame() {
            let table_addr = frame.as_ptr() as u64;
            
            // Initialize new page table
            let new_table = unsafe { &mut *(table_addr as *mut PageTable) };
            new_table.zero();
            
            // Set entry to point to new table
            if let Some(entry) = self.get_entry_mut(index) {
                *entry = PageTableEntry::new(
                    table_addr,
                    PageFlags::VALID | PageFlags::TABLE
                );
                return Some(new_table);
            }
        }
        None
    }
}

// Virtual memory manager
pub struct VirtualMemoryManager {
    root_table: &'static mut PageTable,
}

impl VirtualMemoryManager {
    pub fn new() -> Option<Self> {
        // Allocate root page table
        if let Some(frame) = allocate_frame() {
            let root_addr = frame.as_ptr() as u64;
            let root_table = unsafe { &mut *(root_addr as *mut PageTable) };
            root_table.zero();
            
            return Some(Self { root_table });
        }
        None
    }
    
    // Map a virtual page to a physical frame
    pub fn map_page(&mut self, virt_addr: VirtAddr, phys_addr: PhysAddr, flags: PageFlags) -> Result<(), &'static str> {
        let indices = self.get_page_table_indices(virt_addr);
        
        // Walk through page table levels
        let mut current_table = &mut *self.root_table;
        
        // L0 -> L1 -> L2 (intermediate levels)
        for &index in &indices[0..3] {
            current_table = match current_table.get_next_table(index) {
                Some(table) => table,
                None => current_table.create_next_table(index).ok_or("Failed to create page table")?,
            };
        }
        
        // L3 - final level (actual page mapping)
        let page_index = indices[3];
        if let Some(entry) = current_table.get_entry_mut(page_index) {
            if entry.is_valid() {
                return Err("Page already mapped");
            }
            *entry = PageTableEntry::new(phys_addr, flags | PageFlags::VALID);
            Ok(())
        } else {
            Err("Invalid page table index")
        }
    }
    
    // Unmap a virtual page
    pub fn unmap_page(&mut self, virt_addr: VirtAddr) -> Result<PhysAddr, &'static str> {
        let indices = self.get_page_table_indices(virt_addr);
        
        // Walk through page table levels
        let mut current_table = &mut *self.root_table;
        
        for &index in &indices[0..3] {
            current_table = current_table.get_next_table(index).ok_or("Page not mapped")?;
        }
        
        // Get final page entry
        let page_index = indices[3];
        if let Some(entry) = current_table.get_entry_mut(page_index) {
            if !entry.is_valid() {
                return Err("Page not mapped");
            }
            
            let phys_addr = entry.physical_addr();
            *entry = PageTableEntry::empty();
            
            // TODO: TLB invalidation
            
            Ok(phys_addr)
        } else {
            Err("Invalid page table index")
        }
    }
    
    // Get page table indices for 4-level paging
    fn get_page_table_indices(&self, virt_addr: VirtAddr) -> [usize; 4] {
        [
            ((virt_addr >> 39) & 0x1FF) as usize,  // L0 index
            ((virt_addr >> 30) & 0x1FF) as usize,  // L1 index  
            ((virt_addr >> 21) & 0x1FF) as usize,  // L2 index
            ((virt_addr >> 12) & 0x1FF) as usize,  // L3 index
        ]
    }
    
    // Get physical address for virtual address
    pub fn translate(&self, virt_addr: VirtAddr) -> Option<PhysAddr> {
        let indices = self.get_page_table_indices(virt_addr);
        let offset = virt_addr & 0xFFF;  // Page offset
        
        let mut current_table = &*self.root_table;
        
        // Walk through page table levels
        for &index in &indices[0..3] {
            // For read-only access, we need to be more careful about borrowing
            current_table = unsafe {
                let addr = current_table.get_entry(index)?.physical_addr();
                &*(addr as *const PageTable)
            };
        }
        
        // Get final page entry
        let page_index = indices[3];
        let entry = current_table.get_entry(page_index)?;
        
        if entry.is_valid() {
            Some(entry.physical_addr() + offset)
        } else {
            None
        }
    }
    
    // Get root page table physical address for TTBR register
    pub fn root_table_addr(&self) -> PhysAddr {
        self.root_table as *const _ as u64
    }
}
