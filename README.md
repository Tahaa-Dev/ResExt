## ResExt

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

### Overview of ResExt: Why should I use it?

There are a few design choices and highlights of ResExt that give it an advantage over `anyhow` and similar error handling crates, some of those being:

- **Zero allocation by default:** Static context uses `&'static str`, no heap allocation by default.
- **No custom error types:** Works with any `Result<T, E>` and has methods to convert to other context types, fully compatible with existing code.
- **Minimal bloat:** Zero dependencies, tiny compile times.
- **No type erasure:** Keep your error types concrete when you need them.
- **Pay only for what you use:** All methods that allocate are documented for their exact cost, and every one of them only allocates once.
- **No-std support coming:** Once main crate stabilizes.

### Quick Comparison

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

### How do I use ResExt? (Examples)

First, you have to add ResExt to your dependencies:

```bash
cargo add resext
```

- Or alternatively, add this to Cargo.toml:

```toml
[dependencies]
resext = "0.2.0"
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

fn load_config() -> CtxResult<Config, Error> {
    // Read file with context
    let content = std::fs::read_to_string("foo.toml")
        .context("Failed to read config file")?;

    // Parse with chained context
    let config: Config = toml::from_str(&content)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, "Invalid TOML in input file"))
        .context("Failed to deserialize TOML")
        .with_context(|| format!("Error in file: {}", "foo.toml"))?;

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
- Error in file: foo.toml
Caused by: InvalidInput

---

```
### Important notes

- ResExt is still in development and isn't stable, wait for v0.5.0 at least to use it.
- ResExt is licensed under the **MIT** license.
- ResExt doesn't have ergonomics as good as Anyhow's, as ResExt is meant for highly optimized, low-level production code.
