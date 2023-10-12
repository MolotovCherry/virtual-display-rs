#[macro_export]
macro_rules! debug {
    ($($tt:tt)*) => {
        if cfg!(debug_assertions) {
            ::log::debug!($($tt)*);
        }
    };
}

pub use debug;
