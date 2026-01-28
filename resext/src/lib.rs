//! Context-rich error handling for Rust with zero-cost abstractions.
//!
//! ResExt provides ergonomic error handling with context chains, similar to anyhow
//! but with explicit error types. Choose between a proc macro for clean syntax and
//! custom formatting or
//! a declarative macro for zero dependencies.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext]
//! enum AppError {
//!     Io(std::io::Error),
//!     Parse(std::num::ParseIntError),
//! }
//!
//! fn read_config() -> Res<String> {
//!     let content = std::fs::read_to_string("config.toml")
//!         .context("Failed to read config file")?;
//!     
//!     let value: i32 = content.trim().parse()
//!         .context("Failed to parse config value")?;
//!     
//!     Ok(content)
//! }
//! ```
//!
//! ---
//!
//! # Features
//!
//! - **Type-safe errors** - Explicit error enums, no type erasure
//! - **Context chains** - Add context to errors as they propagate
//! - **Custom formatting** - Configure error display with attributes
//! - **Zero dependencies** - Provides declarative macro with no dependencies
//! - **Proc macro** - Clean syntax with `#[resext]` attribute (default)
//!
//! ---
//!
//! # Proc Macro (Default)
//!
//! The proc macro provides clean syntax with full customization:
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext(
//!     prefix = "ERROR: ",
//!     msg_delimiter = " -> ",
//!     include_variant = true,
//!     alias = AppResult
//! )]
//! enum MyError {
//!     Network(reqwest::Error),
//!     Database { error: sqlx::Error },
//! }
//! ```
//!
//! ## Attribute Options
//!
//! - `prefix` - String prepended to entire error message
//! - `suffix` - String appended to entire error message
//! - `msg_prefix` - String prepended to each context message
//! - `msg_suffix` - String appended to each context message
//! - `msg_delimiter` - Separator between context messages (default: " - ")
//! - `source_prefix` - String prepended to source error (default: "Error: ")
//! - `include_variant` - Include variant name in Display output (default: false)
//! - `alias` - Custom type alias name (default: `Res`)
//!
//! ---
//!
//! # Declarative Macro (Zero Dependencies)
//!
//! For projects that need minimal dependencies:
//!
//! ```toml
//! [dependencies]
//! resext = { version = "0.8", default-features = false, features = ["declarative"] }
//! ```
//!
//! ```rust,ignore
//! use resext::ResExt;
//!
//! ResExt! {
//!     enum MyError {
//!         Io(std::io::Error),
//!         Parse(std::num::ParseIntError),
//!     }
//! }
//! ```
//!
//! ---
//!
//! # Context Methods
//!
//! ## `.context(msg)`
//!
//! Add static context to an error:
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext] enum E { Io(std::io::Error) }
//!
//! std::fs::read("file.txt")
//!     .context("Failed to read file")?;
//! Ok::<(), ResErr>(())
//! ```
//!
//! ## `.with_context(|| msg)`
//!
//! Add dynamic context (computed only on error):
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext] enum E { Io(std::io::Error) }
//!
//! let path = "file.txt";
//! std::fs::read(path)
//!     .with_context(|| format!("Failed to read {}", path))?;
//! Ok::<(), ResErr>(())
//! ```
//!
//! ## `.or_exit(code)`
//!
//! Exit process with given code on error:
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext] enum E { Io(std::io::Error) }
//!
//! fn load_config() -> Res<()> { Ok(()) }
//!
//! let config = load_config().or_exit(1);
//! ```
//!
//! ## `.better_expect(|| msg, code)`
//!
//! Like `or_exit` but with custom message:
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext] enum E { Io(std::io::Error) }
//!
//! fn load_critical_data() -> Res<()> { Ok(()) }
//!
//! let data = load_critical_data()
//!     .better_expect(|| "FATAL: Cannot start without data", 1);
//! ```
//!
//! ---
//!
//! # Error Display Format
//!
//! Errors are displayed with context chains:
//!
//! ```text
//! Failed to load application
//!  - Failed to read config file
//!  - Failed to open file
//! Error: No such file or directory
//! ```
//!
//! With `include_variant = true`:
//!
//! ```text
//! Failed to load application
//!  - Failed to read config file
//! Error: Io: No such file or directory
//! ```
//!
//! ---
//!
//! # Examples
//!
//! ## Basic Error Handling
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext]
//! enum ConfigError {
//!     Io(std::io::Error),
//!     Parse(toml::de::Error),
//! }
//!
//! fn load_config(path: &str) -> Res<Config> {
//!     let content = std::fs::read_to_string(path)
//!         .context("Failed to read config")?;
//!     
//!     toml::from_str(&content)
//!         .with_context(|| format!("Failed to parse {}", path))
//! }
//! ```
//!
//! ## Multiple Error Types
//!
//! ```rust,ignore
//! use resext::resext;
//!
//! #[resext(alias = ApiResult)]
//! enum ApiError {
//!     Network(reqwest::Error),
//!     Database(sqlx::Error),
//!     Json(serde_json::Error),
//! }
//!
//! async fn fetch_user(id: u64) -> ApiResult<User> {
//!     let response = reqwest::get(format!("/users/{}", id))
//!         .await
//!         .context("Failed to fetch user")?;
//!     
//!     let user = response.json()
//!         .await
//!         .context("Failed to parse user data")?;
//!     
//!     Ok(user)
//! }
//! ```
//!
//! ---
//!
//! # Migration from v0.7.0
//!
//! Old declarative syntax:
//!
//! ```rust,ignore
//! ResExt! {
//!     enum MyError {
//!         Io(std::io::Error),
//!     }
//! }
//! ```
//!
//! New proc macro syntax:
//!
//! ```rust,ignore
//! #[resext]
//! enum MyError {
//!     Io(std::io::Error),
//! }
//! ```
//!
//! To keep using the declarative macro:
//!
//! ```toml
//! [dependencies]
//! resext = { version = "0.8", default-features = false, features = ["declarative"] }
//! ```

#[cfg(feature = "declarative")]
mod enum_macro;
mod throw_err;

#[cfg(feature = "proc-macro")]
pub use resext_macro::resext;
