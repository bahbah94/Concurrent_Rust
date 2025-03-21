use std::ops::{Deref,DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Release,Acquire};
use std::cell::UnsafeCell;

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T>{
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {&*self.lock.value.get()}
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *self.lock.value.get()}
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}
pub struct SpinLock<T>{
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send{}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { 
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true,Acquire){
            std::hint::spin_loop();
        }
        Guard {lock : self}
    }

}

