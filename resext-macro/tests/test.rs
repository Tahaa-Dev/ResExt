use resext_macro::resext;

#[resext]
enum ErrTypes {
    Custom(String),
    Io { error: std::io::Error },
}

#[test]
#[should_panic]
fn test_error_propagation() {
    fn temp() -> Res<()> {
        let path = "non_existent";
        let _ = std::fs::read(path).with_context(|| format!("Failed to read file: {}", path))?;

        let _: Res<()> = Err("TEST".to_string()).context("Custom error")?;

        Ok(())
    }
    temp().unwrap();
}

#[test]
fn test_long_context() -> Res<()> {
    let long_result: Res<()> = Ok::<(), std::io::Error>(())
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
    let result: Res<_> = std::fs::read("non_existent")
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
    let result: Res<_> = std::fs::read("non_existent").context("Context message");

    let err = result.unwrap_err();
    let debug_output = format!("{:?}", err);

    assert!(debug_output.contains("Context message"));
    assert!(debug_output.contains("Caused by:"));
}

#[test]
fn test_new_method() {
    let res = ResErr::new(Vec::new(), std::io::Error::other("TEST"));
    let res2 =
        ResErr::new(b"Test constructor `new()` method".to_vec(), std::io::Error::other("TEST"));

    assert_eq!(res.to_string(), "TEST");
    assert_eq!(res2.to_string(), "Test constructor `new()` method\nCaused by: TEST");
}

