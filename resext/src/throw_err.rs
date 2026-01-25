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
