use std::error::Error;
use std::fmt::Display;

use crate::ctx::Ctx;
use crate::res_ext_methods::*;

pub trait ResExt<T, E: Error> {
    /// Similar to `.unwrap()` but exits without printing anything.
    fn or_exit(self, code: i32) -> T;

    /// Prints an error messge and exits with the provided code if an error occurs, returns the
    /// value `T` if the `Result<T, E>` is `Ok(T)` with optional context if verbose is true.
    fn better_expect(self, msg: &str, code: i32, verbose: bool) -> T
    where
        E: Display;

    /// Does the same as `better_expect` but takes a closure that evaluates to a something that implements `std::fmt::Display` and `'static` instead of a static `&'static str` for dynamic errors.    
    fn dyn_expect<F>(self, closure: F, code: i32, verbose: bool) -> T
    where
        E: Display,
        F: FnOnce() -> String;

    /// Prints a message if error occurs but doesn't panic and instead returns the default value
    /// provided.
    fn or_default_context(self, msg: &str, default: T, verbose: bool) -> T
    where
        E: Display;

    /// Does the same as `.or_default_context()` but uses dynamic error messages by taking a
    /// `FnOnce() -> String` instead of a static `&'static str`.
    fn with_default_context<F>(self, closure: F, default: T, verbose: bool) -> T
    where
        E: Display,
        F: FnOnce() -> String;

    /// Adds context to the error.
    fn context(self, msg: &'static str) -> Result<T, Ctx<E>>;

    /// Adds context but takes a closure that evaluates to a something that implements `std::fmt::Display` and `'static` instead of a static `&'static str` for dynamic errors.
    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String;

    /// Works the same as `.context()` but takes a `&[u8]` of raw bytes for the message.
    fn byte_context(self, bytes: &[u8]) -> Result<T, Ctx<E>>;

    /// Does the same as `.ok()` exposed by std but prints a context message without panicking if value is `Err(E)`.
    fn to_option_context(self, msg: &'static str, verbose: bool) -> Option<T>
    where
        E: Display;

    /// Works like `.to_option_context()` but takes a `FnOnce() -> String` instead of a static
    /// `&'static str` for dynamic messages.
    fn with_option_context<F>(self, closure: F, verbose: bool) -> Option<T>
    where
        E: Display,
        F: FnOnce() -> String;
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

    fn or_default_context(self, msg: &str, default: T, verbose: bool) -> T
    where
        E: Display,
    {
        or_default_context::or_default_context_impl(self, msg, default, verbose)
    }

    fn with_default_context<F>(self, closure: F, default: T, verbose: bool) -> T
    where
        E: Display,
        F: FnOnce() -> String,
    {
        or_default_context::with_default_context_impl(self, closure, default, verbose)
    }

    fn context(self, msg: &'static str) -> Result<T, Ctx<E>> {
        context::new_context_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String,
    {
        with_context::with_context_impl(self, closure)
    }

    fn to_option_context(self, msg: &'static str, verbose: bool) -> Option<T>
    where
        E: Display,
    {
        to_option_context::to_option_context_impl(self, msg, verbose)
    }

    fn with_option_context<F>(self, closure: F, verbose: bool) -> Option<T>
    where
        E: Display,
        F: FnOnce() -> String,
    {
        to_option_context::with_option_context_impl(self, closure, verbose)
    }

    fn byte_context(self, bytes: &[u8]) -> Result<T, Ctx<E>> {
        context::byte_context_impl(self, bytes)
    }
}
