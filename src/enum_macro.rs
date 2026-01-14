/// Generates an enum with automatic trait implementations.
///
/// This macro creates an enum and implements:
/// - `From<T>` for each wrapped variant type (auto-converts to `ErrCtx<Enum>`)
/// - `Display` trait (delegates to inner values or uses variant name)
/// - `Error` trait (makes it usable with `?` operator)
/// - `Res<T>` type alias (shorthand for `CtxResult<T, Enum>`)
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,ignore
/// use resext::enumerate;
///
/// enumerate! {
///     pub enum MyError {
///         Io(std::io::Error),
///         Parse(String),
///         Timeout,
///     }
/// }
///
/// fn read_file() -> Res<String> {
///     // io::Error automatically converts to MyError::Io
///     let data = std::fs::read_to_string("file.txt")?;
///     Ok(data)
/// }
/// ```
///
/// ## With Context Chains
///
/// ```rust,ignore
/// use resext::{enumerate, ResExt};
///
/// enumerate! {
///     enum AppError {
///         Io(std::io::Error),
///         Network(String),
///     }
/// }
///
/// fn fetch_config() -> Res<Config> {
///     let data = std::fs::read("config.json")
///         .context("Failed to read config")?;
///
///     let config = parse_config(&data)
///         .map_err(|e| AppError::Network(e.to_string()))?
///         .context("Failed to parse config")?;
///
///     Ok(config)
/// }
/// ```
///
/// ## Pattern Matching on Errors
///
/// ```rust,ignore
/// use resext::enumerate;
///
/// enumerate! {
///     enum MyError {
///         Io(std::io::Error),
///         Timeout,
///     }
/// }
///
/// match some_operation() {
///     Ok(val) => println!("Success: {}", val),
///     Err(e) => match e.error() {
///         MyError::Io(io_err) => eprintln!("IO error: {}", io_err),
///         MyError::Timeout => eprintln!("Operation timed out"),
///     }
/// }
/// ```
///
/// ## With Custom Alias
///
/// ```rust,ignore
/// use resext::enumerate
///
/// enumerate! {
///     enum ErrorTypes {
///         Io(std::io::Error),
///         Parse(serde_json::Error),
///     } as MyResult
/// }
///
/// fn parse_logs() -> MyResult<serde_json::Value> {
///     let content = std::fs::read("log.json")?;
///
///     let parsed_data = serde_json::from_slice(&content)
///         .context("Failed to parse logs")?;
///
///     Ok(parsed_data)
/// }
/// ```
///
/// `enumerate! {}` generated a custom alias (`MyResult<T>`) instead of `Res<T>`
///
/// ## Supported Variants
///
/// - **Wrapped variants**: Must have exactly one unnamed field
///   ```rust
///   Io(std::io::Error)  // ✓ Generates From impl
///   Parse(String)       // ✓ Generates From impl
///   ```
///
/// - **Unit variants**: No fields, used for custom error cases
///   ```rust
///   Timeout             // ✓ No From impl
///   InvalidInput        // ✓ No From impl
///   ```
///
/// - **Named fields**: Not supported, will be supported in v0.8.0 with a proc macro
///   ```rust
///   Custom { code: i32 }  // ✗ Will not compile
///   ```
///
/// ## Generated Code
///
/// For this enum:
/// ```rust
/// use resext::enumerate;
///
/// enumerate! {
///     pub enum MyError {
///         Io(std::io::Error),
///         Timeout,
///     }
/// }
/// ```
///
/// The macro generates:
/// ```rust,ignore
/// #[derive(Debug)]
/// pub enum MyError {
///     Io(std::io::Error),
///     Timeout,
/// }
///
/// impl From<std::io::Error> for ErrCtx<MyError> { ... }
/// impl std::fmt::Display for MyError { ... }
/// impl std::error::Error for MyError {}
/// pub type Res<T> = CtxResult<T, MyError>;
/// ```
#[macro_export]
macro_rules! enumerate {
    (
    $(#[$meta:meta])*
    $vis:vis enum $name:ident {
        $($variant:ident $(($type:ty))?),* $(,)?
    }
    $(as $alias:ident)?) => {
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

        __alias_helper!($vis, $name $($alias)?);
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

#[doc(hidden)]
#[macro_export]
macro_rules! __alias_helper {
    ($vis:vis, $name:ident $alias:ident) => {
        $vis type $alias<T> = $crate::CtxResult<T, $name>;
    };

    ($vis:vis, $name:ident) => {
        $vis type Res<T> = $crate::CtxResult<T, $name>;
    };
}
