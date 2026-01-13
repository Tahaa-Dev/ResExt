#[macro_export]
macro_rules! enumerate {
    (
    $(#[$meta:meta])*
    $vis:vis enum $name:ident {
        $($variant:ident $(($type:ty))?),* $(,)?
    }) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis enum $name {
            $($variant $(($type))?),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(__display_match_arm_pat!(var, $name, $variant $(($type))?) => __display_match_arm_write!(var, f, $variant $(($type))?)),*
                }
            }
        }

        impl std::error::Error for $name {}

        $(__from_impl!($name, $variant $(($type))?);)*

        $vis type Res<T> = $crate::CtxResult<T, $name>;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __display_match_arm_pat {
    ($var:ident, $name:ident, $variant:ident ($type:ty)) => {
        $name::$variant($var)
    };

    ($var:ident, $name:ident, $variant:ident) => {
        $name::$variant
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __display_match_arm_write {
    ($var:ident, $f:ident, $variant:ident ($type:ty)) => {
        write!($f, "{}", $var)
    };

    ($var:ident, $f:ident, $variant:ident) => {
        write!($f, "{}", stringify!($variant))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __from_impl {
    ($name:ident, $variant:ident) => {};

    ($name:ident, $variant:ident ($type:ty)) => {
        impl From<$type> for $crate::ErrCtx<$name> {
            fn from(value: $type) -> Self {
                Self::new($name::$variant(value), Vec::new())
            }
        }
    };
}
