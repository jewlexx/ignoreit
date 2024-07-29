use std::{
    ops::{Deref, DerefMut},
    sync::OnceLock,
};

#[derive(Debug, Clone)]
pub struct UnsafeOnce<T>(OnceLock<T>);

impl<T> UnsafeOnce<T> {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }

    pub unsafe fn get_unchecked(&self) -> &T {
        self.0.get().unwrap_unchecked()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.0.get_mut()
    }

    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        self.0.get_mut().unwrap_unchecked()
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        self.0.set(value)
    }
}

impl<T> Default for UnsafeOnce<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for UnsafeOnce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.get_unchecked() }
    }
}

impl<T> DerefMut for UnsafeOnce<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.get_mut_unchecked() }
    }
}
