use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct Ctx<E> {
    pub(crate) msg: Vec<u8>,
    pub source: E,
}

impl<E: Display> Display for Ctx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            unsafe { std::str::from_utf8_unchecked(&self.msg) },
            &self.source
        )
    }
}

impl<E: Debug + Display> Debug for Ctx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?}",
            unsafe { std::str::from_utf8_unchecked(&self.msg) },
            &self.source
        )
    }
}

impl<E: Display + Error + 'static> Error for Ctx<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<E> Ctx<E> {
    pub fn new(source: E) -> Self {
        Self {
            msg: Vec::new(),
            source,
        }
    }
}

pub trait CtxChain<T, E: Display> {
    /// Method for chaining `.context()` without getting nested `Ctx<Ctx<Ctx<E>>>`.
    fn context(self, msg: &'static str) -> Result<T, Ctx<E>>;

    /// Method for chaining `.with_context()` without getting nested `Ctx<Ctx<Ctx<E>>>`.
    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String;

    /// Method for automatically mapping the type for `E` into the usually inferred type to
    /// make `Ctx<E>` more ergonomic because of no `?` propagation for performance.
    fn map_err_into<E2: From<E>>(self) -> Result<T, Ctx<E2>>;

    fn byte_context(self, bytes: &[u8]) -> Result<T, Ctx<E>>;
}

impl<T, E: Display> CtxChain<T, E> for Result<T, Ctx<E>> {
    fn context(self, msg: &'static str) -> Result<T, Ctx<E>> {
        crate::res_ext_methods::context::extra_ctx_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String,
    {
        crate::res_ext_methods::with_context::extra_with_ctx_impl(self, closure)
    }

    fn map_err_into<E2: From<E>>(self) -> Result<T, Ctx<E2>> {
        self.map_err(|e| Ctx {
            msg: e.msg,
            source: e.source.into(),
        })
    }

    fn byte_context(self, bytes: &[u8]) -> Result<T, Ctx<E>> {
        crate::res_ext_methods::context::extra_byte_context_impl(self, bytes)
    }
}
