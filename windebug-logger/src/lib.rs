use std::fmt::Write;

use log::{Level, Log, SetLoggerError};
use windows::{core::PCWSTR, Win32::System::Diagnostics::Debug::OutputDebugStringW};

#[derive(Debug)]
pub struct WinDebugLogger {
    pub level: Level,
}

impl Log for WinDebugLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // Silently ignore errors
        let _ = log(record);
    }

    fn flush(&self) {}
}

fn log(record: &log::Record) -> Option<()> {
    let target = if record.target().len() > 0 {
        record.target()
    } else {
        record.module_path().unwrap_or_default()
    };

    let time = chrono::Local::now();
    let formatted = time.format("%m/%d %I:%M%P");

    let level = format!("[{}]", record.level());

    // Everything except the timestamp
    let mut base = String::new();
    write!(&mut base, "{formatted} {level:<7} [{target}").ok()?;

    if let Some(line) = record.line() {
        write!(&mut base, ":{line}").ok()?;
    }

    write!(&mut base, "]").ok()?;

    for line in record.args().to_string().lines() {
        let full = format!("{base} {line}\0");
        let full = full.encode_utf16().collect::<Vec<u16>>();

        // Write the output
        unsafe {
            OutputDebugStringW(PCWSTR(full.as_ptr()));
        }
    }

    Some(())
}

/// Initialize the global logger with a specific log level.
///
/// ```
/// # use log::{warn, info};
/// # fn main() {
/// windebug_logger::init_with_level(log::Level::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
/// ```
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = WinDebugLogger { level };
    match log::set_boxed_logger(Box::new(logger)) {
        Ok(()) => {
            log::set_max_level(level.to_level_filter());
            Ok(())
        }
        Err(e) => Err(e),
    }
}
