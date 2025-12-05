use std::borrow::Cow;

use crate::Ctx;

pub(crate) fn new_context_impl<T, E>(res: Result<T, E>, msg: &'static str) -> Result<T, Ctx<E>> {
    res.map_err(|e| Ctx::new(Cow::Borrowed(msg), e))
}

pub(crate) fn extra_ctx_impl<T, E>(
    res: Result<T, Ctx<E>>,
    new_msg: &'static str,
) -> Result<T, Ctx<E>> {
    res.map_err(|ctx| Ctx::new(Cow::Owned(format!("{}\n{}", ctx.msg, new_msg)), ctx.source))
}
