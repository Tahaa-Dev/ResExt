use resext::*;

#[test]
fn test() -> CtxResult<(), std::io::Error> {
    let error: Result<(), std::io::Error> = Err(std::io::Error::other("I/O Error"));

    let ctx = ErrCtx::new(
        std::io::Error::new(
            std::io::ErrorKind::HostUnreachable,
            "Host refused to connect",
        ),
        b"Failed to send request".as_slice(),
    );
    println!("{}\n", ctx);
    println!("{:?}\n", ctx);

    error
        .context("Failed to do I/O work.")
        .context("Failed to read file.")?;

    Ok(())
}
