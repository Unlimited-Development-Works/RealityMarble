extern crate log;

use log::{ LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError };

pub struct PrintLogger {
    max_level: LogLevel
}

impl PrintLogger {
    fn new() -> PrintLogger {
        PrintLogger {
            max_level: LogLevel::Info
        }
    }

    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(PrintLogger::new())
    })
}
}

impl log::Log for PrintLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}