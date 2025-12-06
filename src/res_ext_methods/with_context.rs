use crate::{Ctx, ctx::CtxMsg};

pub(crate) fn with_context_impl<T, E, F>(res: Result<T, E>, closure: F) -> Result<T, Ctx<E>>
where
    F: FnOnce() -> String,
{
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => {
            let mut ctx = Ctx::new(e);
            ctx.msg.push(CtxMsg::Dynamic(closure()));
            Err(ctx)
        }
    }
}

pub(crate) fn extra_with_ctx_impl<T, E, F>(res: Result<T, Ctx<E>>, closure: F) -> Result<T, Ctx<E>>
where
    F: FnOnce() -> String,
{
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            e.msg.push(CtxMsg::Dynamic(closure()));
            Err(e)
        }
    }
}
