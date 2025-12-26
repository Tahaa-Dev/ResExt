use core::error::Error;
use std::fmt::{Debug, Display};

use crate::ResExt;
use crate::res_ext_methods::*;

/// Custom error struct `ErrCtx`.
///
/// Stores two fields, a `msg: Vec<u8>` field which stores the context messages as raw bytes for
/// less memory usage, and `source: E` where `E: std::error::Error` for the lower-level source of
/// the error.
///
/// The `Vec<u8>` for messages will be optimized even further in nexy versions to use `smallvec` or
/// a similar crate for zero-alloc small contexts while longer ones stay alloc.
pub struct ErrCtx<E: Error> {
    pub msg: Vec<u8>,
    pub source: E,
}

impl<E: Display + Error> Display for ErrCtx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.msg.is_empty() {
            write!(f, "{}", &self.source)
        } else {
            write!(
                f,
                "{}\nCaused by: {}\n",
                unsafe { std::str::from_utf8_unchecked(&self.msg) },
                &self.source
            )
        }
    }
}

impl<E: Debug + Error> Debug for ErrCtx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.msg.is_empty() {
            write!(f, "{:?}", &self.source)
        } else {
            write!(
                f,
                "{}\nCaused by: {:?}\n",
                unsafe { std::str::from_utf8_unchecked(&self.msg) },
                &self.source
            )
        }
    }
}

impl<E: Display + Error + 'static> Error for ErrCtx<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }

    fn description(&self) -> &str {
        "Context for error"
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(&self.source)
    }
}

impl<E: Error> From<E> for ErrCtx<E> {
    fn from(value: E) -> Self {
        Self { msg: Vec::with_capacity(0), source: value }
    }
}

impl<E: Error> ErrCtx<E> {
    /// Function for constructing a new `ErrCtx<E>` struct from a raw source error and a `Vec<u8>`
    /// of bytes that gets consumed in the function.
    pub fn new(source: E, mut msg: Vec<u8>) -> Self {
        Self { msg: std::mem::take(&mut msg), source }
    }

    /// Method for getting the messages in a `ErrCtx<E>` struct in `&str` format instead of raw
    /// accessing the `Vec<u8>`.
    pub fn msg(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.msg.as_slice()) }
    }
}

unsafe impl<E: Error> Sync for ErrCtx<E> {}
unsafe impl<E: Error> Send for ErrCtx<E> {}

impl<T, E: Error> ResExt<T, E> for Result<T, ErrCtx<E>> {
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
        context::extra_ctx_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, ErrCtx<E>>
    where
        F: FnOnce() -> String,
    {
        with_context::extra_with_ctx_impl(self, closure)
    }

    fn byte_context(self, bytes: Vec<u8>) -> Result<T, ErrCtx<E>> {
        context::extra_byte_context_impl(self, bytes.as_slice())
    }
}
