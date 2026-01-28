<h1 align="center">ResExt</h1>

[<img alt="crates.io" src="https://img.shields.io/crates/v/resext.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/resext)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-resext-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/resext)
[![CI](https://github.com/Tahaa-Dev/ResExt/workflows/CI/badge.svg)](https://github.com/Tahaa-Dev/ResExt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

ResExt provides ergonomic error handling with context chains, similar to anyhow but with explicit error types. Choose between a proc macro for clean syntax and custom formatting or a declarative macro for zero dependencies.

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
- **Zero dependencies** - Provides declarative macro with no dependencies
- **Proc macro** - Clean syntax with `#[resext]` attribute (default)

---

## Customization

```rust
#[resext(
    prefix = "ERROR: ",
    msg_delimiter = " -> ",
    include_variant = true,
    alias = AppResult
)]
enum MyError {
    Network(reqwest::Error),
    Database(sqlx::Error),
}
```

---

## Feature Flags

- `proc-macro` (default) - Use proc macro for clean syntax and custom formatting
- `declarative` - Use declarative macro for zero dependencies

---

## Documentation

- [Main crate documentation](resext/README.md)
- [Proc macro documentation](resext-macro/README.md)

---

## License

**MIT** license - See [LICENSE](LICENSE) for details.

---

## Links

- [Crates.io](https://crates.io/crates/resext)
- [Documentation](https://docs.rs/resext)
- [Repository](https://github.com/Tahaa-Dev/ResExt)
- [Changelog](CHANGELOG.md)
