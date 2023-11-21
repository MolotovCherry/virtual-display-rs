#![allow(clippy::missing_errors_doc)]

mod win_debug;
mod win_logger;

use std::error::Error;

use log::{Level, Log};

use crate::win_debug::WinDebugLogger;
use crate::win_logger::WinLogger;

// A logger which logs to multiple logger implementations
pub struct DriverLogger {
    pub level: Level,
    win_debug: Option<WinDebugLogger>,
    win_logger: Option<WinLogger>,
}

impl DriverLogger {
    #[must_use]
    pub fn new(level: Level) -> Self {
        Self {
            level,
            win_logger: None,
            win_debug: None,
        }
    }

    pub fn debug(&mut self) -> &mut Self {
        self.win_debug = Some(WinDebugLogger { level: self.level });
        self
    }

    pub fn name(&mut self, name: &str) -> Result<&mut Self, Box<dyn Error>> {
        self.win_logger = Some(WinLogger::new(name)?);
        Ok(self)
    }

    pub fn init(self) -> Result<(), Box<dyn Error>> {
        let level = self.level;

        match log::set_boxed_logger(Box::new(self)) {
            Ok(()) => {
                log::set_max_level(level.to_level_filter());
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
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

        if let Some(debug) = self.win_debug.as_ref() {
            debug.log(record);
        }

        if let Some(logger) = self.win_logger.as_ref() {
            logger.log(record);
        }
    }

    fn flush(&self) {
        if let Some(debug) = self.win_debug.as_ref() {
            debug.flush();
        }

        if let Some(logger) = self.win_logger.as_ref() {
            logger.flush();
        }
    }
}
