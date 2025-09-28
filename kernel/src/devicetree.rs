// Basic device tree parsing for ARM64 memory discovery

use core::ptr::read_volatile;
use core::slice;

// FDT (Flattened Device Tree) header
#[repr(C)]
pub struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

#[derive(Copy, Clone, Debug)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
}

// Helper function for big-endian reads
fn read_be(ptr: *const u32) -> u32 {
    unsafe {
        u32::from_be(read_volatile(ptr))
    }
}

pub struct DeviceTree {
    header: *const FdtHeader,
    memory_regions: [Option<MemoryRegion>; 8],
    region_count: usize,
}

impl DeviceTree {
    pub fn new(fdt_addr: *const u8) -> Option<Self> {
        let header = fdt_addr as *const FdtHeader;
        
        unsafe {
            // Verify magic number
            if read_be(&(*header).magic) != FDT_MAGIC {
                return None;
            }
            
            // Check version
            let version = read_be(&(*header).version);
            if version < 16 {
                return None;
            }
        }
        
        Some(DeviceTree {
            header,
            memory_regions: [const { None }; 8],
            region_count: 0,
        })
    }
    
    pub fn parse_memory(&mut self) -> Result<(), &'static str> {
        unsafe {
            let header = &*self.header;
            let totalsize = read_be(&header.totalsize) as isize;
            let struct_offset = read_be(&header.off_dt_struct) as isize;
            
            let struct_ptr = (self.header as *const u8).offset(struct_offset) as *const u32;
            let mut current = struct_ptr;
            let end = (self.header as *const u8).offset(totalsize);
            
            while (current as *const u8) < end {
                let token = read_be(&*current);
                current = current.offset(1);
                
                match token {
                    FDT_BEGIN_NODE => {
                        // Skip node name
                        let name_ptr = current as *const u8;
                        let mut len = 0;
                        while *name_ptr.offset(len) != 0 {
                            len += 1;
                        }
                        // Align to 4 bytes
                        len = (len + 4) & !3;
                        current = (current as *const u8).offset(len) as *const u32;
                        
                        // Check if this is a memory node
                        let name = slice::from_raw_parts(name_ptr, len as usize);
                        if let Ok(name_str) = core::str::from_utf8(name) {
                            if name_str.starts_with("memory") {
                                self.parse_memory_node(&mut current)?;
                            }
                        }
                    }
                    FDT_END_NODE => {
                        // End of current node
                    }
                    FDT_PROP => {
                        // Skip property
                        let len = read_be(&*current);
                        current = current.offset(1);
                        let _nameoff = read_be(&*current);
                        current = current.offset(1);
                        // Skip property data (aligned to 4 bytes)
                        let aligned_len = (len + 3) & !3;
                        current = (current as *const u8).offset(aligned_len as isize) as *const u32;
                    }
                    FDT_NOP => {
                        // No operation
                    }
                    FDT_END => {
                        break;
                    }
                    _ => {
                        return Err("Invalid FDT token");
                    }
                }
            }
        }
        
        Ok(())
    }
    
    unsafe fn parse_memory_node(&mut self, current: &mut *const u32) -> Result<(), &'static str> {
        // Look for "reg" property in memory node
        while read_be(&**current) != FDT_END_NODE {
            let token = read_be(&**current);
            *current = current.offset(1);
            
            if token == FDT_PROP {
                let len = read_be(&**current);
                *current = current.offset(1);
                let _nameoff = read_be(&**current);
                *current = current.offset(1);
                
                // Parse reg property (address, size pairs)
                if len >= 16 && self.region_count < 8 {
                    let addr_high = read_be(&**current);
                    *current = current.offset(1);
                    let addr_low = read_be(&**current);
                    *current = current.offset(1);
                    let size_high = read_be(&**current);
                    *current = current.offset(1);
                    let size_low = read_be(&**current);
                    *current = current.offset(1);
                    
                    let start = ((addr_high as u64) << 32) | (addr_low as u64);
                    let size = ((size_high as u64) << 32) | (size_low as u64);
                    
                    self.memory_regions[self.region_count] = Some(MemoryRegion { start, size });
                    self.region_count += 1;
                } else {
                    // Skip remaining property data
                    let aligned_len = (len + 3) & !3;
                    *current = (*current as *const u8).offset(aligned_len as isize) as *const u32;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn memory_regions(&self) -> &[Option<MemoryRegion>] {
        &self.memory_regions[..self.region_count]
    }
}

pub fn parse_device_tree(fdt_addr: *const u8) -> Option<DeviceTree> {
    let mut dt = DeviceTree::new(fdt_addr)?;
    dt.parse_memory().ok()?;
    Some(dt)
}