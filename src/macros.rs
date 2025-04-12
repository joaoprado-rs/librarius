#[macro_export]
/// Macro that logs a message at the "Info" level.
///
/// The `info!` macro accepts any number of formatted arguments,
/// uses `format_args!` to create an `Arguments` value,
/// and then passes it to the `Logger::log` method to log the message.
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::log::Level::Info, format_args!($($arg)*));
    };
}


#[macro_export]
/// Macro that logs a message at the "Warn" level.
///
/// The `warn!` macro works similarly to the `info!` macro, but logs the message
/// with a "Warn" (warning) severity level, highlighting messages that need attention.
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::log::Level::Warn, format_args!($($arg)*));
    };
}



#[macro_export]
/// Macro that logs a message at the "Error" level.
///
/// The `error!` macro is used to log error messages, typically associated with serious
/// failures or critical problems that require immediate attention. It passes the arguments
/// to the `Logger::log` function with the "Error" severity level.
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::log::Level::Error, format_args!($($arg)*));
    };
}

#[macro_export]
/// Macro that logs a message at the "Debug" level.
///
/// The `debug!` macro is used to log debug messages. It is typically enabled during development
/// to provide detailed information about the program's execution.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log($crate::log::Level::Debug, format_args!($($arg)*));
    };
}

/**
 * Macro argument patterns in Rust:
 * 
 * - `$(...)*`: 0 or more arguments, **without separator**. Example: `debug!(foo, bar, baz)`.
 * - `$(...),*`: 0 or more arguments, **separated by commas**. Example: `debug!(foo, bar, baz,)`.
 * - `$(...);*`: 0 or more arguments, **separated by semicolons**. Example: `debug!(foo; bar; baz;)`.
 * - `$(...)+`: 1 or more arguments, **without separator**. Example: `debug!(foo bar baz)`.
 * - `$(...),+`: 1 or more arguments, **separated by commas**. Example: `debug!(foo, bar, baz)`.
 * - `$(...);+`: 1 or more arguments, **separated by semicolons**. Example: `debug!(foo; bar; baz)`.
 * - `$(...)?`: 0 or 1 argument, **without separator**. Example: `debug!(foo)`.
 * - `$(...),?`: 0 or 1 argument, **separated by commas**. Example: `debug!(foo,)`.
 * - `$(...);?`: 0 or 1 argument, **separated by semicolons**. Example: `debug!(foo;)`.
 */

/**
 * `$x` represents the argument inside the pattern.
 * 
 * Example:
 *  - `$x:expr` can be an expression.
 *  - `$x:literal` can be a literal value.
 */

/**
 * Token types:
 * 
 * - `expr`: Any valid Rust expression. It can be followed by `=>`, `,`, or `;` depending on the macro's syntax.
 * - `literal`: A literal value, such as a number or string, which may be followed by any other tokens.
 * - `tt` (token tree): Represents a **generic sequence of tokens**. This is the most flexible and powerful token type,
 *   allowing any valid Rust syntax
 */
