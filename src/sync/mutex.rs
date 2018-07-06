use core::cell::UnsafeCell;
use core::default::Default;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use psp2_sys::kernel::threadmgr::sceKernelCreateMutex;
use psp2_sys::kernel::threadmgr::sceKernelDelayThread;
use psp2_sys::kernel::threadmgr::sceKernelDeleteMutex;
use psp2_sys::kernel::threadmgr::sceKernelLockMutex;
use psp2_sys::kernel::threadmgr::sceKernelTryLockMutex;
use psp2_sys::kernel::threadmgr::sceKernelUnlockMutex;
use psp2_sys::types::SceUID;

mod utils {
    /// Write a number as an hexadecimal formatter bytestring
    ///
    /// Panics if the buffer is shorter than `size_of::<usize>()`.
    pub fn write_hex(number: usize, buf: &mut [u8]) {
        let length = ::core::mem::size_of::<usize>() / 4;
        for i in 0..length {
            buf[buf.len() - (i + 2)] = match (number & 0xF) as u8 {
                x @ 0x0u8...0x9u8 => x as u8 + b'0',
                y @ 0xAu8...0xFu8 => y as u8 + b'A',
                _ => unreachable!(),
            };
        }
    }
}

/// A mutual exclusiong primitive which uses the PS Vita kernel API.
pub struct Mutex<T: ?Sized> {
    lock: SceUID,        // the kernel lock
    init: AtomicBool,    // false until the lock initialization has started
    done: AtomicBool,    // false until the lock initialization has finished
    data: UnsafeCell<T>, // the synced data
}

/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: SceUID,
    data: &'a UnsafeCell<T>,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new lock wrapping the supplied data.
    pub const fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: 0,
            init: AtomicBool::new(false),
            done: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock the inner mutex.
        //
        // To get the inner value, we'd like to call `data.into_inner()`,
        // but because `Mutex` impl-s `Drop`, we can't move out of it, so
        // we'll have to destructure it manually instead.
        let data = unsafe {
            let Mutex { ref data, .. } = self;
            ::core::ptr::read(data)
        };

        data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Check the lock was properly initializated and return it.
    fn init_lock(&self) -> SceUID {
        if self.init.compare_and_swap(false, true, Ordering::Acquire) == false {
            // if the lock initialization was not started: start it !
            // - name the mutex according to its memory location
            let ptr: usize = self as *const Self as *const usize as usize;
            let mut name = *b"__rust_mutex_0x00000000\0";
            utils::write_hex(ptr, &mut name[15..23]);
            // - initialize the mutex
            unsafe { sceKernelCreateMutex(&name as *const u8, 0, 0, ::core::ptr::null_mut()) };
            self.done.store(true, Ordering::Relaxed);
        } else {
            // wait for the lock initialization to be over.
            while !self.done.load(Ordering::Relaxed) {
                unsafe { sceKernelDelayThread(1000) };
            }
        }
        self.lock
    }

    /// Locks the mutex and returns a guard.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    ///
    /// ```rust,ignore
    /// let mylock = vita::sync::Mutex::new(0);
    /// {
    ///     let mut data = mylock.lock();
    ///     // The lock is now locked and the data can be accessed
    ///     *data += 1;
    ///     // The lock is implicitly dropped
    /// }
    ///
    /// ```
    pub fn lock(&self) -> MutexGuard<T> {
        let lock = self.init_lock();
        unsafe { sceKernelLockMutex(lock, 1, ::core::ptr::null_mut()) };
        MutexGuard {
            lock: lock,
            data: &self.data,
        }
    }

    /// Tries to lock the mutex.
    ///
    /// If it is already locked, it will return None. Otherwise it returns
    /// a guard within Some.
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let lock = self.init_lock();
        if unsafe { sceKernelTryLockMutex(lock, 1) } >= 0 {
            Some(MutexGuard {
                lock: lock,
                data: &self.data,
            })
        } else {
            None
        }
    }
}

impl<T: ?Sized + Debug> Debug for Mutex<T> {
    fn fmt(&self, f: &mut Formatter) -> ::core::fmt::Result {
        match self.try_lock() {
            Some(guard) => write!(f, "Mutex {{ data: {:?} }}", &*guard),
            None => write!(f, "Mutex {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default> Default for Mutex<T> {
    fn default() -> Mutex<T> {
        Mutex::new(Default::default())
    }
}

impl<'a, T: ?Sized> ::core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        unsafe { &(*self.data.get()) }
    }
}

impl<'a, T: ?Sized> ::core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        unsafe { &mut (*self.data.get()) }
    }
}

impl<'a, T: ?Sized> ::core::ops::Drop for MutexGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        unsafe { sceKernelUnlockMutex(self.lock, 1) };
    }
}

impl<T: ?Sized> ::core::ops::Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe { sceKernelDeleteMutex(self.lock) };
    }
}
