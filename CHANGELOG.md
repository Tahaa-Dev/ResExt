# Changelog

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
