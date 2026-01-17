//! # ResExt - Low-Allocation Error Handling for Rust
//!
//! ResExt provides a declarative macro for generating complete error handling
//! infrastructure with minimal runtime overhead and zero external dependencies.
//!
//! ## Features
//!
//! - **Zero allocations** for error construction (allocations only on context chains)
//! - **Local code generation** - no orphan rule violations
//! - **Explicit error types** - know exactly what can fail
//! - **Context chains** - add debugging information without losing the root cause
//! - **Single macro** - everything you need in one invocation
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use resext::ResExt;
//!
//! ResExt! {
//!     pub enum MyError {
//!         Io(std::io::Error),
//!         Parse(serde_json::Error),
//!         Network(reqwest::Error),
//!         NotFound,
//!     }
//! }
//!
//! fn read_config(path: &str) -> Res<Config> {
//!     let content = std::fs::read_to_string(path)
//!         .context("Failed to read config file")?;
//!
//!     let config = serde_json::from_str(&content)
//!         .with_context(|| format!("Failed to parse config from: {}", path))?;
//!
//!     Ok(config)
//! }
//! ```
//!
//! ## What Gets Generated
//!
//! The `ResExt!` macro generates:
//!
//! 1. Your error enum with `Display`, `Debug`, and `Error` trait implementations
//! 2. `ResErr` struct that wraps your enum with context messages
//! 3. `From` implementations for automatic error conversion
//! 4. `ResExt` trait with context methods (`.context()`, `.with_context()`, etc.)
//! 5. `Res<T>` type alias (or custom alias via `as YourAlias`)
//!
//! All code is generated **locally in your crate**, avoiding orphan rule issues.
//!
//! ## Design Philosophy
//!
//! ResExt prioritizes:
//! 1. **Performance** - minimal allocations
//! 2. **Explicitness** - you declare exactly what errors can occur
//! 3. **Debuggability** - context chains show the full error path
//! 4. **Simplicity** - one macro, zero configuration
//!
//! Ergonomics are important but secondary to the above goals.
//!
//! ## Examples
//!
//! ### Basic Error Handling
//!
//! ```rust,ignore
//! use resext::ResExt;
//!
//! ResExt! {
//!     enum FileError {
//!         Io(std::io::Error),
//!         InvalidFormat,
//!     }
//! }
//!
//! fn process_file(path: &str) -> Res<Vec<u8>> {
//!     std::fs::read(path).context("Failed to read file")
//! }
//! ```
//!
//! ### Adding Dynamic Context
//!
//! ```rust,ignore
//! use resext::ResExt;
//! ResExt! {
//!     enum MyError {
//!         Io(std::io::Error),
//!     }
//! }
//! fn read_multiple_files(paths: &[&str]) -> Res<Vec<Vec<u8>>> {
//!     paths.iter()
//!         .map(|path| {
//!             std::fs::read(path)
//!                 .with_context(|| format!("Failed to read: {}", path))
//!         })
//!         .collect()
//! }
//! ```
//!
//! ### Custom Type Alias
//!
//! ```rust,ignore
//! use resext::ResExt;
//!
//! ResExt! {
//!     enum AppError {
//!         Config(std::io::Error),
//!         Database(String),
//!     }
//!     as AppResult  // Custom alias instead of Res
//! }
//!
//! fn load_config() -> AppResult<Config> {
//!     // ...
//! #   Ok(Config {})
//! }
//! # struct Config {}
//! ```
//!
//! ### Exit on Error (CLI Tools)
//!
//! ```rust,ignore
//! use resext::ResExt;
//! ResExt! {
//!     enum IoError {
//!         Io(std::io::Error),
//!     }
//! }
//!
//! fn main() {
//!     let config = std::fs::read_to_string("config.toml")
//!         .context("Failed to load config")
//!         .or_exit(1);  // Exit with code 1 on error
//!
//!     // Or with custom message:
//!     let data = std::fs::read("data.bin")
//!         .better_expect(|| "Critical: Cannot start without data.bin", 1);
//! }
//! ```
//!
//! ## Performance
//!
//! ResExt is designed for minimal overhead:
//!
//! - **Error construction:** Zero allocations (just wrapping the inner error)
//! - **Context chains:** One allocation per `.context()` call (reuses existing buffer)
//! - **Error propagation:** Zero-cost (same as `?` operator)
//!
//! Benchmarks (compared to plain `Result<T, E>`):
//! - Error construction: ~0ns overhead
//! - Single context: ~50ns (one allocation)
//! - Context chain (5 deep): ~150ns (buffer reuse)

mod enum_macro;
mod throw_err;
