/// Generate a complete error handling setup with context chain support.
///
/// This macro generates all the types and traits needed for error handling
/// with context chains, similar to anyhow but with explicit error types.
///
/// # What Gets Generated
///
/// 1. Your error enum with `Display`, `Debug`, and `Error` implementations
/// 2. `ResErr` struct that wraps your enum with context messages
/// 3. `From` implementations for automatic error conversion
/// 4. `ResExt` trait with context methods
/// 5. `Res<T>` type alias (or custom alias)
///
/// # Syntax
///
/// ```rust,ignore
/// ResExt! {
///     #[derive(Clone)]  // Optional attributes
///     pub enum ErrorName {
///         Variant(ErrorType),  // Wrapped error type
///         AnotherVariant,      // Unit variant
///     }
///     as CustomAlias  // Optional custom type alias
/// }
/// ```
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,ignore
/// use resext::ResExt;
///
/// ResExt! {
///     pub enum MyError {
///         Io(std::io::Error),
///         Parse(std::num::ParseIntError),
///     }
/// }
///
/// fn example() -> Res<i32> {
///     let content = std::fs::read_to_string("number.txt")
///         .context("Failed to read file")?;
///
///     let number = content.trim().parse()
///         .context("Failed to parse number")?;
///
///     Ok(number)
/// }
/// ```
///
/// ## Custom Type Alias
///
/// ```rust,ignore
/// use resext::ResExt;
///
/// ResExt! {
///     enum AppError {
///         Config(std::io::Error),
///     }
///     as AppResult
/// }
///
/// fn load() -> AppResult<()> {
///     Ok(())
/// }
/// ```
///
/// # Performance
///
/// - Error construction: Zero allocations
/// - First `.context()`: One allocation for the message buffer
/// - Subsequent `.context()`: Reuses existing buffer
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

        /// Wrapper type that holds your error with optional context messages.
        ///
        /// This type is automatically created when you use `.context()` or
        /// `.with_context()` on a Result.
        ///
        /// # Display Format
        ///
        /// When displayed, shows context messages like this:
        ///
        /// ```text
        /// Failed to process request
        /// - Failed to load user data
        /// Error: Connection refused
        /// ```
        #[doc(hidden)]
        $vis struct ResErr {
            msg: Vec<u8>,
            $vis source: $name,
        }

        impl core::fmt::Write for ResErr {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.msg.extend_from_slice(s.as_bytes());
                Ok(())
            }
        }

        $crate::__alias_helper!($vis $($alias)?);

        impl std::fmt::Display for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.is_empty() {
                    write!(f, "{}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nError: {}",
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
                        "{}\nError: {:?}\n",
                        unsafe { std::str::from_utf8_unchecked(&self.msg) },
                        &self.source
                    )
                }
            }
        }

        impl ResErr {
            /// Helper method for constructing ResErr without using `.context()` or
            /// `.with_context()` on a Result.
            ///
            /// This method:
            /// ```rust,ignore
            /// ResErr::new(b"Failed to read file".to_vec(), std::io::Error::other(""));
            /// ```
            ///
            /// - is the same as:
            /// ```rust,ignore
            /// ResErr { b"Failed to read file".to_vec(),
            /// ErrorEnum::Io(std::io::Error::other("")) }
            /// ```
            $vis fn new<E>(msg: Vec<u8>, source: E) -> Self where $name: From<E> {
                Self { msg: msg, source: $name::from(source) }
            }
        }

        impl From<$name> for ResErr {
            fn from(value: $name) -> Self {
                Self { msg: Vec::new(), source: value }
            }
        }


        /// Extension trait for adding context to Result types.
        ///
        /// Automatically implemented for all `Result<T, E>` where `E` can be
        /// converted into your error enum.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// std::fs::read("file.txt")
        ///     .context("Failed to read file")?;
        /// ```
        #[doc(hidden)]
        $vis trait ResExt<'r, T> {
            /// Add a static context message to an error.
            ///
            /// The message is only allocated if an error occurs.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// std::fs::read("config.toml")
            ///     .context("Failed to read config")?;
            /// ```
            #[doc(hidden)]
            fn context(self, msg: &str) -> Result<T, ResErr>;

            /// Add a dynamic context message (computed only on error).
            ///
            /// Use this when the context message needs runtime information.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// std::fs::read(path)
            ///     .with_context(|| format!("Failed to read: {}", path))?;
            /// ```
            #[doc(hidden)]
            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, ResErr>;

            /// Add raw bytes as context (must be valid UTF-8).
            ///
            /// # Safety
            ///
            /// The bytes must be valid UTF-8
            #[doc(hidden)]
            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr>;

            /// Exit the process with the given code if the result is an error.
            ///
            /// Useful for CLI applications that want to exit on critical errors.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// let config = load_config().or_exit(1);
            /// ```
            #[doc(hidden)]
            fn or_exit(self, code: i32) -> T;

            /// Like `or_exit` but prints a custom message before exiting.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// let data = load_critical_data()
            ///     .better_expect(|| "FATAL: Cannot start without data", 1);
            /// ```
            #[doc(hidden)]
            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T;
        }


        impl<'r, T> ResExt<'r, T> for Result<T, ResErr> {
            fn context(self, msg: &str) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        use core::fmt::Write;
                        if err.msg.is_empty() {
                            let _ = write!(&mut err, "{}", msg);
                        } else {
                            let _ = write!(&mut err, "\n - {}", msg);
                        }
                        Err(err)
                    }
                }
            }

            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        use core::fmt::Write;
                        if err.msg.is_empty() {
                            let _ = write!(&mut err, "{}", args);
                        } else {
                            let _ = write!(&mut err, "\n - {}", args);
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
                            let len = bytes.len();
                            let cap = err.msg.capacity();
                            if cap < len + 4 {
                                err.msg.reserve_exact((len + 4) - cap);
                            }
                            err.msg.extend_from_slice(b"\n - ");
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
                        eprintln!("{}\nError: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

        impl<'r, T, E: std::fmt::Display> ResExt<'r, T> for Result<T, E>
        where
            $name: From<E>,
        {
            fn context(self, msg: &str) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: msg.as_bytes().to_vec(), source: err.into() }),
                }
            }

            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => {
                        use core::fmt::Write;
                        let mut err_buf = ResErr::new(Vec::new(), err);
                        let _ = write!(&mut err_buf, "{}", args);
                        Err(err_buf)
                    }
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
                        eprintln!("{}\nError: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

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
    ($name:ident, $variant:ident) => {};

    ($name:ident, $variant:ident $type:ty) => {
        impl From<$type> for ResErr {
            fn from(value: $type) -> Self {
                Self { msg: Vec::new(), source: $name::$variant(value) }
            }
        }

        impl From<$type> for $name {
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
