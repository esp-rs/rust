use crate::cell::UnsafeCell;
use crate::mem::MaybeUninit;
use crate::sys::cvt_nz;

// The newlib definitions in libc for pthread_mutex_t and PTHREAD_MUTEX_INITIALIZER
// are way off compared to what we have in ESP-IDF:
// 1) ESP-IDF's pthread_mutex_t is only 4 bytes and not 40+ (because it is in fact a pointer to dynamically allocated structure)
// 2) While the above is just a waste of space, ESP-IDF PTHREAD_MUTEX_INITIALIZER is the real issue:
//    in libc it is defined as a series of 0s, while in ES-IDF it is equal to 0xffffffff
#[cfg(target_os = "espidf")]
pub type pthread_mutex_t = u32;
#[cfg(target_os = "espidf")]
pub type pthread_mutexattr_t = u64;
#[cfg(target_os = "espidf")]
const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = 0xffffffff;

#[cfg(not(target_os = "espidf"))]
pub type pthread_mutex_t = libc::pthread_mutex_t;
#[cfg(not(target_os = "espidf"))]
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
#[cfg(not(target_os = "espidf"))]
const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;

pub struct Mutex {
    inner: UnsafeCell<pthread_mutex_t>,
}

pub type MovableMutex = Box<Mutex>;

#[inline]
pub unsafe fn raw(m: &Mutex) -> *mut pthread_mutex_t {
    m.inner.get()
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

#[allow(dead_code)] // sys isn't exported yet
impl Mutex {
    pub const fn new() -> Mutex {
        // Might be moved to a different address, so it is better to avoid
        // initialization of potentially opaque OS data before it landed.
        // Be very careful using this newly constructed `Mutex`, reentrant
        // locking is undefined behavior until `init` is called!
        Mutex { inner: UnsafeCell::new(PTHREAD_MUTEX_INITIALIZER) }
    }
    #[inline]
    pub unsafe fn init(&mut self) {
        // Issue #33770
        //
        // A pthread mutex initialized with PTHREAD_MUTEX_INITIALIZER will have
        // a type of PTHREAD_MUTEX_DEFAULT, which has undefined behavior if you
        // try to re-lock it from the same thread when you already hold a lock
        // (https://pubs.opengroup.org/onlinepubs/9699919799/functions/pthread_mutex_init.html).
        // This is the case even if PTHREAD_MUTEX_DEFAULT == PTHREAD_MUTEX_NORMAL
        // (https://github.com/rust-lang/rust/issues/33770#issuecomment-220847521) -- in that
        // case, `pthread_mutexattr_settype(PTHREAD_MUTEX_DEFAULT)` will of course be the same
        // as setting it to `PTHREAD_MUTEX_NORMAL`, but not setting any mode will result in
        // a Mutex where re-locking is UB.
        //
        // In practice, glibc takes advantage of this undefined behavior to
        // implement hardware lock elision, which uses hardware transactional
        // memory to avoid acquiring the lock. While a transaction is in
        // progress, the lock appears to be unlocked. This isn't a problem for
        // other threads since the transactional memory will abort if a conflict
        // is detected, however no abort is generated when re-locking from the
        // same thread.
        //
        // Since locking the same mutex twice will result in two aliasing &mut
        // references, we instead create the mutex with type
        // PTHREAD_MUTEX_NORMAL which is guaranteed to deadlock if we try to
        // re-lock it from the same thread, thus avoiding undefined behavior.
        let mut attr = MaybeUninit::<pthread_mutexattr_t>::uninit();
        cvt_nz(libc::pthread_mutexattr_init(attr.as_mut_ptr() as *mut _)).unwrap();
        let attr = PthreadMutexAttr(&mut attr);
        cvt_nz(libc::pthread_mutexattr_settype(
            attr.0.as_mut_ptr() as *mut _,
            libc::PTHREAD_MUTEX_NORMAL,
        ))
        .unwrap();
        cvt_nz(libc::pthread_mutex_init(self.inner.get() as *mut _, attr.0.as_ptr() as *mut _))
            .unwrap();
    }
    #[inline]
    pub unsafe fn lock(&self) {
        let r = libc::pthread_mutex_lock(self.inner.get() as *mut _);
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn unlock(&self) {
        let r = libc::pthread_mutex_unlock(self.inner.get() as *mut _);
        debug_assert_eq!(r, 0);
    }
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        libc::pthread_mutex_trylock(self.inner.get() as *mut _) == 0
    }
    #[inline]
    #[cfg(not(target_os = "dragonfly"))]
    pub unsafe fn destroy(&self) {
        let r = libc::pthread_mutex_destroy(self.inner.get() as *mut _);
        debug_assert_eq!(r, 0);
    }
    #[inline]
    #[cfg(target_os = "dragonfly")]
    pub unsafe fn destroy(&self) {
        let r = libc::pthread_mutex_destroy(self.inner.get() as *mut _);
        // On DragonFly pthread_mutex_destroy() returns EINVAL if called on a
        // mutex that was just initialized with libc::PTHREAD_MUTEX_INITIALIZER.
        // Once it is used (locked/unlocked) or pthread_mutex_init() is called,
        // this behaviour no longer occurs.
        debug_assert!(r == 0 || r == libc::EINVAL);
    }
}

pub struct ReentrantMutex {
    inner: UnsafeCell<pthread_mutex_t>,
}

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
    pub const unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex { inner: UnsafeCell::new(PTHREAD_MUTEX_INITIALIZER) }
    }

    pub unsafe fn init(&self) {
        let mut attr = MaybeUninit::<pthread_mutexattr_t>::uninit();
        cvt_nz(libc::pthread_mutexattr_init(attr.as_mut_ptr() as *mut _)).unwrap();
        let attr = PthreadMutexAttr(&mut attr);
        cvt_nz(libc::pthread_mutexattr_settype(
            attr.0.as_mut_ptr() as *mut _,
            libc::PTHREAD_MUTEX_RECURSIVE,
        ))
        .unwrap();
        cvt_nz(libc::pthread_mutex_init(self.inner.get() as *mut _, attr.0.as_ptr() as *mut _))
            .unwrap();
    }

    pub unsafe fn lock(&self) {
        let result = libc::pthread_mutex_lock(self.inner.get() as *mut _);
        debug_assert_eq!(result, 0);
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        libc::pthread_mutex_trylock(self.inner.get() as *mut _) == 0
    }

    pub unsafe fn unlock(&self) {
        let result = libc::pthread_mutex_unlock(self.inner.get() as *mut _);
        debug_assert_eq!(result, 0);
    }

    pub unsafe fn destroy(&self) {
        let result = libc::pthread_mutex_destroy(self.inner.get() as *mut _);
        debug_assert_eq!(result, 0);
    }
}

struct PthreadMutexAttr<'a>(&'a mut MaybeUninit<pthread_mutexattr_t>);

impl Drop for PthreadMutexAttr<'_> {
    fn drop(&mut self) {
        unsafe {
            let result = libc::pthread_mutexattr_destroy(self.0.as_mut_ptr() as *mut _);
            debug_assert_eq!(result, 0);
        }
    }
}
