# resext-macro

**Procedural macro implementation for ResExt**

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
- `ResErr` wrapper struct with context storage
- `ResExt` trait with context methods
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
#[resext(
    source_prefix = "Caused by: "
)]
```

- `source_prefix` - Prepended to underlying error (default: "Error: ")

### Variant Display

```rust
#[resext(
    include_variant = true
)]
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

## Generated Code

For this input:

```rust
#[resext]
pub enum MyError {
    Io(std::io::Error),
}
```

The macro generates (simplified):

```rust
#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
}

impl Display for MyError { /* ... */ }
impl Error for MyError {}

pub struct ResErr {
    msg: Vec<u8>,
    pub source: MyError,
}

impl Display for ResErr { /* ... */ }
impl Debug for ResErr { /* ... */ }

impl From<std::io::Error> for MyError { /* ... */ }
impl From<std::io::Error> for ResErr { /* ... */ }

pub trait ResExt<T> {
    fn context(self, msg: &str) -> Result<T, ResErr>;
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr>;
    // ... other methods
}

impl<T> ResExt<T> for Result<T, ResErr> { /* ... */ }
impl<T, E> ResExt<T> for Result<T, E> where MyError: From<E> { /* ... */ }

pub type Res<T> = Result<T, ResErr>;
```

---

## Performance Characteristics

- Context storage uses `Vec<u8>` for string data
- Pre-calculates required capacity to minimize allocations
- Reuses buffer for multiple context calls
- Zero overhead when errors don't occur

---

## Implementation Details

The macro uses:

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

## Dependencies

- `syn` - Parsing
- `quote` - Code generation
- `proc-macro2` - Token manipulation

---

## License

MIT - See [LICENSE](../LICENSE) for details.

---

## See Also

- [resext](../resext/README.md) - Main crate documentation
- [ResExt repository](https://github.com/Tahaa-Dev/ResExt)

