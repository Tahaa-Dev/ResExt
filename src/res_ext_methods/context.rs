use core::error::Error;

use crate::ctx::ErrCtx;

pub(crate) fn new_context_impl<T, E: Error>(
    res: Result<T, E>,
    msg: &'static str,
) -> Result<T, ErrCtx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(ErrCtx { msg: msg.as_bytes().to_vec(), source: e }),
    }
}

pub(crate) fn extra_ctx_impl<T, E: Error>(
    res: Result<T, ErrCtx<E>>,
    msg: &'static str,
) -> Result<T, ErrCtx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            let bytes = msg.as_bytes();
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

pub(crate) fn byte_context_impl<T, E: Error>(
    res: Result<T, E>,
    mut bytes: Vec<u8>,
) -> Result<T, ErrCtx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(ErrCtx { msg: std::mem::take(&mut bytes), source: e }),
    }
}

pub(crate) fn extra_byte_context_impl<T, E: Error>(
    res: Result<T, ErrCtx<E>>,
    bytes: &[u8],
) -> Result<T, ErrCtx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
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
