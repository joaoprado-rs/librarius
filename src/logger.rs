use chrono::{DateTime, Local, Offset};
use std::io::Write;
use std::sync::OnceLock;

use std::{fmt::Arguments, fs::OpenOptions};

use crate::Level;

#[derive(Debug)]
pub struct Logger {
    pub level: Level,
    pub log_file: bool,
    pub file_path: Option<String>,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    pub fn init(logger: Logger) {
        LOGGER.set(logger).expect("Logger already initialized");
    }

    pub fn get() -> &'static Logger {
        LOGGER
            .get()
            .expect("Logger not initialized. Call `init()` first.")
    }
    pub fn log(&self, level: Level, arg: Arguments) {
        let current_local: DateTime<Local> = Local::now();
        let custom_format = current_local.format("%Y-%m-%d %H:%M:%S").to_string();
        let offset = current_local.offset().fix();
        let offset_string = format!("{}", offset);
        if level as u8 <= self.level as u8 {
            if let Some(path) = &self.file_path {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                writeln!(
                    file,
                    "[{}{}]-{:?}: {}",
                    custom_format, offset_string, &level, arg
                )
                .unwrap()
            } else {
                println!("[{}{}]-{:?}: {}", custom_format, offset_string, &level, arg);
            }
        }
    }
}
