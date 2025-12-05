use std::borrow::Cow;

use crate::Ctx;

pub(crate) fn with_context_impl<T, E, F>(res: Result<T, E>, closure: F) -> Result<T, Ctx<E>>
where
    F: Fn(&E) -> String,
{
    res.map_err(|e| Ctx::new(Cow::Owned(closure(&e)), e))
}

pub(crate) fn extra_with_ctx_impl<T, E, F>(res: Result<T, Ctx<E>>, closure: F) -> Result<T, Ctx<E>>
where
    F: Fn(&E) -> String,
{
    res.map_err(|ctx| {
        Ctx::new(
            Cow::Owned(format!("{}\n{}", ctx.msg, closure(&ctx.source))),
            ctx.source,
        )
    })
}
