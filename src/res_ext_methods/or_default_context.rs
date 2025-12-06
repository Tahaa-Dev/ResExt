use std::fmt::Display;

pub(crate) fn or_default_context_impl<T, E>(
    res: Result<T, E>,
    msg: &str,
    default: T,
    verbose: bool,
) -> T
where
    E: Display,
{
    match res {
        Ok(t) => t,
        Err(e) if verbose => {
            eprintln!("{}: {}", msg, e);
            default
        }
        Err(_) => {
            eprintln!("{}", msg);
            default
        }
    }
}

pub(crate) fn with_default_context_impl<T, E, F>(
    res: Result<T, E>,
    closure: F,
    default: T,
    verbose: bool,
) -> T
where
    E: Display,
    F: FnOnce() -> String,
{
    match res {
        Ok(t) => t,
        Err(e) if verbose => {
            eprintln!("{}: {}", closure(), e);
            default
        }
        Err(_) => {
            eprintln!("{}", closure());
            default
        }
    }
}
