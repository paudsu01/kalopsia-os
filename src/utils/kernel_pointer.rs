use core::marker::{Send, Sync};

/// A simple wrapper for a raw mutable pointer (`*mut T`)
/// that manually asserts Send/Sync safety.
///
/// This is typically done when a Spinlock or other external
/// synchronization primitive guarantees that the access will be safe.
#[repr(transparent)]
pub struct KernelPointer<T> {
    ptr: *mut T,
}

// SAFETY: We assert that the data structure holding this pointer (example: for the Buddy Allocator)
// is wrapped in a Spinlock, ensuring that no two threads can access or modify the
// underlying memory region simultaneously via this pointer unless it's safe (e.g., read-only).
unsafe impl<T> Send for KernelPointer<T> {}
unsafe impl<T> Sync for KernelPointer<T> {}
