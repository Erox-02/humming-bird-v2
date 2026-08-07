use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::sync::Once;

static INIT: Once = Once::new();

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let target = record.target();
            let args = record.args();
            
            eprintln!("{} - {} - {} - {}", 
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                level,
                target,
                args
            );
        }
    }
    
    fn flush(&self) {}
}

pub fn setup_logger() -> Result<(), SetLoggerError> {
    let mut result = Ok(());
    INIT.call_once(|| {
        result = log::set_boxed_logger(Box::new(SimpleLogger))
            .map(|()| log::set_max_level(LevelFilter::Info));
    });
    result.map_err(|e| e)
}

pub fn get_logger() -> &'static str {
    "hbp100"
}
