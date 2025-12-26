use core::error::Error;

use crate::ctx::ErrCtx;

pub(crate) fn with_context_impl<T, E: Error, F>(
    res: Result<T, E>,
    closure: F,
) -> Result<T, ErrCtx<E>>
where
    F: FnOnce() -> String,
{
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(ErrCtx { msg: closure().as_bytes().to_vec(), source: e }),
    }
}

pub(crate) fn extra_with_ctx_impl<T, E: Error, F>(
    res: Result<T, ErrCtx<E>>,
    closure: F,
) -> Result<T, ErrCtx<E>>
where
    F: FnOnce() -> String,
{
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            let s = closure();
            let bytes = s.as_bytes();
            let len = bytes.len();
            let cap = e.msg.capacity();
            if cap <= 3 + len {
                e.msg.reserve_exact(3 + len - cap);
            }
            e.msg.extend_from_slice(b"\n- ");
            e.msg.extend_from_slice(bytes);
            Err(e)
        }
    }
}
