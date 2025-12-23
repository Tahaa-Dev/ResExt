mod ctx;
mod res_ext;

mod res_ext_methods;

use std::process::exit;

pub use res_ext::ResExt;

pub use error::Ctx as ErrCtx;
pub use error::CtxResult;

/// Module for custom error types defined by ResExt.
mod error {
    pub use crate::ctx::Ctx;
    pub type CtxResult<T, E> = std::result::Result<T, Ctx<E>>;
}

/// Takes a `condition` which is a closure that evaluates to a `bool` (`FnOnce() -> bool`) and if it returns true, it
/// prints an error message to stderr and exits with provided code.
pub fn throw_err_if<C: FnOnce() -> bool>(condition: C, msg: &'static str, code: i32) {
    if condition() {
        eprintln!("{}", msg);
        exit(code);
    }
}

pub fn dyn_error_if<F: FnOnce() -> String, C: FnOnce() -> bool>(
    condition: C,
    closure: F,
    code: i32,
) {
    if condition() {
        eprintln!("{}", closure());
        exit(code);
    }
}
