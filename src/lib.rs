mod ctx;
mod res_ext;

mod res_ext_methods;

pub use res_ext::ResExt;

pub use crate::ctx::Ctx as ErrCtx;

pub type CtxResult<T, E> = std::result::Result<T, ErrCtx<E>>;

/// Throw an error if `condition` is true.
///
/// Accepts either a static message which only needs to implement `std::fmt::Display` or a dynamic closure `FnOnce() -> T where T: std::fmt::Display`
///
/// ## Examples
/// ```
/// use resext::throw_err_if;
///
/// fn main() {
///     let x = 5;
///     // Static
///     throw_err_if!(x > 10, "x is too big", 1);
///     // Dynamic
///     throw_err_if!(x > 10, || format!("x={} is too big", x), 1);
/// }
/// ```
/// ## Internals
///
/// Uses `eprintln!("{}", msg)` to print the message then exits with `std::process::exit(code)`.
#[macro_export]
macro_rules! throw_err_if {
    ($condition:expr, || $msg:expr, $code:expr) => {
        if $condition {
            eprintln!("{}", $msg);
            std::process::exit($code);
        }
    };

    ($condition:expr, $msg:expr, $code:expr) => {
        if $condition {
            eprintln!("{}", $msg);
            std::process::exit($code);
        }
    };
}
