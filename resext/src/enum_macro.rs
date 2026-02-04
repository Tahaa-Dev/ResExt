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
    $vis:vis enum $name:ident $([$size:expr])? {
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

        $crate::__gen_res_buf!($($size)?);

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
            msg: ResBuf,
            $vis source: $name,
        }

        impl core::fmt::Write for ResErr {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                use core::fmt::Write;
                self.msg.write_str(s)
            }
        }

        $crate::__alias_helper!($vis $($alias)?);

        impl std::fmt::Display for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.curr_pos == 0 {
                    write!(f, "{}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nError: {}",
                        unsafe { std::str::from_utf8_unchecked(&self.msg.get_slice()) },
                        &self.source
                    )
                }
            }
        }

        impl std::fmt::Debug for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.curr_pos == 0 {
                    write!(f, "{:?}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nError: {:?}\n",
                        unsafe { std::str::from_utf8_unchecked(&self.msg.get_slice()) },
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
            $vis fn new<E>(msg: &[u8], source: E) -> Self where $name: From<E> {
                let mut buf = ResBuf::new();
                buf.extend_from_slice(msg);
                Self { msg: buf, source: $name::from(source) }
            }
        }

        impl From<$name> for ResErr {
            fn from(value: $name) -> Self {
                Self { msg: ResBuf::new(), source: value }
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
                        if err.msg.curr_pos == 0 {
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
                        if err.msg.curr_pos == 0 {
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
                        if err.msg.curr_pos == 0 {
                            err.msg.extend_from_slice(bytes);
                        } else {
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
                    Err(err) => {
                        let mut buf = ResBuf::new();
                        buf.extend_from_slice(msg.as_bytes());
                        Err(ResErr { msg: buf, source: err.into() })
                    }
                }
            }

            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => {
                        use core::fmt::Write;
                        let mut err_buf = ResErr::new(&[0u8], err);
                        let _ = write!(&mut err_buf, "{}", args);
                        Err(err_buf)
                    }
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => {
                        let mut buf = ResBuf::new();
                        buf.extend_from_slice(bytes);
                        Err(ResErr { msg: buf, source: err.into() })
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
                Self { msg: ResBuf::new(), source: $name::$variant(value) }
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

#[doc(hidden)]
#[macro_export]
macro_rules! __gen_res_buf {
    ($size:expr) => {
        struct ResBuf {
            curr_pos: u16,
            buf: [u8; $size],
        }

        impl ResBuf {
            fn new() -> Self {
                Self { buf: [0; $size], curr_pos: 0 }
            }

            fn as_str(&self) -> &str {
                unsafe { core::str::from_utf8_unchecked(&self.buf[..self.curr_pos as usize]) }
            }

            fn get_slice(&self) -> &[u8] {
                &self.buf[..self.curr_pos as usize]
            }

            fn extend_from_slice(&mut self, bytes: &[u8]) {
                let pos = self.curr_pos as usize;
                let cap = $size - pos;
                let to_copy = core::cmp::min(cap, bytes.len());

                self.buf[pos..pos + to_copy].copy_from_slice(&bytes[..to_copy]);
                self.curr_pos += to_copy as u16;
            }
        }

        impl core::fmt::Write for ResBuf {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                let bytes = s.as_bytes();
                let pos = self.curr_pos as usize;
                let cap = $size - pos;

                let to_copy = match bytes[..core::cmp::min(bytes.len(), cap)]
                    .iter()
                    .rposition(|&b| (b & 0xC0) != 0x80)
                {
                    Some(start_of_last_char) => {
                        let last_char_byte = bytes[start_of_last_char];
                        let width = match last_char_byte {
                            0..=127 => 1,
                            192..=223 => 2,
                            224..=239 => 3,
                            240..=247 => 4,
                            _ => 1,
                        };
                        if start_of_last_char + width <= bytes.len() {
                            start_of_last_char + width
                        } else {
                            start_of_last_char
                        }
                    }
                    None => 0,
                };

                self.buf[pos..pos + to_copy].copy_from_slice(&bytes[..to_copy]);
                self.curr_pos += to_copy as u16;

                Ok(())
            }
        }
    };

    () => {
        const SIZE: usize = 94;
        struct ResBuf {
            curr_pos: u16,
            buf: [u8; SIZE],
        }

        impl ResBuf {
            fn new() -> Self {
                Self { buf: [0; SIZE], curr_pos: 0 }
            }

            fn as_str(&self) -> &str {
                unsafe { core::str::from_utf8_unchecked(&self.buf[..self.curr_pos as usize]) }
            }

            fn get_slice(&self) -> &[u8] {
                &self.buf[..self.curr_pos as usize]
            }

            fn extend_from_slice(&mut self, bytes: &[u8]) {
                let pos = self.curr_pos as usize;
                let cap = SIZE - pos;
                let to_copy = core::cmp::min(cap, bytes.len());

                self.buf[pos..pos + to_copy].copy_from_slice(&bytes[..to_copy]);
                self.curr_pos += to_copy as u16;
            }
        }

        impl core::fmt::Write for ResBuf {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                let bytes = s.as_bytes();
                let pos = self.curr_pos as usize;
                let cap = SIZE - pos;

                let to_copy = match bytes[..core::cmp::min(bytes.len(), cap)]
                    .iter()
                    .rposition(|&b| (b & 0xC0) != 0x80)
                {
                    Some(start_of_last_char) => {
                        let last_char_byte = bytes[start_of_last_char];
                        let width = match last_char_byte {
                            0..=127 => 1,
                            192..=223 => 2,
                            224..=239 => 3,
                            240..=247 => 4,
                            _ => 1,
                        };
                        if start_of_last_char + width <= bytes.len() {
                            start_of_last_char + width
                        } else {
                            start_of_last_char
                        }
                    }
                    None => 0,
                };

                self.buf[pos..pos + to_copy].copy_from_slice(&bytes[..to_copy]);
                self.curr_pos += to_copy as u16;

                Ok(())
            }
        }
    };
}
