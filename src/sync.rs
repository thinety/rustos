use core::cell::UnsafeCell;
use core::ops;
use core::sync::atomic::{AtomicBool, Ordering};

struct RawMutex {
    locked: AtomicBool,
}

impl RawMutex {
    #[inline]
    const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    #[inline]
    fn lock(&self) {
        loop {
            if self
                .locked
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
            loop {
                if !self.locked.load(Ordering::Relaxed) {
                    break;
                }
            }
        }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        !self.locked.load(Ordering::Relaxed)
            && self
                .locked
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

pub struct Mutex<T> {
    raw_mutex: RawMutex,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            raw_mutex: RawMutex::new(),
            data: UnsafeCell::new(data),
        }
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.raw_mutex.lock();
        MutexGuard { mutex: self }
    }

    #[inline]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.raw_mutex.try_lock() {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

impl<T> ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.mutex.raw_mutex.unlock();
        }
    }
}
