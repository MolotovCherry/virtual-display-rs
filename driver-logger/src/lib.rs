mod win_debug;
mod win_logger;

use std::error::Error;

use log::{Level, Log};

use crate::{win_debug::WinDebugLogger, win_logger::WinLogger};

// A logger which logs to multiple logger implementations
pub struct DriverLogger {
    pub level: Level,
    loggers: Vec<Box<dyn Log>>,
}

impl DriverLogger {
    #[allow(clippy::missing_errors_doc)]
    pub fn new(name: &str, level: Level) -> Result<Self, Box<dyn Error>> {
        let loggers: Vec<Box<dyn Log>> = vec![
            Box::new(WinDebugLogger { level }),
            Box::new(WinLogger::new(name)?),
        ];

        Ok(Self { level, loggers })
    }
}

impl Log for DriverLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        for logger in &self.loggers {
            logger.log(record);
        }
    }

    fn flush(&self) {
        for logger in &self.loggers {
            logger.flush();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn init_with_level(name: &str, level: Level) -> Result<(), Box<dyn Error>> {
    let logger = DriverLogger::new(name, level)?;
    match log::set_boxed_logger(Box::new(logger)) {
        Ok(()) => {
            log::set_max_level(level.to_level_filter());
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
