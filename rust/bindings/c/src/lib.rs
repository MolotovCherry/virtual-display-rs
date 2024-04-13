use std::ffi::{c_char, CStr};

use driver_ipc::{Dimen, Id, Mode as RMode, RefreshRate};

mod client;
mod driver_client;
mod utils;

/// Cannot be freed if allocated/created on c side and passed to rust
/// If you received this type from a fn call, then it must be freed
#[repr(C)]
#[derive(Debug, Clone)]
struct Monitor {
    /// identifier
    id: Id,
    /// null if there's no name. non null if there is. must be null terminated
    name: *const c_char,
    /// length of name array
    name_len: usize,
    enabled: bool,
    /// array of modes. cannot be null. but len may be 0
    modes: *const Mode,
    /// length of modes array
    modes_len: usize,
}

/// Cannot be freed if allocated/created on c side and passed to rust
/// If you received this type from a fn call, then it must be freed
#[repr(C)]
#[derive(Debug, Clone)]
struct Mode {
    width: Dimen,
    height: Dimen,
    /// array of refresh rates. cannot be null, but len may be 0
    refresh_rates: *const RefreshRate,
    /// length of refresh_rates array
    refresh_rates_len: usize,
}

/// You must call free on it when done
#[repr(C)]
#[derive(Debug, Clone)]
enum ReplyCommand {
    /// Reply to previous current system monitor state request
    /// ptr to array of monitor + len of array
    /// cbindgen:field-names=[arr, len, _reserved]
    // ptr, len, cap
    State(*mut Monitor, usize, usize),
}

impl TryFrom<&Monitor> for driver_ipc::Monitor {
    type Error = ();

    fn try_from(value: &Monitor) -> Result<driver_ipc::Monitor, ()> {
        let name = if value.name.is_null() {
            None
        } else {
            let name = unsafe { CStr::from_ptr(value.name) };
            let Ok(name) = name.to_str() else {
                return Err(());
            };

            Some(name.to_owned())
        };

        let modes = if value.modes.is_null() {
            Vec::new()
        } else {
            let modes = if value.modes.is_null() {
                &[]
            } else {
                unsafe { std::slice::from_raw_parts(value.modes, value.modes_len) }
            };

            let mut r_modes = Vec::with_capacity(modes.len());
            for mode in modes {
                let refresh_rates = if mode.refresh_rates.is_null() {
                    &[]
                } else {
                    unsafe {
                        std::slice::from_raw_parts(mode.refresh_rates, mode.refresh_rates_len)
                    }
                };

                r_modes.push(RMode {
                    width: mode.width,
                    height: mode.height,
                    refresh_rates: refresh_rates.to_owned(),
                });
            }

            r_modes
        };

        let monitor = driver_ipc::Monitor {
            id: value.id,
            name,
            enabled: value.enabled,
            modes,
        };

        Ok(monitor)
    }
}

impl TryFrom<ReplyCommand> for driver_ipc::ReplyCommand {
    type Error = ();

    fn try_from(value: ReplyCommand) -> Result<Self, Self::Error> {
        let val = match value {
            ReplyCommand::State(data, len, cap) => {
                let slice = unsafe { Vec::from_raw_parts(data, len, cap) };
                let slice = slice
                    .into_iter()
                    .map(|i| (&i).try_into())
                    .collect::<Result<Vec<driver_ipc::Monitor>, _>>()?;

                driver_ipc::ReplyCommand::State(slice)
            }
        };

        Ok(val)
    }
}

impl From<driver_ipc::ReplyCommand> for ReplyCommand {
    fn from(value: driver_ipc::ReplyCommand) -> Self {
        match value {
            driver_ipc::ReplyCommand::State(v) => {
                let v = v
                    .into_iter()
                    .map(|m| ().try_into())
                    .collect::<Result<Vec<Monitor>, _>>();

                ReplyCommand::State(ptr.as_mut_ptr(), len, cap)
            }

            _ => unreachable!(),
        }
    }
}
