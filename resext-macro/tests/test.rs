use resext_macro::resext;

#[resext(
    alias = Resext
    delimiter = " ● "
)]
enum ErrTypes {
    Custom(String),
    Io { error: std::io::Error },
}

#[test]
#[should_panic]
fn test_error_propagation() {
    fn temp() -> Resext<()> {
        let path = "non_existent";
        let _ = std::fs::read(path).with_context(format_args!("Failed to read file: {}", path))?;

        let _: Resext<()> = Err("TEST".to_string()).context("Custom error")?;

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
fn test_error_display_format() {
    let result: Resext<_> = std::fs::read("non_existent")
        .context("Failed to read config")
        .with_context(format_args!("Failed to load application"));

    let err = result.unwrap_err();
    let output = format!("{}", err);

    assert!(output.contains("Failed to read config"));
    assert!(output.contains(" ● Failed to load application"));
    assert!(output.contains("Error:"));
}

#[test]
fn test_error_debug_format() {
    let result: Resext<_> = std::fs::read("non_existent").context("Context message");

    let err = result.unwrap_err();
    let debug_output = format!("{:?}", err);

    assert!(debug_output.contains("Context message"));
    assert!(debug_output.contains("Error:"));
}

#[test]
fn test_new_method() {
    let res = ResextErr::new("", std::io::Error::other("TEST"));
    let res2 = ResextErr::new("Test constructor `new()` method", std::io::Error::other("TEST"));

    assert_eq!(res.to_string(), "Error: TEST");
    assert_eq!(res2.to_string(), "Test constructor `new()` method\nError: TEST");
}
