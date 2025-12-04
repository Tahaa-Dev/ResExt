use crate::Ctx;

pub(crate) fn context_impl<T, E>(res: Result<T, E>, msg: &'static str) -> Result<T, Ctx<E>> {
    res.map_err(|e| Ctx::new(msg, e))
}
