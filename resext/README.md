# resext

**Main crate providing error handling with context chains**

This is the primary interface for ResExt. It re-exports either the proc macro or declarative macro based on feature flags.

---

## Installation

```toml
[dependencies]
resext = "0.8"
```

---

## Quick Example

```rust
use resext::resext;

#[resext]
enum FileError {
    Io(std::io::Error),
    Parse(serde_json::Error),
}

fn load_data(path: &str) -> Res<Data> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read file")?;
    
    let data = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path))?;
    
    Ok(data)
}
```

---

## Proc Macro (Default)

The proc macro provides clean syntax with full customization:

```rust
#[resext(
    prefix = "ERROR: ",
    suffix = "\n",
    msg_prefix = "  at: ",
    msg_suffix = "",
    msg_delimiter = "\n",
    source_prefix = "Caused by: ",
    include_variant = true,
    alias = MyResult
)]
enum MyError {
    Network(reqwest::Error),
    Database { error: sqlx::Error },
}
```

### Attribute Options

- `prefix` - String prepended to entire error message
- `suffix` - String appended to entire error message
- `msg_prefix` - String prepended to each context message
- `msg_suffix` - String appended to each context message
- `msg_delimiter` - Separator between context messages | default: " - " (NOTE: the delimiter always includes a newline before it, e.g. if delimiter = " - ", then messages will have "\n - " between them, not just " - ")
- `source_prefix` - String prepended to source error (default: "Error: ")
- `include_variant` - Include variant name in Display output (default: false)
- `alias` - Custom type alias name (default: `Res`)

### Generated Items

The macro generates:

1. `Display` and `Error` implementations for your enum
2. `ResErr` struct wrapping your enum with context
3. `ResExt` trait with context methods
4. `From` implementations for automatic conversion
5. Type alias for `Result<T, ResErr>`

---

## Declarative Macro (Zero Dependencies)

For projects that need minimal dependencies:

```toml
[dependencies]
resext = { version = "0.8", default-features = false, features = ["declarative"] }
```

```rust
use resext::ResExt;

ResExt! {
    enum MyError {
        Io(std::io::Error),
        Parse(std::num::ParseIntError),
    }
}
```

### Limitations of Declarative Macro

- No custom formatting options
- Less ergonomic syntax
- No named field support
- Limited customization

### Advantages of Declarative Macro

- Zero dependencies (no syn, quote, proc-macro2)
- Faster compile times
- Simpler implementation

---

## Context Methods

### `.context(msg)`

Add static context to an error:

```rust
std::fs::read("file.txt")
    .context("Failed to read file")?;
```

### `.with_context(|| msg)`

Add dynamic context (computed only on error):

```rust
std::fs::read(path)
    .with_context(|| format!("Failed to read {}", path))?;
```

### `.or_exit(code)`

Exit process with given code on error:

```rust
let config = load_config().or_exit(1);
```

### `.better_expect(|| msg, code)`

Like `or_exit` but with custom message:

```rust
let data = load_critical_data()
    .better_expect(|| "FATAL: Cannot start without data", 1);
```

---

## Error Display Format

Errors are displayed with context chains:

```
Failed to load application
 - Failed to read config file
 - Failed to open file
Error: No such file or directory
```

With `include_variant = true`:

```
Failed to load application
 - Failed to read config file
Error: Io: No such file or directory
```

---

## Comparison with Alternatives

### vs anyhow

- **ResExt**: Type-safe, explicit error enums
- **anyhow**: Type-erased, dynamic errors

Use ResExt for libraries (explicit error types), anyhow for applications (flexible error handling).

### vs thiserror

- **ResExt**: Context chains built-in
- **thiserror**: Manual error wrapping

Use ResExt when you need context propagation, thiserror for simple error types.

---

## Examples

### Basic Error Handling

```rust
#[resext]
enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

fn load_config(path: &str) -> Res<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read config")?;
    
    toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path))
}
```

### Multiple Error Types

```rust
#[resext(alias = ApiResult)]
enum ApiError {
    Network(reqwest::Error),
    Database(sqlx::Error),
    Json(serde_json::Error),
}

async fn fetch_user(id: u64) -> ApiResult<User> {
    let response = reqwest::get(format!("/users/{}", id))
        .await
        .context("Failed to fetch user")?;
    
    let user = response.json()
        .await
        .context("Failed to parse user data")?;
    
    Ok(user)
}
```

### Named Fields

```rust
#[resext]
enum DatabaseError {
    Connection { error: sqlx::Error },
    Query { error: sqlx::Error },
}
```

---

## Migration from v0.7.0

Old declarative syntax:

```rust
ResExt! {
    enum MyError {
        Io(std::io::Error),
    }
}
```

New proc macro syntax:

```rust
#[resext]
enum MyError {
    Io(std::io::Error),
}
```

To keep using the declarative macro:

```toml
[dependencies]
resext = { version = "0.8", default-features = false, features = ["declarative"] }
```

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

---

## License

MIT - See [LICENSE](../LICENSE) for details.
