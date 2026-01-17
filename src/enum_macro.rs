#[macro_export]
macro_rules! ResExt {
    (
    $(#[$meta:meta])*
    $vis:vis enum $name:ident {
        $($variant:ident $(($type:ty))?),* $(,)?
    }
    $(as $alias:ident)?) => {
        $(#[$meta])*
        #[allow(dead_code)]
        #[derive(Debug)]
        $vis enum $name {
            $($variant $(($type))?),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($crate::__display_match_arm_pat!(var, $name, $variant $(($type))?) => $crate::__display_match_arm_write!(var, f, $variant $(($type))?)),*
                }
            }
        }

        impl std::error::Error for $name {}

        $vis struct ResErr {
            msg: Vec<u8>,
            $vis source: $name,
        }

        $crate::__alias_helper!($vis $($alias)?);

        impl std::fmt::Display for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.is_empty() {
                    write!(f, "{}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nCaused by: {}\n",
                        unsafe { std::str::from_utf8_unchecked(&self.msg) },
                        &self.source
                    )
                }
            }
        }

        impl std::fmt::Debug for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.is_empty() {
                    write!(f, "{:?}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nCaused by: {:?}\n",
                        unsafe { std::str::from_utf8_unchecked(&self.msg) },
                        &self.source
                    )
                }
            }
        }

        impl From<$name> for ResErr {
            fn from(value: $name) -> Self {
                Self { msg: Vec::new(), source: value }
            }
        }

        $vis trait ResExt<T> {
            fn context(self, msg: &str) -> Result<T, ResErr>;

            fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr>;

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr>;

            fn or_exit(self, code: i32) -> T;

            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T;
        }

        // Hide unnecessary implementation details from docs
        $crate::__impl_resext!($name);

        $($crate::__from_impl!($name, $variant $($type)?);)*
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
    ($enum:ident, $variant:ident) => {};

    ($enum:ident, $variant:ident $type:ty) => {
        impl From<$type> for ResErr {
            fn from(value: $type) -> Self {
                Self { msg: Vec::new(), source: $enum::$variant(value) }
            }
        }

        impl From<$type> for $enum {
            fn from(value: $type) -> Self {
                Self::$variant(value)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __alias_helper {
    ($vis:vis $alias:ident) => {
        $vis type $alias<T> = Result<T, ResErr>;
    };

    ($vis:vis) => {
        $vis type Res<T> = Result<T, ResErr>;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_resext {
    ($enum:ident) => {
        impl<T> ResExt<T> for Result<T, ResErr> {
            fn context(self, msg: &str) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.is_empty() {
                            err.msg.extend_from_slice(msg.as_bytes());
                        } else {
                            let bytes = msg.as_bytes();
                            let diff: isize = err.msg.capacity() as isize - bytes.len() as isize;
                            if diff < 0 {
                                err.msg.reserve_exact((-diff) as usize);
                            }
                            err.msg.extend_from_slice(b"\n- ");
                            err.msg.extend_from_slice(bytes);
                        }
                        Err(err)
                    }
                }
            }

            fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.is_empty() {
                            err.msg.extend_from_slice(f().as_bytes());
                        } else {
                            let s = f();
                            let bytes = s.as_bytes();
                            let diff: isize = err.msg.capacity() as isize - bytes.len() as isize;
                            if diff < 0 {
                                err.msg.reserve_exact((-diff) as usize);
                            }
                            err.msg.extend_from_slice(b"\n- ");
                            err.msg.extend_from_slice(bytes);
                        }
                        Err(err)
                    }
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.is_empty() {
                            err.msg.extend_from_slice(bytes);
                        } else {
                            let diff: isize = err.msg.capacity() as isize - bytes.len() as isize;
                            if diff < 0 {
                                err.msg.reserve_exact((-diff) as usize);
                            }
                            err.msg.extend_from_slice(b"\n- ");
                            err.msg.extend_from_slice(bytes);
                        }
                        Err(err)
                    }
                }
            }

            fn or_exit(self, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(_) => std::process::exit(code),
                }
            }

            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{}\nCaused by: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

        impl<T, E: std::fmt::Display> ResExt<T> for Result<T, E>
        where
            $enum: From<E>,
        {
            fn context(self, msg: &str) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: msg.as_bytes().to_vec(), source: err.into() }),
                }
            }

            fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: f().as_bytes().to_vec(), source: err.into() }),
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: bytes.to_vec(), source: err.into() }),
                }
            }

            fn or_exit(self, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(_) => std::process::exit(code),
                }
            }

            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{}\nCaused by: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }
    };
}
