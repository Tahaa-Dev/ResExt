use crate::{Ctx, ctx::CtxMsg};

pub(crate) fn new_context_impl<T, E>(res: Result<T, E>, msg: &'static str) -> Result<T, Ctx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(e) => {
            let mut ctx = Ctx::new(e);
            ctx.msg.push(CtxMsg::Static(msg));
            Err(ctx)
        }
    }
}

pub(crate) fn extra_ctx_impl<T, E>(res: Result<T, Ctx<E>>, msg: &'static str) -> Result<T, Ctx<E>> {
    match res {
        Ok(ok) => Ok(ok),
        Err(mut e) => {
            e.msg.push(CtxMsg::Static(msg));
            Err(e)
        }
    }
}
