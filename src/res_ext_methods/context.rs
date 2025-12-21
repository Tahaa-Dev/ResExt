use crate::ctx::Ctx;

pub(crate) fn new_context_impl<T, E>(res: Result<T, E>, msg: &'static str) -> Result<T, Ctx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(Ctx {
            msg: msg.as_bytes().to_vec(),
            source: e,
        }),
    }
}

pub(crate) fn extra_ctx_impl<T, E>(res: Result<T, Ctx<E>>, msg: &'static str) -> Result<T, Ctx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            e.msg.extend_from_slice(b"\n- ");
            e.msg.extend_from_slice(msg.as_bytes());
            Err(e)
        }
    }
}

pub(crate) fn byte_context_impl<T, E>(res: Result<T, E>, bytes: &[u8]) -> Result<T, Ctx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => Err(Ctx {
            msg: bytes.to_vec(),
            source: e,
        }),
    }
}

pub(crate) fn extra_byte_context_impl<T, E>(
    res: Result<T, Ctx<E>>,
    bytes: &[u8],
) -> Result<T, Ctx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            e.msg.extend_from_slice(b"\n- ");
            e.msg.extend_from_slice(bytes);
            Err(e)
        }
    }
}
