pub mod allocator;
pub mod paging;
pub mod frame_allocator;
pub mod mmu;
pub mod test;

use crate::devicetree::parse_device_tree;
use frame_allocator::init_frame_allocator;

/// Initialize memory management subsystem
pub fn init() {
    crate::println!("Initializing memory management...");
    
    // Parse device tree to discover memory regions
    let fdt_addr = 0x40000000 as *const u8; // QEMU default FDT location
    if let Some(dt) = parse_device_tree(fdt_addr) {
        // Extract non-None memory regions into a fixed array
        let mut memory_regions = [None; 8];
        let mut region_count = 0;
        
        for region in dt.memory_regions() {
            if let Some(mem_region) = region {
                memory_regions[region_count] = Some(*mem_region);
                region_count += 1;
            }
        }
        
        if region_count > 0 {
            // Use the first valid memory region found
            if let Some(first_region) = memory_regions[0] {
                // Initialize physical frame allocator with the first region
                init_frame_allocator(&[first_region]);
                
                // Get frame allocator statistics
                let (free, total) = frame_allocator::frame_allocator_stats();
                crate::println!("Memory: Physical frame allocator ready ({} free / {} total frames)", 
                               free, total);
                
                // Initialize MMU (for now, skip to avoid complexity)
                // TODO: Enable MMU initialization once we handle identity mapping properly
                // if let Err(e) = MemoryManagementUnit::init() {
                //     crate::println!("Memory: MMU initialization failed: {}", e);
                // }
                
                crate::println!("Memory: Virtual memory management ready");
            } else {
                crate::println!("Memory: Warning - Invalid memory region found");
            }
        } else {
            crate::println!("Memory: Warning - No memory regions found");
        }
    } else {
        crate::println!("Memory: Warning - Using fallback memory configuration");
        
        // Fallback: assume 1GB of RAM starting at 0x40000000
        use crate::devicetree::MemoryRegion;
        let fallback_region = MemoryRegion {
            start: 0x40000000,
            size: 1024 * 1024 * 1024,  // 1GB
        };
        init_frame_allocator(&[fallback_region]);
        
        let (free, total) = frame_allocator::frame_allocator_stats();
        crate::println!("Memory: Fallback frame allocator ready ({} free / {} total frames)", 
                       free, total);
    }
    
    // Run memory tests to verify functionality
    test::run_memory_tests();
    
    crate::println!("Memory: Memory management system initialized");
}
