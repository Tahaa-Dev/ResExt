use std::process::exit;

pub(crate) fn or_exit_impl<T, E>(res: Result<T, E>, code: i32) -> T {
    match res {
        Ok(t) => t,
        Err(_) => exit(code),
    }
}
