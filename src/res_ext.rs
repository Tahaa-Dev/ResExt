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

    /// Does the same as `better_expect` but takes a closure that evaluates to a `String` (`FnOnce() -> String`) instead of a static `&'static str` for dynamic errors.
    fn dyn_expect<F>(self, closure: F, code: i32, verbose: bool) -> T
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

    fn context(self, msg: &'static str) -> Result<T, Ctx<E>> {
        context::new_context_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String,
    {
        with_context::with_context_impl(self, closure)
    }

    fn byte_context(self, bytes: &[u8]) -> Result<T, Ctx<E>> {
        context::byte_context_impl(self, bytes)
    }
}
