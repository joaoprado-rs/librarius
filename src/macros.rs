#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::Logger::get().log($crate::config::log::Level::Info, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::Logger::get().log($crate::config::log::Level::Warn, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::Logger::get().log($crate::config::log::Level::Error, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::Logger::get().log($crate::config::log::Level::Debug, format_args!($($arg)*));
    };
}

/*
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

/*
 * `$x` represents the argument inside the pattern.
 *
 * Example:
 *  - `$x:expr` can be an expression.
 *  - `$x:literal` can be a literal value.
 */

/*
 * Token types:
 *
 * - `expr`: Any valid Rust expression. It can be followed by `=>`, `,`, or `;` depending on the macro's syntax.
 * - `literal`: A literal value, such as a number or string, which may be followed by any other tokens.
 * - `tt` (token tree): Represents a **generic sequence of tokens**. This is the most flexible and powerful token type,
 *   allowing any valid Rust syntax
 */
