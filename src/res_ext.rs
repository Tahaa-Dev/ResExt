use std::fmt::Display;

use crate::Ctx;
use crate::res_ext_methods::*;

pub trait ResExt<T, E> {
    /// Similar to `.unwrap()` but exits without printing anything.
    fn or_exit(self, code: i32) -> T;

    /// Prints an error messge and exits with the provided code if an error occurs, returns the
    /// value `T` if the `Result<T, E>` is `Ok(T)` with optional context if verbose is true.
    fn better_expect(self, msg: &str, code: i32, verbose: bool) -> T
    where
        E: Display;

    /// Prints a message if error occurs but doesn't panic and instead returns the default value
    /// provided.
    fn or_default_context(self, msg: &str, default: T, verbose: bool) -> T
    where
        E: Display;

    /// Adds context to the error.
    fn context(self, msg: &'static str) -> Result<T, Ctx<E>>;

    /// Adds context but takes a closure that evaluates to a `&str` instead of a static `&str`.
    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: Fn(&E) -> &'static str;

    /// Converts the error into the expected type by the function signature.
    fn map_err_into<E2>(self) -> Result<T, E2>
    where
        E: Into<E2>;

    /// Does the same as `.ok()` exposed by std but prints a context message without panicking if value is `Err(E)`.
    fn to_option_context(self, msg: &'static str, verbose: bool) -> Option<T>
    where
        E: Display;
}

impl<T, E> ResExt<T, E> for Result<T, E> {
    fn or_exit(self, code: i32) -> T {
        or_exit::or_exit_impl(self, code)
    }

    fn better_expect(self, msg: &str, code: i32, verbose: bool) -> T
    where
        E: Display,
    {
        better_expect::better_expect_impl(self, msg, code, verbose)
    }

    fn or_default_context(self, msg: &str, default: T, verbose: bool) -> T
    where
        E: Display,
    {
        or_default_context::or_default_context_impl(self, msg, default, verbose)
    }

    fn context(self, msg: &'static str) -> Result<T, Ctx<E>> {
        context::context_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: Fn(&E) -> &'static str,
    {
        with_context::with_context_impl(self, closure)
    }

    fn map_err_into<E2>(self) -> Result<T, E2>
    where
        E: Into<E2>,
    {
        map_err_into::map_err_into_impl(self)
    }

    fn to_option_context(self, msg: &'static str, verbose: bool) -> Option<T>
    where
        E: Display,
    {
        to_option_context::to_option_context_impl(self, msg, verbose)
    }
}
