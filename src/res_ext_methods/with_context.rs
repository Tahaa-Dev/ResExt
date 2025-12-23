use std::error::Error;

use crate::ctx::Ctx;

pub(crate) fn with_context_impl<T, E: Error, F>(res: Result<T, E>, closure: F) -> Result<T, Ctx<E>>
where
    F: FnOnce() -> String,
{
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(Ctx {
            msg: closure().as_bytes().to_vec(),
            source: e,
        }),
    }
}

pub(crate) fn extra_with_ctx_impl<T, E: Error, F>(
    res: Result<T, Ctx<E>>,
    closure: F,
) -> Result<T, Ctx<E>>
where
    F: FnOnce() -> String,
{
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            let string = closure();
            let bytes = string.as_bytes();
            e.msg.reserve_exact(3 + bytes.len());
            e.msg.extend_from_slice(b"\n- ");
            e.msg.extend_from_slice(bytes);
            Err(e)
        }
    }
}
