pub mod config;
pub mod logger;
pub mod macros; // info!, warn!, etc.

pub use crate::config::log::Level;
pub use config::Config;
pub use logger::Logger;

pub fn init(config: Config) {
    Logger::init(Logger {
        level: config.level,
        log_file: config.log_to_file,
        file_path: config.file_path.clone(),
    });
}
