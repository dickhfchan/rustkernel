// ARM64 physical memory frame allocator

use core::ptr::NonNull;
use spin::Mutex;
use crate::devicetree::MemoryRegion;

// 4KB page size for ARM64
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12;

// Physical frame number type
pub type FrameNumber = usize;

// Convert physical address to frame number
pub fn addr_to_frame(addr: u64) -> FrameNumber {
    (addr as usize) >> PAGE_SHIFT
}

// Convert frame number to physical address
pub fn frame_to_addr(frame: FrameNumber) -> u64 {
    (frame << PAGE_SHIFT) as u64
}

// Bitmap-based frame allocator
pub struct FrameAllocator {
    bitmap: &'static mut [u8],
    start_frame: FrameNumber,
    total_frames: usize,
    free_frames: usize,
    next_free_hint: usize,
}

impl FrameAllocator {
    pub fn new(memory_regions: &[MemoryRegion], bitmap_storage: &'static mut [u8]) -> Self {
        // Find the largest memory region for simplicity
        let main_region = memory_regions
            .iter()
            .max_by_key(|region| region.size)
            .expect("No memory regions found");
        
        let start_frame = addr_to_frame(main_region.start);
        let total_frames = (main_region.size as usize) >> PAGE_SHIFT;
        
        // Initialize bitmap - all frames marked as used initially
        let bitmap_bytes = (total_frames + 7) / 8;
        assert!(bitmap_storage.len() >= bitmap_bytes, "Bitmap storage too small");
        
        for byte in &mut bitmap_storage[..bitmap_bytes] {
            *byte = 0xFF; // All used
        }
        
        let mut allocator = Self {
            bitmap: &mut bitmap_storage[..bitmap_bytes],
            start_frame,
            total_frames,
            free_frames: 0,
            next_free_hint: 0,
        };
        
        // Mark usable frames as free (skip kernel area)
        let kernel_end_frame = addr_to_frame(0x41000000); // Rough kernel end
        let usable_start = if kernel_end_frame > start_frame {
            kernel_end_frame - start_frame
        } else {
            0
        };
        
        for frame_idx in usable_start..total_frames {
            allocator.mark_frame_free(frame_idx);
        }
        
        crate::println!("FrameAllocator: {} frames total, {} frames free", 
                       total_frames, allocator.free_frames);
        
        allocator
    }
    
    // Allocate a single physical frame
    pub fn allocate_frame(&mut self) -> Option<FrameNumber> {
        if self.free_frames == 0 {
            return None;
        }
        
        // Start searching from hint
        for i in 0..self.total_frames {
            let frame_idx = (self.next_free_hint + i) % self.total_frames;
            if self.is_frame_free(frame_idx) {
                self.mark_frame_used(frame_idx);
                self.next_free_hint = (frame_idx + 1) % self.total_frames;
                return Some(self.start_frame + frame_idx);
            }
        }
        
        None
    }
    
    // Deallocate a physical frame
    pub fn deallocate_frame(&mut self, frame: FrameNumber) {
        if frame < self.start_frame || frame >= self.start_frame + self.total_frames {
            return; // Invalid frame
        }
        
        let frame_idx = frame - self.start_frame;
        if !self.is_frame_free(frame_idx) {
            self.mark_frame_free(frame_idx);
        }
    }
    
    // Check if frame is free
    fn is_frame_free(&self, frame_idx: usize) -> bool {
        let byte_idx = frame_idx / 8;
        let bit_idx = frame_idx % 8;
        
        if byte_idx >= self.bitmap.len() {
            return false;
        }
        
        (self.bitmap[byte_idx] & (1 << bit_idx)) == 0
    }
    
    // Mark frame as used
    fn mark_frame_used(&mut self, frame_idx: usize) {
        let byte_idx = frame_idx / 8;
        let bit_idx = frame_idx % 8;
        
        if byte_idx < self.bitmap.len() && self.is_frame_free(frame_idx) {
            self.bitmap[byte_idx] |= 1 << bit_idx;
            self.free_frames -= 1;
        }
    }
    
    // Mark frame as free
    fn mark_frame_free(&mut self, frame_idx: usize) {
        let byte_idx = frame_idx / 8;
        let bit_idx = frame_idx % 8;
        
        if byte_idx < self.bitmap.len() && !self.is_frame_free(frame_idx) {
            self.bitmap[byte_idx] &= !(1 << bit_idx);
            self.free_frames += 1;
        }
    }
    
    // Get allocation statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.free_frames, self.total_frames)
    }
}

// Global frame allocator
static FRAME_ALLOCATOR: Mutex<Option<FrameAllocator>> = Mutex::new(None);

// Static storage for bitmap (supports up to 256MB of RAM)
static mut BITMAP_STORAGE: [u8; 8192] = [0; 8192];

pub fn init_frame_allocator(memory_regions: &[MemoryRegion]) {
    let bitmap_storage = unsafe { &mut BITMAP_STORAGE };
    let allocator = FrameAllocator::new(memory_regions, bitmap_storage);
    *FRAME_ALLOCATOR.lock() = Some(allocator);
}

pub fn allocate_frame() -> Option<NonNull<u8>> {
    let mut allocator_guard = FRAME_ALLOCATOR.lock();
    if let Some(allocator) = allocator_guard.as_mut() {
        if let Some(frame) = allocator.allocate_frame() {
            let addr = frame_to_addr(frame);
            return NonNull::new(addr as *mut u8);
        }
    }
    None
}

pub fn deallocate_frame(frame_addr: NonNull<u8>) {
    let addr = frame_addr.as_ptr() as u64;
    let frame = addr_to_frame(addr);
    
    let mut allocator_guard = FRAME_ALLOCATOR.lock();
    if let Some(allocator) = allocator_guard.as_mut() {
        allocator.deallocate_frame(frame);
    }
}

pub fn frame_allocator_stats() -> (usize, usize) {
    let allocator_guard = FRAME_ALLOCATOR.lock();
    if let Some(allocator) = allocator_guard.as_ref() {
        allocator.stats()
    } else {
        (0, 0)
    }
}