#[macro_export]
macro_rules! enumerate {
    ($(#[$meta:meta])*
    $vis:vis enum $name:ident {
        $($variant:ident $(($type:ty))?),* $(,)?
    }) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis enum $name {
            $($variant $(($type))?),*
        }
    };
}
