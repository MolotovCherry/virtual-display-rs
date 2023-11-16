/// An unsafe wrapper to allow sending across threads
///
/// USE WISELY, IT CAN CAUSE UB OTHERWISE
pub struct Sendable<T>(T);
unsafe impl<T> Send for Sendable<T> {}
unsafe impl<T> Sync for Sendable<T> {}

impl<T> Sendable<T> {
    /// `T` must be Send+Sync safe
    pub unsafe fn new(t: T) -> Self {
        Sendable(t)
    }
}

impl<T> Deref for Sendable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Sendable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_export]
macro_rules! debug {
    ($($tt:tt)*) => {
        if cfg!(debug_assertions) {
            ::log::debug!($($tt)*);
        }
    };
}

use std::ops::{Deref, DerefMut};
