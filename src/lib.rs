mod ctx;
mod opt_ext;
mod res_ext;

// mod opt_ext_methods;
mod res_ext_methods;

use std::process::exit;

// pub use opt_ext::OptExt;
pub use res_ext::ResExt;

pub use error::Ctx as ErrCtx;
pub use error::CtxResult;

/// Module for custom error types defined by ResExt.
mod error {
    pub use crate::ctx::Ctx;
    pub type CtxResult<T, E> = std::result::Result<T, Ctx<E>>;
}
/// Function for printing a message to `stderr` and exiting with provided code. The message is
/// printed on a separate line.
pub fn throw_err(msg: &'static str, code: i32) {
    eprintln!("{}", msg);
    exit(code);
}

/// Takes a `condition` which is a closure that evaluates to a `bool` (`FnOnce() -> bool`) and if it returns true, it
/// does the same as `throw_err()`. See `throw_err()` for more documentation.
pub fn throw_err_if<C: FnOnce() -> bool>(condition: C, msg: &'static str, code: i32) {
    if condition() {
        eprintln!("{}", msg);
        exit(code);
    }
}

/// Does the same as `throw_err()` but takes a`FnOnce() -> String` for the message instead of a static
/// `&'static str`. See `throw_err()` for more documentation.
pub fn throw_dyn_error<F: FnOnce() -> String>(closure: F, code: i32) {
    eprintln!("{}", closure());
    exit(code);
}

/// The equivalent of `throw_err_if()` for `throw_err()` but for `throw_dyn_error()`. See `throw_err()` for more documentation.
pub fn throw_dyn_error_if<F: FnOnce() -> String, C: FnOnce() -> bool>(
    condition: C,
    closure: F,
    code: i32,
) {
    if condition() {
        eprintln!("{}", closure());
        exit(code);
    }
}
