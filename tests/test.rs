use resext::{ResExt, panic_if};

ResExt! {
    pub enum ErrorTypes {
        Io(std::io::Error),
        Var(std::env::VarError),
        Other,
    } as Resext
}

#[test]
#[should_panic]
fn test_error_propagation() {
    fn temp() -> Resext<()> {
        let path = "non_existent";
        let _ = std::fs::read(path).with_context(|| format!("Failed to read file: {}", path))?;

        std::env::var("non_existent").context("Failed to read env variable")?;

        Ok(())
    }
    temp().unwrap();
}

#[test]
fn test_long_context() -> Resext<()> {
    let long_result: Resext<()> = Ok::<(), std::io::Error>(())
        .context("msg1")
        .context("msg2")
        .context("msg3")
        .context("msg4")
        .context("msg5");

    long_result?;

    Ok(())
}

#[test]
fn test_panic_methods() {
    let ok: String = Ok::<String, std::env::VarError>("String".to_string())
        .better_expect(|| "Failed to get env vat", 2);

    let ok2: u8 = Ok::<u8, std::io::Error>(0).or_exit(1);

    panic_if!(ok != "String" || ok2 != 0, "Error: Result does not equal \"String\"", 1);
}

#[test]
fn test_error_display_format() {
    let result: Resext<_> = std::fs::read("non_existent")
        .context("Failed to read config")
        .with_context(|| "Failed to load application".to_string());

    let err = result.unwrap_err();
    let output = format!("{}", err);

    assert!(output.contains("Failed to read config"));
    assert!(output.contains("- Failed to load application"));
    assert!(output.contains("Caused by:"));
}

#[test]
fn test_error_debug_format() {
    let result: Resext<_> = std::fs::read("non_existent").context("Context message");

    let err = result.unwrap_err();
    let debug_output = format!("{:?}", err);

    assert!(debug_output.contains("Context message"));
    assert!(debug_output.contains("Caused by:"));
}
