# Librarius

Librarius is a logging library in Rust that provides logging functionality for different severity levels, such as `Info`, `Warn`, `Error`, and `Debug`. The library allows logs to be sent to the terminal or a file, depending on the configuration. It is easy to integrate and configure within any Rust application.

## Features

- Support for multiple log levels: `Info`, `Warn`, `Error`, `Debug`.
- Logs can be output to the terminal or written to a file.
- Custom log formatting with date, time, and timezone.
- Easy integration with logging macros (`info!`, `warn!`, `error!`).
- Simple configuration using the `Config` struct.

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
librarius = "0.1"

## Usage Example

### Logging at the terminal

```rust
use librarius::{Logger, Level, Config};

fn main() {

    // Create the configuration
    let config = Config::new(Level::Info);
    
    // Initialize the logger
    librarius::init(config);

    // Using the log macros
    info!("This is an info log");
    warn!("This is a warning log");
    error!("This is an error log");
    debug!("This is a debug log");
}```


### Logging at a file
```rust
use librarius::{Logger, Level, Config};

fn main() {

    // Create the configuration
    let config = Config::with_file(Level::Info, String::from("logs.txt"));
    
    // Initialize the logger
    librarius::init(config);

    // Using the log macros
    info!("This is an info log");
    warn!("This is a warning log");
    error!("This is an error log");
    debug!("This is a debug log");
}```

## Configuration
You can configure Librarius by creating a Config, where you define the log level and whether logs should be written to a file:

```rust
use librarius::{Config, Level};

let config = Config::with_file(Level::Info, String::from("logs.txt"));
```

## License
This project is licensed under the MIT License - see the LICENSE file for details.