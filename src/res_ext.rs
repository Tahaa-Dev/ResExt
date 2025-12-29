use core::error::Error;
use std::fmt::Display;

use crate::ctx::ErrCtx;
use crate::res_ext_methods::*;

pub trait ResExt<T, E: Error> {
    /// Similar to `.unwrap()` but exits without printing anything to stderr or stdout.
    fn or_exit(self, code: i32) -> T;

    /// Prints an error messge and exits with the provided code if an error occurs, returns the
    /// value `T` if the `Result<T, E>` is `Ok(T)` with optional context if verbose is true.
    fn better_expect(self, msg: &str, code: i32, verbose: bool) -> T
    where
        E: Display;

    /// Does the same as `better_expect` but takes a closure that evaluates to a `String`
    /// (`FnOnce() -> String`) instead of a static `&'static str` for dynamic errors.
    fn dyn_expect<F>(self, closure: F, code: i32, verbose: bool) -> T
    where
        E: Display,
        F: FnOnce() -> String;

    /// Adds context to the error.
    ///
    /// Uses a single `Vec<u8>` allocation on the first context in the chain for messages as
    /// `ErrCtx<E>` stores raw bytes of messages.
    /// This behavior will be optimized in next versions to use `smallvec` (or a similar crate) instead of the `Vec<u8>`.
    ///
    /// ## Examples
    /// ```rust,ignore
    /// use resext::*;
    /// use std::io::{ErrorKind, Error as IoErr};
    /// use toml::Value;
    ///
    /// fn read_config_file() -> CtxResult<Value, IoErr> {
    ///     let path = "config.toml";
    ///     let content = std::fs::read_to_string(path).context("Failed to read config file.")?;
    ///
    ///     let config: Value = toml::from_str(content)
    ///         .map_err(|_| IoErr::new(ErrorKind::InvalidInput, "Invalid TOML in input file"))
    ///         .context("Failed to deserialize config.")
    ///         .with_context(|| "Error in config file: {}.", path)?;
    ///
    ///     Ok(config)
    /// }
    /// ```
    /// - if the first `Result` is `Err`, it'll output:
    /// ```text
    /// Failed to read config file.
    /// Caused by: NotFound
    /// ```
    /// - And if the second `Result` is `Err`:
    /// ```text
    /// Failed to deserialize config.
    /// - Error in config file: config.toml.
    /// Caused by: InvalidInput
    /// ```
    fn context(self, msg: &'static str) -> Result<T, ErrCtx<E>>;

    /// Adds context but takes a closure that evaluates to `String` (`FnOnce() -> String`)
    /// instead of a static `&'static str` for dynamic context.
    ///
    /// Works the same way as `.context()` in every part except the message argument.
    /// See `.context()` docs for more details.
    fn with_context<F>(self, closure: F) -> Result<T, ErrCtx<E>>
    where
        F: FnOnce() -> String;

    /// Works the same as `.context()` but takes a `Vec<u8>` of raw bytes for the message.
    ///
    /// Useful for low-level applications where memory usage and speed matter or where you
    /// have raw bytes instead of strings.
    /// See `.context()` docs for more details.
    ///
    /// **NOTE:** This method consumes the input `Vec<u8>` if it is the first context in the chain, if you need the `Vec<u8>`, use `.clone()`0.
    ///
    /// ## Safety
    /// This method is **unsafe** since it can lead to invalid UTF-8 context
    /// messages.
    unsafe fn byte_context(self, bytes: Vec<u8>) -> Result<T, ErrCtx<E>>;
}

impl<T, E: Error> ResExt<T, E> for Result<T, E> {
    fn or_exit(self, code: i32) -> T {
        or_exit::or_exit_impl(self, code)
    }

    fn better_expect(self, msg: &str, code: i32, verbose: bool) -> T
    where
        E: Display,
    {
        better_expect::better_expect_impl(self, msg, code, verbose)
    }

    fn dyn_expect<F>(self, closure: F, code: i32, verbose: bool) -> T
    where
        E: Display,
        F: FnOnce() -> String,
    {
        better_expect::dyn_expect_impl(self, closure, code, verbose)
    }

    fn context(self, msg: &'static str) -> Result<T, ErrCtx<E>> {
        context::new_context_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, ErrCtx<E>>
    where
        F: FnOnce() -> String,
    {
        with_context::with_context_impl(self, closure)
    }

    unsafe fn byte_context(self, bytes: Vec<u8>) -> Result<T, ErrCtx<E>> {
        context::byte_context_impl(self, bytes)
    }
}
