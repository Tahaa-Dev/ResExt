use std::fmt::Display;

pub(crate) fn to_option_context_impl<T, E>(
    res: Result<T, E>,
    msg: &'static str,
    verbose: bool,
) -> Option<T>
where
    E: Display,
{
    match res {
        Ok(t) => Some(t),
        Err(e) if verbose => {
            eprintln!("{}: {}", msg, e);
            None
        }
        Err(_) => {
            eprintln!("{}", msg);
            None
        }
    }
}

pub(crate) fn with_option_context_impl<T, E, F>(
    res: Result<T, E>,
    closure: F,
    verbose: bool,
) -> Option<T>
where
    E: Display,
    F: FnOnce() -> String,
{
    match res {
        Ok(t) => Some(t),
        Err(e) if verbose => {
            eprintln!("{}: {}", closure(), e);
            None
        }
        Err(_) => {
            eprintln!("{}", closure());
            None
        }
    }
}
