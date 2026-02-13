# resext-macro

**Procedural macro for ResExt**

This crate provides the `#[resext]` attribute macro for ergonomic error handling. It is not meant to be used directly - use the `resext` crate instead.

---

## Overview

The proc macro generates all necessary error handling code from a simple attribute:

```rust
#[resext]
enum MyError {
    Io(std::io::Error),
    Network(reqwest::Error),
}
```

This expands to approximately 200 lines of boilerplate including:

- `Display` and `Error` trait implementations
-  Wrapper struct with inline zero-alloc context storage
-  Trait with context methods
- `From` implementations for automatic conversion
- Type alias for `Result<T, ResErr>`

---

## Attribute Options

### Basic Options

```rust
#[resext(
    prefix = "ERROR: ",
    suffix = "\n"
)]
```

- `prefix` - Prepended to all error messages
- `suffix` - Appended to all error messages

### Context Formatting

```rust
#[resext(
    msg_prefix = "  ",
    msg_suffix = "",
    delimiter = "\n"
)]
```

- `msg_prefix` - Prepended to each context message
- `msg_suffix` - Appended to each context message
- `delimiter` - Separator between contexts (default: " - ")

### Source Formatting

```rust
#[resext(source_prefix = "Caused by: ")]
```

- `source_prefix` - Prepended to underlying error (default: "Error: ")

### Variant Display

```rust
#[resext(include_variant = true)]
enum MyError {
    Io(std::io::Error),
    Network(reqwest::Error),
}
```

With `include_variant = true`, errors display as:

```text
Context
Error: Io: No such file or directory
```

Without it (default):

```text
Context
Error: No such file or directory
```

### Custom Type Alias

```rust
#[resext(alias = AppResult)]
enum AppError {
    // ...
}

// Generated:
pub type AppResult<T> = Result<T, ResErr>;
```

### Context storage inline buffer size

```rust
#[resext(buf_size = 72)]
```

- `buf_size` sets the size for the inline context storage byte buffer, so with the attribute above, you get 72 bytes of context messages (including delimiters, so it's more accurately `buf_size - (number_of_messages - 1) * len_of_delimiter`)

### Heap support

```rust
#[resext(alloc = true)]
```

- `alloc` adds heap spilling if context exceeds `buf_size`

---

## Supported Variant Types

### Tuple Variants (Single Field)

```rust
#[resext]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}
```

### Named Fields (Single Field)

```rust
#[resext]
enum MyError {
    Io { error: std::io::Error },
    Network { error: reqwest::Error },
}
```

### Unit Variants

```rust
#[resext]
enum MyError {
    Io(std::io::Error),
    NotFound,  // Unit variant
}
```

Displays as the variant name: `NotFound`

---

## Limitations

- Variants must have at most one field
- Multi-field variants are not supported
- Generic enums are not supported

---

## Performance Characteristics

- Context storage uses `[u8; BUF_SIZE]` for string data
- Zero-alloc even on errors
- Only allocates if alloc attribute is true which is optional

---

## Implementation Details

1. The macro uses:

- `syn` for parsing Rust syntax
- `quote` for code generation
- `proc-macro2` for token manipulation

---

## Error Messages

The macro provides helpful error messages:

```rust
#[resext]
enum MyError {
    Multi(std::io::Error, String),  // Error
}
```

```text
error: enum variants used in `#[resext]` can only have 1 field
 --> src/main.rs:X:Y
  |
X |     Multi(std::io::Error, String),
  |          ^^^^^^^^^^^^^^^^^^^^^^^^
```

---

## License

MIT - See [LICENSE](../LICENSE) for details.

---

## See Also

- [resext](../resext/README.md) - Main crate documentation
- [ResExt repository](https://github.com/Tahaa-Dev/ResExt)
