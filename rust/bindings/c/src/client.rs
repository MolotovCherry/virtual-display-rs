use std::{
    ffi::{c_char, CStr},
    sync::Mutex,
};

pub use driver_ipc::sync::Client;
use driver_ipc::Id;

use crate::{utils::LazyLock, Monitor, ReplyCommand};

static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - must not use ptr after it is freed
/// - must have been a ptr given to you from this library
#[no_mangle]
unsafe extern "C" fn client_free(ptr: *mut Client) {
    unsafe {
        _ = Box::from_raw(ptr);
    }
}

/// create client
/// connect to pipe virtualdisplaydriver
///
/// returns null ptr if connection failed
#[no_mangle]
extern "C" fn client_connect() -> *mut Client {
    Client::connect()
        .map(|c| Box::into_raw(Box::new(c)))
        .unwrap_or(std::ptr::null_mut())
}

/// choose which pipe name you connect to
/// pipe name is ONLY the name, only the {name} portion of \\.\pipe\{name}
///
/// # SAFETY
/// - name arg must be null terminated
/// - must be valid char data
/// - must contain valid utf8 (won't be ub, but function will fail)
///
/// returns null ptr if function failed
#[no_mangle]
unsafe extern "C" fn client_connect_to(name: *const c_char) -> *mut Client {
    let name = unsafe { CStr::from_ptr(name) };
    let Ok(name) = name.to_str() else {
        return std::ptr::null_mut();
    };

    Client::connect_to(name)
        .map(|c| Box::into_raw(Box::new(c)))
        .unwrap_or(std::ptr::null_mut())
}

/// Notifies driver of changes (additions/updates/removals)
///
/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - monitors is a ptr to an valid array of Monitor
/// - len must be a valid len for the array
/// - this is thread safe, but will fail if any functions are called simultaneously
///
/// returns if function succeeded or not
#[no_mangle]
unsafe extern "C" fn client_notify(ptr: *mut Client, monitors: *const Monitor, len: usize) -> bool {
    let Ok(_lock) = LOCK.try_lock() else {
        return false;
    };

    let monitors = unsafe { std::slice::from_raw_parts(monitors, len) };
    let Ok(monitors) = monitors
        .iter()
        .map(std::convert::TryInto::try_into)
        .collect::<Result<Vec<driver_ipc::Monitor>, _>>()
    else {
        return false;
    };

    let client = unsafe { &mut *ptr };
    client.notify(&monitors).is_ok()
}

/// Remove specific monitors by id
///
/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - `ids` is an array of Id
/// - `ids_len` must be valid len for the array
/// - this is thread safe, but will fail if any functions are called simultaneously
#[no_mangle]
pub unsafe extern "C" fn client_remove(ptr: *mut Client, ids: *const Id, ids_len: usize) -> bool {
    let Ok(_lock) = LOCK.try_lock() else {
        return false;
    };

    let client = unsafe { &mut *ptr };
    let ids = unsafe { std::slice::from_raw_parts(ids, ids_len) };

    client.remove(ids).is_ok()
}

/// Remove all monitors
///
/// # SAFETY:
/// - ptr must be a valid, unfreed, Client
/// - this is thread safe, but will fail if any functions are called simultaneously
#[no_mangle]
pub unsafe extern "C" fn remove_all(ptr: *mut Client) -> bool {
    let Ok(_lock) = LOCK.try_lock() else {
        return false;
    };

    let client = unsafe { &mut *ptr };
    client.remove_all().is_ok()
}

/// Receive generic reply
///
/// If `last` is false, will only receive new messages from the point of calling
/// If `last` is true, will receive the the last message received, or if none, blocks until the next one
///
/// The reason for the `last` flag is that replies are auto buffered in the background, so if you send a
/// request, the reply may be missed
///
/// # SAFETY
/// - ptr must be a valid, unfreed, Client
/// - returns null ptr if function failed
/// - this is thread safe, but will fail if any functions are called simultaneously
#[no_mangle]
pub unsafe extern "C" fn receive_reply(ptr: *mut Client, last: bool) -> *mut ReplyCommand {
    let Ok(_lock) = LOCK.try_lock() else {
        return std::ptr::null_mut();
    };

    let client = unsafe { &mut *ptr };
    // let reply = client.receive_reply(last).map(|r| {
    //     let RReplyCommand::State(data) = r else {
    //         return None;
    //     };

    //     let leak = data.leak();

    //     //Some(ReplyCommand::State(leak.as_ptr(), leak.len()))
    // });

    todo!()
}

// /// Receive an event. Only new events after calling this are received
// pub fn receive_event(&mut self) -> EventCommand {
//     RUNTIME.block_on(self.0.receive_event())
// }

// /// Request state update
// /// use `receive()` to get the reply
// pub fn request_state(&self) -> Result<()> {
//     RUNTIME.block_on(self.0.request_state())
// }

// /// Persist changes to registry for current user
// pub fn persist(monitors: &[Monitor]) -> Result<()> {
//     AsyncClient::persist(monitors)
// }
