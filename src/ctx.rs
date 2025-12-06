use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::res_ext_methods::{context::extra_ctx_impl, with_context::extra_with_ctx_impl};

pub struct Ctx<E> {
    pub msg: Vec<CtxMsg>,
    pub source: E,
}

pub(crate) enum CtxMsg {
    Static(&'static str),
    Dynamic(String),
}

impl Display for CtxMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CtxMsg::Static(s) => write!(f, "{}", s),
            CtxMsg::Dynamic(s) => write!(f, "{}", s),
        }
    }
}

impl<E: Display> Display for Ctx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        for msg in &self.msg {
            if first {
                write!(f, "{}", msg)?;
                first = false;
            } else {
                write!(f, "\n{}", msg)?;
            }
        }
        write!(f, ": {}", &self.source)
    }
}

impl<E: Debug + Display> Debug for Ctx<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        for msg in &self.msg {
            if first {
                write!(f, "{}", msg)?;
                first = false;
            } else {
                write!(f, "\n{}", msg)?;
            }
        }
        write!(f, ": {:?}", &self.source)
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

pub trait CtxChain<T, E> {
    /// Method for chaining `.context()` without getting nested `Ctx<Ctx<Ctx<E>>>`.
    fn context(self, msg: &'static str) -> Result<T, Ctx<E>>;

    /// Method for chaining `.with_context()` without getting nested `Ctx<Ctx<Ctx<E>>>`.
    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String;
}

impl<T, E> CtxChain<T, E> for Result<T, Ctx<E>> {
    fn context(self, msg: &'static str) -> Result<T, Ctx<E>> {
        extra_ctx_impl(self, msg)
    }

    fn with_context<F>(self, closure: F) -> Result<T, Ctx<E>>
    where
        F: FnOnce() -> String,
    {
        extra_with_ctx_impl(self, closure)
    }
}
