use resext::*;

#[test]
fn test_core() -> CtxResult<(), std::io::Error> {
    let error: Result<&str, std::io::Error> = Err(std::io::Error::other("I/O Failed"));

    let ctx = ErrCtx::new(
        std::io::Error::new(std::io::ErrorKind::HostUnreachable, "Host refused to connect"),
        b"Failed to send request".to_vec(),
    );
    println!("{}\n", ctx);
    println!("{:?}\n", ctx);

    let ctx: CtxResult<&str, std::io::Error> = error
        .context("Failed to do I/O work.")
        .byte_context(b"Failed to read file.".to_vec())
        .with_context(|| format!("File [{}] failed to open.", "foo.txt"));

    let ctx_err = ctx.as_ref().unwrap_err();

    assert_eq!(
        ctx_err.msg(),
        String::from(
            "Failed to do I/O work.\n- Failed to read file.\n- File [foo.txt] failed to open."
        )
    );

    println!("{}", ctx_err);

    // Statement for checking error formatting with `?`, commented out as it fails tests but is
    // useful for general testing / debugging.
    //ctx?;

    Ok(())
}

#[test]
fn test_empty_ctx() {
    let ctx = ErrCtx::new(
        std::io::Error::other("error"),
        b"".to_vec(), // Empty context
    );
    let output = format!("{}", ctx);
    assert!(!output.contains("\n- "));
}

#[test]
fn test_methods() {
    let res: Result<usize, std::io::Error> = Ok(20);
    let res2: Result<&str, std::io::Error> = Ok("ok");

    let value = res.or_exit(1);
    let value2 = res2.better_expect("Failed to unwrap Result.", 1, true);

    throw_err_if!(value.to_string() == value2, "Values aren't unique", 1);
    throw_err_if!(
        value != 20,
        || format!(
            "Value didn't unwrap correctly, it unwrapped into [`{}`] instead of [`20`]",
            value
        ),
        1
    );
}

#[test]
fn test_long_context_chain() {
    let result: Result<(), std::io::Error> = Err(std::io::Error::other("root"));
    let ctx: ErrCtx<std::io::Error> = result
        .context("first")
        .context("second")
        .context("third")
        .context("fourth")
        .context("fifth")
        .unwrap_err();

    let msg = ctx.msg();
    assert!(msg.contains("first"));
    assert!(msg.contains("fifth"));
    assert_eq!(msg.matches("\n- ").count(), 4); // 4 delimiters for 5 messages
}
