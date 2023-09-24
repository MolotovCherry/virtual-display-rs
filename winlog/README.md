# winlog2

This is a fork of `winlog`.

A simple [Rust log](https://docs.rs/log/latest/log/) backend to send messages to the [Windows event log](https://docs.microsoft.com/en-us/windows/desktop/eventlog/event-logging).

## Features

* Writes Rust log messages to the Windows event log using the
  [RegisterEventSourceW](https://docs.microsoft.com/en-us/windows/desktop/api/Winbase/nf-winbase-registereventsourcew)
  and [ReportEventW](https://docs.microsoft.com/en-us/windows/desktop/api/winbase/nf-winbase-reporteventw) APIs.
* Supports `env_logger` filtering, initialized from RUST_LOG environment variable. (optional)
* Provides utility functions to register/unregister your
  [event source](https://docs.microsoft.com/en-us/windows/desktop/eventlog/event-sources) in the Windows registry.
* Embeds a small (120-byte) message resource library containing the
  necessary log message templates in your executable.

The five Rust log levels are mapped to Windows [event types](https://docs.microsoft.com/en-us/windows/desktop/eventlog/event-types) as follows:

| Rust Log Level | Windows Event Type | Windows Event Id |
| -------------- | ------------------ | ---------------- |
| Error          | Error              | 1                |
| Warn           | Warning            | 2                |
| Info           | Informational      | 3                |
| Debug          | Informational      | 4                |
| Trace          | Informational      | 5                |


## Requirements

* Windows or MinGW
* [Windows, optional] PowerShell (used for the end-to-end test)

## Usage

### Cargo.toml

Plain winlog:
```
[dependencies]
log = "*"
winlog = "*"
```
Or to enable env_logger filtering support:
```
[dependencies]
log = "*"
winlog = { version = "0.2.5", features = ["env_logger"] }
```

### Register log source with Windows

Register the log source in the Windows registry:
```
winlog::register("Example Log").unwrap();
```
This usually requires `Administrator` permission so this is usually done during
installation time.

If your MSI installer (or similar) registers your event sources you should not call this.


### Log events

Without env_logger filtering:
```
use log::{info, trace};

winlog::init("Example Log").unwrap();

info!("Hello, Event Log");
trace!("This will be logged too");
```

Use the winlog backend with env_logger filter enabled:
```
use log::{info, trace};

// # export RUST_LOG="info"
winlog::init("Example Log").unwrap();
info!("Hello, Event Log");
trace!("This will be filtered out");
```

### Deregister log source

Deregister the log source: 
```
winlog::deregister("Example Log").unwrap();
```
This is usually done during program uninstall. If your MSI 
installer (or similar) deregisters your event sources you should not call this.

## What's New

### 0.3.0
* Fork from original repo.
* Use `windows-sys` instead of `winapi`.
* Update other dependencies.
* Generate `eventmsgs.rc` and compile it with `winres`.
* Fix `end-to-end` test to deregister correctly even if it fails.
* Remove APIs that silently fails.

### 0.2.6

* Disable unneeded regex features to speed up the build.
* Improve error reporting/handling in `build.rs`.

### 0.2.5

* Gitlab CI builds on Windows 10 and Debian/MinGW.
* Optional support for env_logger event (enable feature `env_logger`).
* Always run `windrc/windrc` on MinGW.
* Include linker configuration in `.cargo/config`. 

## Building

### Windows

```sh
cargo build --release
```

### MinGW

Install MinGW (Ubuntu):

```sh
sudo apt install mingw-w64
```

Install Rust:

```sh
rustup target install x86_64-pc-windows-gnu
```

Currently the install from rustup doesn't use the correct linker so you have to add the following to `.cargo/config`:

    [target.x86_64-pc-windows-gnu]
    linker = "/usr/bin/x86_64-w64-mingw32-gcc"

Build:
```sh
cargo build --release
```

### Internals

Artifacts `eventmsgs.rc` and `MSG00409.bin` are under source control so users 
don't need to have `mc.exe` installed for a standard build.

## Testing

The end-to-end test requires 'Full Control' permissions on the 
`HKLM\SYSTEM\CurrentControlSet\Services\EventLog\Application`
registry key.

```cargo test```

Process:
1. Create a unique temporary event source name (`winlog-test-###########`).
2. Register our compiled test executable as ```EventMessageFile``` for 
   the event source in the Windows registry. You can see a new key at 
   `HKLM\SYSTEM\CurrentControlSet\Services\EventLog\Application\winlog-test-###########`.
2. Write some log messages to the event source.
3. Use PowerShell to retrieve the logged messages.
4. Assert that the retrieved log messages are correct. 
5. Deregister our event source. This removes the `winlog-test-###########` 
   registry key.


## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in the work by you, as defined in the Apache-2.0 license, shall 
be dual licensed as above, without any additional terms or conditions.
