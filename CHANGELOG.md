
# Changelog

## v0.6.1 - 2025-12-28

### Summary

- Removed `DynResult<T>` type alias (didn't match ResExt's design goals)
- Updated docs and tests accordingly
- Updated README.md
- Added CHANGELOG.md (this file)

### Details

After experimentation, we've decided ResExt should focus on explicit error 
types and performance rather than anyhow-like ergonomics. Users who need 
dynamic error handling should use anyhow instead.
