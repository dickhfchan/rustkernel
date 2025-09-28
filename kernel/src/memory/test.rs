// Memory management testing utilities

use crate::memory::frame_allocator::{allocate_frame, deallocate_frame, frame_allocator_stats};

pub fn test_frame_allocation() {
    crate::println!("Memory Test: Testing frame allocation...");
    
    let (free_before, total) = frame_allocator_stats();
    crate::println!("Memory Test: Initial stats - {} free / {} total", free_before, total);
    
    // Allocate some frames
    let mut allocated_frames = [None; 10];
    let mut allocated_count = 0;
    
    for i in 0..10 {
        if let Some(frame) = allocate_frame() {
            allocated_frames[i] = Some(frame);
            allocated_count += 1;
        } else {
            crate::println!("Memory Test: Failed to allocate frame {}", i);
            break;
        }
    }
    
    let (free_after_alloc, _) = frame_allocator_stats();
    crate::println!("Memory Test: After allocating {} frames - {} free", 
                   allocated_count, free_after_alloc);
    
    // Verify allocation worked
    if free_before - free_after_alloc == allocated_count {
        crate::println!("Memory Test: ✓ Frame allocation working correctly");
    } else {
        crate::println!("Memory Test: ✗ Frame allocation counters incorrect");
    }
    
    // Deallocate frames
    for frame in allocated_frames.iter().filter_map(|f| *f) {
        deallocate_frame(frame);
    }
    
    let (free_after_dealloc, _) = frame_allocator_stats();
    crate::println!("Memory Test: After deallocation - {} free", free_after_dealloc);
    
    // Verify deallocation worked
    if free_after_dealloc == free_before {
        crate::println!("Memory Test: ✓ Frame deallocation working correctly");
    } else {
        crate::println!("Memory Test: ✗ Frame deallocation counters incorrect");
    }
    
    crate::println!("Memory Test: Frame allocation test completed");
}

pub fn test_heap_allocation() {
    crate::println!("Memory Test: Testing heap allocation...");
    
    // Test basic heap allocation using Vec
    use alloc::vec::Vec;
    
    let mut test_vec = Vec::new();
    for i in 0..100 {
        test_vec.push(i);
    }
    
    // Verify the vector contents
    let mut all_correct = true;
    for (index, &value) in test_vec.iter().enumerate() {
        if index != value {
            all_correct = false;
            break;
        }
    }
    
    if all_correct && test_vec.len() == 100 {
        crate::println!("Memory Test: ✓ Heap allocation working correctly");
    } else {
        crate::println!("Memory Test: ✗ Heap allocation failed");
    }
    
    drop(test_vec);
    crate::println!("Memory Test: Heap allocation test completed");
}

pub fn run_memory_tests() {
    crate::println!("Memory Test: Starting memory management tests...");
    test_heap_allocation();
    test_frame_allocation();
    crate::println!("Memory Test: All memory tests completed");
}