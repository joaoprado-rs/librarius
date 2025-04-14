pub mod log {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Level {
        Info,
        Warn,
        Error,
        Debug,
    }
}

pub struct Config {
    pub level: log::Level,
    pub log_to_file: bool,
    pub file_path: Option<String>,
}

impl Config {
    pub fn new(level: log::Level) -> Self {
        Config {
            level,
            log_to_file: false,
            file_path: None,
        }
    }

    pub fn with_file(level: log::Level, file_path: &str) -> Self {
        Config {
            level: level,
            log_to_file: true,
            file_path: Some(file_path.to_string()),
        }
    }
}
