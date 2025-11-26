use spin::{mutex::Mutex, MutexGuard};

/// We want interior mutability for our static variable since the `GlobalAlloc` trait only takes an
/// immutable reference. So, for the `ALLOCATOR` static variable, we wrap it around a `Mutex`
/// However, because we cannot implement external traits on external types, we create a internal
/// type wrapper over `Mutex`
#[allow(dead_code)]
pub struct MutexWrapper<T> {
    mutex: Mutex<T>,
}

#[allow(dead_code)]
impl<T> MutexWrapper<T> {
    pub const fn new(val: T) -> Self {
        MutexWrapper {
            mutex: Mutex::new(val),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.mutex.lock()
    }
}
