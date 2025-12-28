<h1 style="text-align: center;">ResExt</h1>
<div style="display:flex; justify-content:center;"><div style="width:40%; background-color:#5e5e5e; height:2px; border-radius:1px"/></div>

[<img alt="crates.io" src="https://img.shields.io/crates/v/resext.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/resext)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-resext-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/resext)

<div style="display:flex; justify-content:center"><div style="width:90%; height:2px;background-color:#5e5e5e; border-radius:1px;"/></div>

***Simple, lightweight, low-alloc\* error handling crate for Rust.***

ResExt provides ergonomic methods for `Result<T, E>` without the overhead of type erasure or boxing. Add context to errors, handle failures gracefully, and maintain zero-allocation paths for static messages.

\*Context chaining uses a single `Vec<u8>` allocation which will be optimized to be zero-alloc in later versions.

---

### Allocation Strategy

- **Static context:** Zero allocations (messages stored in binary)
- **Context chaining:** Single `Vec<u8>` for first context (all messages in one allocation)
- **Dynamic context:** Allocates only for the generated string

Compare to `anyhow`: boxes every error

---

## Overview of ResExt: Why should I use it?

There are a few design choices and highlights of ResExt that give it an advantage over `anyhow` and similar error handling crates, some of those being:

- **Zero allocation by default:** Static context uses `&'static str`, no heap allocation by default.
- **No custom error types:** Works with any `Result<T, E>` and has methods to get the raw source of errors, fully compatible with old code.
- **Minimal bloat:** Zero dependencies, tiny compile times.
- **No type erasure:** Keep your error types concrete when you need them.
- **Pay only for what you use:** All methods that allocate are documented for their exact cost, and every one of them only allocates once.
- **No-std support coming:** Once main crate stabilizes.

---

## Quick Comparison

| Feature                   | `anyhow`              | `resext`                                          |
| ------------------------- | ------------------- | ----------------------------------------------- |
| Zero-alloc static context | No                  | Yes (coming)                                    |
| Dynamic context           | Yes                 | Yes                                             |
| Type erasure              | Yes (always)        | No, zero boxing overhead, zero extra allocation |
| Dependencies              | 1                   | 0                                               |
| No-std support            | Yes, but uses alloc | Coming without alloc                            |
| Context chaining          | Yes                 | Yes                                             |
| Allocations per error     | 2+ (box and context)  | 0-1 (single `Vec<u8>` only if context)          |

---

## How do I use ResExt? (Examples)

First, you have to add ResExt to your dependencies:

```bash
cargo add resext
```

- Or alternatively, add this to Cargo.toml:

```toml
[dependencies]
resext = "0.6.1"
```

Then, import the crate at the top of the file:

```rust
// To get the full crate with all traits and the Ctx<E> struct
use resext::*;

// Or get a particular part of ResExt
use resext::ResExt; // for Result handling

// the Ctx<E> struct for context with all its methods 
use resext::ErrCtx;

// CtxResult<T, E> type alias for Result<T, ErrCtx<E>>
use resext::CtxResult;

// Macro for throwing errors on a condition
use resext::throw_err_if;
```

- Then use the new methods! Example code:

```rust
use resext::*;
use std::io::{Error, ErrorKind};
use toml::Value;

fn load_config() -> CtxResult<Value, Error> {
    let path = "config.toml";
    // Read file with context
    let content = std::fs::read_to_string(path)
        .context("Failed to read config file")?;

    // Parse with chained context
    let config: Value = toml::from_str(&content)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, "Invalid TOML in input file"))
        .context("Failed to deserialize TOML")
        .with_context(|| format!("Error in file: {}", path))?;

    throw_err_if!(config.is_array(), || format!("Invalid input for config in file: {}.\n Config cannot be an array."), 1);

    Ok(config)
}
```

- Which on error prints out (for the first `Result`):

```
Failed to read config file
Caused by: NotFound
```

- Or for the second `Result`:

```
Failed to deserialize TOML
- Error in file: config.toml
Caused by: InvalidInput
```

---

## Important notes

- ResExt is still in development and isn't stable, wait for v1.0.0 to rely on it for production code.
- ResExt is licensed under the **MIT** license.
- ResExt doesn't have ergonomics as good as anyhow's, as ResExt is meant for highly optimized, low-level production code, not ergonomics in exchange for performance.
- For contribution details, see <a href="CONTRIBUTING.md">CONTRIBUTING.md</a>.
- For the changelog, see <a href="CHANGELOG.md">CHANGELOG.md</a>.
