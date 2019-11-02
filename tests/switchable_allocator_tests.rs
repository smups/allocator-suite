#![feature(allocator_api)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]
#![feature(thread_local)]
#![feature(const_fn)]

#[cfg(test)]
mod switchable_allocator_tests {
    // Allocator generator macro
    use allocator_suite::switchable_allocator;

    // General imports
    use std::alloc::System;
    use allocator_suite::adaptors::prelude::*;

    switchable_allocator!(
        ApplicationAllocator,
        BumpAllocator<ArenaMemorySource<MemoryMapSource>>,
        MultipleBinarySearchTreeAllocator<MemoryMapSource>,
        GlobalAllocToAllocatorAdaptor<System>,
        GlobalAllocToAllocatorAdaptor(System)
    );

    #[test]
    pub fn switchable_generation() {
        use crate::switchable_allocator_tests::GLOBAL;

        let _vec = Vec::<usize>::with_capacity(1234);
    }
}