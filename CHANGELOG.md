# Changelog

## v0.6.1 - 2025-12-28

### Summary

- Made `.byte_context()` an unsafe method.

### Details

- Made `.byte_context()` unsafe as it can lead to invalid UTF-8 context messages.
