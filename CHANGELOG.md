# Changelog

## v1.1.0 - 2026-02-17

### BREAKING CHANGES:

- Made `.context()` and `.with_context()` a single method
- Made the proc-macro not work without the main crate for better docs and util support in the future

---

## v1.0.2 - 2026-02-16

## Fixed

- `#[cfg(not(doc))]` making docs fail by changing it to `#[doc(hidden)]`

---

## v1.0.1 - 2026-02-13

### BREAKING CHANGES:

- Removed all methods from extension trait except for context traits (`.context()` and `.with_context()`)
- Removed `std` attribute as it is useless now

---

## v1.0.0 - 2026-02-13

### BREAKING CHANGES:

- Removed `.byte_context()` method
- Replaced feature flags for std and alloc to attributes in the macro

### Added

- Heap spilling if context exceeds `buf_size`
- New methods: `.fmt_log()` and `.write_log()`
- Full no-std support

---

## v0.9.0 - 2026-02-06

### BREAKING CHANGES:

- Switched to Zero-Allocation Inline Storage for error messages.
- Removed the deprecated declarative macro.

### Added

- Saturating Inline Storage: Replaced `Vec<u8>` with a stack-allocated buffer (ResBuf).
  - Controlled via the `buf_size` attribute (default: 64 bytes).
  - Guaranteed 0-allocation path for error context.
  - Built-in UTF-8 boundary safety to prevent invalid truncation except for the `unsafe` method `.byte_context()`
- Dynamic Context Optimization: `.with_context()` now accepts `core::fmt::Arguments<'_>` via `format_args!()`, avoiding heap-allocated `String`s during context creation.
- Dynamic Naming Logic: Item names (Err, Buf, and ResExt trait) are now generated based on the provided alias for better project organization and LSP support.
- Enhanced `.or_exit()`: The method now prints the error to Stderr before exiting, providing a clean "unwrap-like" method.

## Changed

- Internal Cleanup: Fully unified the codebase around the procedural macro engine.
Removed
- Declarative Macro: The old `macro_rules!` declarative implementation has been fully removed in favor of the more robust` #[resext]` attribute.

---

## v0.8.0 - 2026-01-28

### Added

- **Proc macro attribute** `#[resext]` for cleaner, more ergonomic syntax
- Custom formatting options:
  - `prefix` - String prepended to entire error message
  - `suffix` - String appended to entire error message
  - `msg_prefix` - String prepended to each context message
  - `msg_suffix` - String appended to each context message
  - `delimiter` - Separator between context messages (default: " - ")
  - `source_prefix` - String prepended to underlying error (default: "Error: ")
- `include_variant` option to show variant names in error display
- `alias` option for custom type alias names
- Named field support for enum variants (`Variant { error: Type }`)
- Comprehensive test suite with context chaining, display formatting, and error propagation tests

### Changed

- Improved error messages using `syn::Error` with proper span information
- Optimized memory allocation by pre-calculating required capacity
- Better documentation with examples for all major features

### Deprecated

- Declarative macro syntax (still available via `declarative` feature flag)

---

## v0.7.0 - 2026-01-18

### Summary

BREAKING CHANGES:
- Removed `ResExt` trait and `ErrCtx<E>` types.
- Added `ResExt! {}` macro for error-handling which generates all methods and types locally in your projects.

### Details

As it is clear from recent updates, there have been experiments with ResExt's API to rely more on a macro-based approach for generating error-enums instead of having the old trait-based API for ResExt which will provide great ergonomics without losing performance.

---

## v0.6.1 - 2025-12-28

### Summary

- Made `.byte_context()` an unsafe method.

### Details

- Made `.byte_context()` unsafe as it can lead to invalid UTF-8 context messages.
