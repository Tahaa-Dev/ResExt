<h1 align="center">ResExt</h1>

[<img alt="crates.io" src="https://img.shields.io/crates/v/resext.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/resext)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-resext-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/resext)
[![CI](https://github.com/Tahaa-Dev/ResExt/workflows/CI/badge.svg)](https://github.com/Tahaa-Dev/ResExt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

ResExt provides ergonomic error-handling similar to anyhow, but it uses explicit types with `From<E>` trait `impl`s, zero-alloc for every use case and inline byte buffers for context storage.

---

## Quick Start

Run:

```bash
cargo add resext
```

- Or add to your `Cargo.toml`:

```toml
[dependencies]
resext = "0.8"
```

---

## Usage

```rust
use resext::resext;

#[resext]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

fn read_config() -> Res<String> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let value: i32 = content.trim().parse()
        .context("Failed to parse config value")?;
    
    Ok(content)
}
```

## Features

- **Type-safe errors** - Explicit error enums, no type erasure
- **Context chains** - Add context to errors as they propagate
- **Custom formatting** - Configure error display with attributes
- **Easy error-handling** - Ergonomic, anyhow-like error-handling for seamless `?` usage for error-propagation
- **Zero-alloc** - ResExt is 100% allocation free for restricted environments with inline arrays for context buffers and no boxing (type erasure)

---

## Customization

```rust
#[resext(
    prefix = "ERROR: ",
    delimiter = " -> ",
    include_variant = true,
    alias = AppResult
)]
enum MyError {
    Network(reqwest::Error),
    Database(sqlx::Error),
}
```

---

## Documentation

- [Main crate README.md](resext/README.md)
- [Proc macro README.md](resext-macro/README.md)
- [Main crate Documentation](https://docs.rs/resext)
- [Proc macro Documentation](https://docs.rs/resext-macro)

---

## License

**MIT** license - See [LICENSE](LICENSE) for details.

---

## Links

- [Crates.io](https://crates.io/crates/resext)
- [Documentation](https://docs.rs/resext)
- [Repository](https://github.com/Tahaa-Dev/ResExt)
- [Changelog](CHANGELOG.md)
