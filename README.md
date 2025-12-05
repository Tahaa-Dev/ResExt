## ResExt

**Simple, lightweight, zero-cost error handling extensions for Rust.**

ResExt provides ergonomic methods for `Result<T, E>` and `Option<T>` without the overhead of type erasure or boxing. Add context to errors, handle failures gracefully, and maintain zero-allocation paths for static messages.

---
#### Overview of ResExt: Why should I use it?

There are a few design choices and highlights of ResExt that give it an advantage over `anyhow` and similar error handling crates, some of those being:

- **Zero allocation by default:** Static context uses `&'static str`, no heap allocation by default.
- **No custom error types:** Works with any `Result<T, E>` and has methods to convert to other context types, fully compatible with existing code
- **Minimal bloat:** Zero dependencies, tiny compile times.
- **No type erasure:** Keep your error types concrete when you need them.
- **Pay only for what you use:** Dynamic context via `.with_context()` and other dynamic error methods only allocate when you call any of them.
- **No-std support coming:** Once main crate stabilizes.

###### Quick Comparison

| Feature | `anyhow` | `resext` |
|---------|----------|----------|
| Zero-alloc static context | ❌ | ✅ |
| Dynamic context | ✅ | ✅ |
| Type erasure | ✅ (always) | ❌ (opt-in) |
| Dependencies | 0 | 0 |
| No-std support | ❌ | 🚧 Coming |
| Context chaining | ✅ | ✅ |

---
#### How do I use ResExt? (Examples)

First, you have to add ResExt to your dependencies:

```bash
cargo add resext
```

- Or alternatively, add this to Cargo.toml:

```toml
[dependencies]
resext = "0.1.0"
```

Then, import the crate at the top of the file:

```rust
// To get the full crate with all traits and the Ctx<E> struct
use resext::*;

// Or to get just a particular trait
use resext::ResExt; // for Result handling
use resext::OptExt; // for Option handling
use resext::CtxChain; // trait for chaining context

// the Ctx<E> struct for context with all its methods 
use resext::Ctx;
```

Then use the new methods! Example code:

```rust
use resext::*;

fn load_config(path: &str) -> Result<Config, Ctx<std::io::Error>> {
    // Read file with context
    let content = std::fs::read_to_string(path)
        .context("Failed to read config file")?;

    // Parse with chained context
    let config: Config = toml::from_str(&content)
        .map_err(|e| std::io::Error::other(e))
        .context("Failed to deserialize TOML")
        .with_context(|_| format!("Error in file: {}", path))?;

    Ok(config)
}
```

---
#### Main methods


| Method                 | Functionality                                                                                                                                                                        | Allocates?                                                                                               |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `or_exit`              | Exits with provided code on error                                                                                                                                                    | No❌                                                                                                      |
| `better_expect`        | Exits with provided code, prints a message and optionally the original error if verbose is true on error                                                                             | No❌                                                                                                      |
| `dyn_expect`           | Does the same as `better_expect` but takes a closure that evaluates to a `String` as a message for dynamic errors                                                                    | Yes✅                                                                                                     |
| `or_default_context`   | Returns the provided default value and prints a static error message with optionally the original error if verbose is true on error                                                  | No❌                                                                                                      |
| `with_default_context` | Does the same as `or_default_context` but takes a closure that evaluates to `String` as a message for dynamic context                                                                | Yes✅                                                                                                     |
| `context`              | Wraps error into struct `Ctx<E>` which adds a message to the error to make it richer.                                                                                                | Once only in a Chain (allocates the first time it's called on a `Result<T,E>` as it creates a new Vec)⚠️ |
| `with_context`         | Does the same as `context` but takes a closure that evaluates to `String` as a message for dynamic context.                                                                          | Yes✅                                                                                                     |
| `map_err_into`         | Takes the error of the `Result<T,E>` and converts it into the usually inferred type if `E` supports `Into<E2>` where `E2` is the usually inferred type                               | No❌                                                                                                      |
| `to_option_context`    | Converts `Result<T,E>` into `Option<T>` and if `Result<T,E>` is an error returns `None` and prints the context message provided and optionally the original error if verbose is true | No❌                                                                                                      |
| `with_option_context`  | Does the same as `to_option_context` but takes a closure that evaluates to a `String` as a message for dynamic error messages.                                                       | Yes✅                                                                                                     |

###### For `Result<T, E>` (chaining):

- `.context(msg)`: Add another layer without nested types.
- `.with_context(closure)`: Add dynamic context to existing context.

**Note:** Context messages are displayed on separate lines, with the source error on the last line, example:

```rust
fn throw_error() -> Result<(), String> {
    Err("Threw error!".to_string())
}
fn main() {
    throw_error()
        .context("Error1")
        .context("Error2")?;
}
```

- Displays:

```
Error1
Error2: Threw error!
```

---
#### Important notes

- ResExt is still in development and isn't stable, wait for v1.0.0 to use it.
- Contributions are welcome from anyone, you may use this project however you like as it's licensed under the MIT license.
- The project is a stable build and doesn't require Nightly.
- These methods aren't all the crate will support, the API is still growing and I'll add more methods in the future.
