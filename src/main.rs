use terminal_light::*;

/// print the "luma" value of the terminal's background color
/// (from 0 (black) to 1 (white)) or an error when it failed.
fn main() -> Result<(), TlError> {
    println!("{}", luma()?);
    Ok(())
}
