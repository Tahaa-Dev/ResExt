<h1 align="center">ResExt</h1>

[<img alt="crates.io" src="https://img.shields.io/crates/v/resext.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/resext)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-resext-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/resext)
[![CI](https://github.com/Tahaa-Dev/ResExt/workflows/CI/badge.svg)](https://github.com/Tahaa-Dev/ResExt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<div style="display:flex; justify-content:center"><div style="width:90%; height:2px;background-color:#5e5e5e; border-radius:1px;"/></div>

***Simple, lightweight, low-alloc error handling crate for Rust.***

ResExt provides a declarative macro (`ResExt! {}`) for easy, ergonomic and performant error-handling.

---

## Quick Start

Add ResExt to your dependencies:

```sh
cargo add resext
```

- or add this to your Cargo.toml:

```toml
[dependencies]
resext = "0.7.0"
```

### Basic Usage

```rust
use resext::ResExt;

ResExt! {
    enum MyError {
        Io(std::io::Error),
        Env(std::env::VarError),
        Network(reqwest::Error),
        Other,
    }
}
```

This code generates:

1. `MyError` enum with `Error`, `Debug` and `Display` implementations
2. `From<T> for MyError` for types of the enum's tuple variants
3. `ResErr` struct for context wrapper around `MyError`
4. `ResExt` trait with context methods
5. `Res<T>` type alias for `Result<T, MyError>`

---

## Example

```rust
use resext::ResExt;
use std::io;
use crate::Data;

ResExt! {
    enum ErrorTypes {
        Io(io::Error),
        Network(reqwest::Error),
        Other,
    }
}

async fn parse_page(url: String) -> Res<Data> {
    let content = reqwest::get(url).await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;

    let data: Data = crate::parse_page_content(content)
        .context("Failed to parse page data")?;

    Ok(data)
}
```


ResExt uses a unique approach for error-handling that provides `anyhow` methods and convenient error-propagation as well as `thiserror`'s performance and concrete types with a macro that generates both methods and the error enum locally in your project.

---
## Notes

- ResExt is still in heavy development, wait for v1.0.0 to rely on it for production code.
- For changes, see <a href="CHANGELOG.md">CHANGELOG.md</a>.
- For contribution details, see <a href="CONTRIBUTING.md">CONTRIBUTING.md</a>.
- This project is licensed under the <a href="LICENSE">MIT license</a>.
