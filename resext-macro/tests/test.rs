#![allow(invalid_from_utf8)]
#![no_std]
extern crate alloc;
use alloc::string::ToString;

use resext_macro::resext;

// Temporary macro as usage in actual projects requires resext crate
struct Writer<W: core::fmt::Write + ?Sized>(W);

impl<W: core::fmt::Write + ?Sized> core::fmt::Write for Writer<W> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }
}

macro_rules! ctx {
    ($fmt:expr, $($args:tt)*) => {
        {
            |w, d, mp, ms| {
                use core::fmt::Write;

                let mut w = Writer(w);

                let _ = w.write_str(d);
                let _ = w.write_str(mp);
                let _ = write!(w, $fmt, $($args)*);
                let _ = w.write_str(ms);

                w.0
            }
        }
    };

    ($fmt:expr) => {
        {
            |w, d, mp, ms| {
                use core::fmt::Write;

                let mut w = Writer(w);

                let _ = w.write_str(d);
                let _ = w.write_str(mp);
                let _ = write!(w, $fmt);
                let _ = w.write_str(ms);

                w.0
            }
        }
    };
}

#[resext(
    alias = Resext
    delimiter = " ● "
    buf_size = 24
    alloc = true
)]
enum ErrTypes {
    HttpResponse(usize),
    Utf8 { error: core::str::Utf8Error },
}

#[test]
fn test_error_propagation() {
    fn temp() -> Resext<()> {
        let path = "non_existent";

        let _ = core::str::from_utf8(&[0, 158, 22]).context(ctx!(
            "Failed to format file extension from bytes for path: {}",
            path
        ))?;

        let _: Resext<()> = Err(286).context("Custom error")?;

        Ok(())
    }

    let err = temp().unwrap_err();

    assert_eq!(
        format_args!("{}", err).to_string(),
        "Failed to format file extension from bytes for path: non_existent\nError: invalid utf-8 sequence of 1 bytes from index 1"
    );
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
        .context(ctx!("Failed to load application"));

    let err = result.unwrap_err();
    let output = format_args!("{}", err).to_string();

    assert!(output.contains("Failed to read config"));
    assert!(output.contains(" ● Failed to load application"));
    assert!(output.contains("Error:"));
}

#[test]
fn test_error_debug_format() {
    let result: Resext<_> =
        core::str::from_utf8(&[0, 158, 22]).context("Context message");

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
    assert_eq!(
        format_args!("{}", res2).to_string(),
        "Test constructor `new()` method\nError: 429"
    );
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
            "Good...\nError: invalid utf-8 sequence of 1 bytes from index 1"
        );
    }
}
