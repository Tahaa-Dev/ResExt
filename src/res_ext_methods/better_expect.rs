use std::{fmt::Display, process::exit};

pub(crate) fn better_expect_impl<T, E>(res: Result<T, E>, msg: &str, code: i32, verbose: bool) -> T
where
    E: Display,
{
    match res {
        Ok(t) => t,
        Err(e) if verbose => {
            eprintln!("{}: {}", msg, e);
            exit(code);
        }
        Err(_) => {
            eprintln!("{}", msg);
            exit(code);
        }
    }
}

pub(crate) fn dyn_expect_impl<T, E, F>(res: Result<T, E>, closure: F, code: i32, verbose: bool) -> T
where
    E: Display,
    F: FnOnce() -> String,
{
    match res {
        Ok(t) => t,
        Err(e) if verbose => {
            eprintln!("{}: {}", closure(), e);
            exit(code);
        }
        Err(_) => {
            eprintln!("{}", closure());
            exit(code);
        }
    }
}
