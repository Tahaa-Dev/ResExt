use crate::Ctx;

pub(crate) fn with_context_impl<T, E, F>(res: Result<T, E>, closure: F) -> Result<T, Ctx<E>>
where
    F: Fn(&E) -> &'static str,
{
    res.map_err(|e| Ctx::new(closure(&e), e))
}
