pub mod log {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Level {
        Info,
        Warn,
        Error,
        Debug,
    }
}
pub struct Config {
    pub level: log::Level,
}

impl Config {
    pub fn new(level: log::Level) -> Self {
        Config { level }
    }
}
