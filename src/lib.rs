/*!
`resext` provides simple, lightweight, low-cost error-handling methods in Rust without losing ergonomics.

The main points of this crate are:

### 1. `ResExt`: Error-handling trait

A trait that provides methods for error-handling which is implemented for `Result<T, E>`, so you can just import it in your project like this:

```rust,ignore
use resext::ResExt;
```

- and use all the methods it provides for `Result<T, E>`!

### 2. `ErrCtx<E>`: Custom error struct

This is the main struct ResExt uses to add context to errors.

It stores two fields:

1. `msg: Vec<u8>` which is the field that stores all context messages in the chain in raw bytes for more optimized memory usage than storing actual string messages.
2. `source: E where E: std::error::Error` which stores the source that the context originated from (e.g. `std::io::Error::other("")`).

### Example

```rust,ignore
use resext::*;
use std::io::{Error, ErrorKind::InvalidInput};
use serde_json::Value;

fn process_logs() -> CtxResult<Value, Error> {
    let path: &str = "log.json";

    let content = std::fs::read_to_string(path)
        .context("Failed to process logs.")
        .with_context(|| format!("Failed to read logs file: {}.", path))?;

    let logs: Value = serde_json::from_str::<Value>(&content)
        .map_err(|_| Error::new(InvalidInput, "Invalid JSON input in file."))
        .context("Failed to parse logs for processing.")
        .with_context(|| format!("Invalid JSON in logs file: {}.", path))?;

    logs
}
```

- Output for first `Result` on error:

```text
Failed to process logs.
- Failed to read logs file: log.json.
Caused by: NotFound
```

- Output for second `Result` on error:

```text
Failed to parse logs for processing.
- Invalid JSON in logs file: log.json.
Caused by: InvalidInput
```

You can chain more context of course, this is just an example.

---

This crate is licensed under the MIT license.
*/

mod ctx;
mod enum_macro;
mod res_ext;
mod res_ext_methods;
mod throw_err;

pub use crate::ctx::ErrCtx;
pub use res_ext::ResExt;

/// Type alias for `Result<T, ErrCtx<E>>`.
pub type CtxResult<T, E> = std::result::Result<T, ErrCtx<E>>;
