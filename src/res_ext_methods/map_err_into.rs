pub(crate) fn map_err_into_impl<T, E, E2>(res: Result<T, E>) -> Result<T, E2>
where
    E: Into<E2>,
{
    res.map_err(|e| e.into())
}
