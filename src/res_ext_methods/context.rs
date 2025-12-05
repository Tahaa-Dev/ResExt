use std::borrow::Cow;

use crate::Ctx;

pub(crate) fn new_context_impl<T, E>(res: Result<T, E>, msg: &'static str) -> Result<T, Ctx<E>> {
    res.map_err(|e| {
        let mut ctx = Ctx::new(e);
        ctx.msg.push(Cow::Borrowed(msg));
        ctx
    })
}

pub(crate) fn extra_ctx_impl<T, E>(res: Result<T, Ctx<E>>, msg: &'static str) -> Result<T, Ctx<E>> {
    res.map_err(|mut e| {
        e.msg.push(Cow::Borrowed(msg));
        e
    })
}
