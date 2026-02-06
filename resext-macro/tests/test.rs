#![no_std]
#![allow(invalid_from_utf8)]
use resext_macro::resext;
extern crate alloc;
use alloc::string::ToString;

#[resext(
    alias = Resext
    delimiter = " ● "
)]
enum ErrTypes {
    Index(usize),
    Utf8 { error: core::str::Utf8Error },
}

#[test]
#[should_panic]
fn test_error_propagation() {
    fn temp() -> Resext<()> {
        let path = "non_existent";
        let _ = core::str::from_utf8(&[0, 158, 22])
            .with_context(format_args!("Failed to read file: {}", path))?;

        let _: Resext<()> = Err(286).context("Custom error")?;

        Ok(())
    }
    temp().unwrap();
}

#[test]
fn test_long_context() -> Resext<()> {
    let long_result: Resext<()> = Ok::<(), usize>(())
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
    let result: Resext<_> = core::str::from_utf8(&[0, 158, 22])
        .context("Failed to read config")
        .with_context(format_args!("Failed to load application"));

    let err = result.unwrap_err();
    let output = format_args!("{}", err).to_string();

    assert!(output.contains("Failed to read config"));
    assert!(output.contains(" ● Failed to load application"));
    assert!(output.contains("Error:"));
}

#[test]
fn test_error_debug_format() {
    let result: Resext<_> = core::str::from_utf8(&[0, 158, 22]).context("Context message");

    let err = result.unwrap_err();
    let debug_output = format_args!("{:?}", err).to_string();

    assert!(debug_output.contains("Context message"));
    assert!(debug_output.contains("Error:"));
}

#[test]
fn test_new_method() {
    let res = ResextErr::new("", 404);
    let res2 = ResextErr::new("Test constructor `new()` method", 429);

    assert_eq!(format_args!("{}", res).to_string(), "Error: 404");
    assert_eq!(format_args!("{}", res2).to_string(), "Test constructor `new()` method\nError: 429");
}

mod isolated_test {
    use alloc::string::ToString;
    use resext_macro::resext;
    #[test]
    fn test_msg_truncation() {
        #[resext(buf_size = 5)]
        enum TestErr {
            Utf8(core::str::Utf8Error),
        }

        let res = core::str::from_utf8(&[0, 158, 22]).context("Good💖");

        assert_eq!(
            format_args!("{}", res.unwrap_err()).to_string(),
            "Good\nError: invalid utf-8 sequence of 1 bytes from index 1"
        );
    }
}
