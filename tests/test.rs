use resext::*;

fn integration_tests() {
    let error: Result<&str, std::io::Error> = Err(std::io::Error::other("Error"));

    let ok: Result<&str, std::io::Error> = Ok("World!");

    let error_with_ctx = error.context("I/O Error").better_expect("Error", 1, true);

    let ok_to_value = ok.or_exit(1);

    println!("{}\n{}", ok_to_value, error_with_ctx);
}
