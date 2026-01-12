/// Panic with message if `condition` is true
///
/// Accepts either a static message which only needs to implement `std::fmt::Display` or a dynamic closure `FnOnce() -> T where T: std::fmt::Display`
///
/// ## Examples
///
/// ```rust,ignore
/// use resext::panic_if;
///
/// let x = 5;
/// // Static
/// panic_if!(x > 10, "x is too big", 1);
/// // Dynamic
/// panic_if!(x > 10, || format!("x={} is too big", x), 1);
/// ```
/// ## Internals
///
/// Uses `eprintln!("{}", msg)` to print the message then exits with `std::process::exit(code)`.
#[macro_export]
macro_rules! panic_if {
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

/// Return `ErrCtx {msg, source}` if `condition` is true
///
/// ## Examples
///
/// ```rust,ignore
/// use resext::*;
/// use std::io::Error;
///
/// fn ret_static_err() -> CtxResult<(), Error> {
///     let res: CtxResult<(), Error> = return_err_if!(true, "I/O Error", Error::other("Example"));
///
///     res
/// }
///
/// fn ret_dyn_err(number: usize) -> CtxResult<(), Error> {
///     let res: CtxResult<(), Error> = return_err_if!(true, || format!("Operation: {} failed", number), Error::other("Example"));
///
///     res
/// }
/// ```
#[macro_export]
macro_rules! return_err_if {
    ($condition: expr, || $msg:expr, $source:expr) => {
        if $condition {
            Err($crate::ErrCtx::new($source, $msg.as_bytes().to_vec()))
        } else {
            Ok(())
        }
    };

    ($condition: expr, $msg:expr, $source:expr) => {
        if $condition {
            Err($crate::ErrCtx::new($source, $msg.as_bytes().to_vec()))
        } else {
            Ok(())
        }
    };
}
