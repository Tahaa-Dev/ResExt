# Changelog

## v0.8.0 - 2026-01-28

### Added

- **Proc macro attribute** `#[resext]` for cleaner, more ergonomic syntax
- Custom formatting options:
  - `prefix` - String prepended to entire error message
  - `suffix` - String appended to entire error message
  - `msg_prefix` - String prepended to each context message
  - `msg_suffix` - String appended to each context message
  - `msg_delimiter` - Separator between context messages (default: " - ")
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
