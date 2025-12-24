use core::error::Error;
use std::fmt::{Debug, Display};

use crate::ResExt;
use crate::res_ext_methods::*;

pub struct Ctx<E: Error> {
    pub(crate) msg: Vec<u8>,
    pub(crate) source: E,
}

impl<E: Display + Error> Display for Ctx<E> {
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

impl<E: Debug + Error> Debug for Ctx<E> {
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

impl<E: Display + Error + 'static> Error for Ctx<E> {
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

impl<E: Error> From<E> for Ctx<E> {
    fn from(value: E) -> Self {
        Self { msg: Vec::with_capacity(0), source: value }
    }
}

impl<E: Error> Ctx<E> {
    pub fn new(source: E, msg: &[u8]) -> Self {
        Self { msg: msg.to_vec(), source }
    }

    pub fn msg(&self) -> String {
        unsafe { std::string::String::from_utf8_unchecked(self.msg.to_vec()) }
    }
}

unsafe impl<E: Error> Sync for Ctx<E> {}
unsafe impl<E: Error> Send for Ctx<E> {}

impl<T, E: Error> ResExt<T, E> for Result<T, Ctx<E>> {
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
        context::extra_ctx_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String,
    {
        with_context::extra_with_ctx_impl(self, closure)
    }

    fn byte_context(self, bytes: &[u8]) -> Result<T, Ctx<E>> {
        context::extra_byte_context_impl(self, bytes)
    }
}
