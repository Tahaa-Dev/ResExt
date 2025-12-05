use std::borrow::Cow;

use crate::Ctx;

pub(crate) fn with_context_impl<T, E, F>(res: Result<T, E>, closure: F) -> Result<T, Ctx<E>>
where
    F: Fn(&E) -> String,
{
    res.map_err(|e| {
        let message = Cow::Owned(closure(&e));
        let mut ctx = Ctx::new(e);
        ctx.msg.push(message);
        ctx
    })
}

pub(crate) fn extra_with_ctx_impl<T, E, F>(res: Result<T, Ctx<E>>, closure: F) -> Result<T, Ctx<E>>
where
    F: Fn(&E) -> String,
{
    res.map_err(|mut e| {
        e.msg.push(Cow::Owned(closure(&e.source)));
        e
    })
}
