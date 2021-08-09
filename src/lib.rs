/*!
This crate answers the question *"Is the terminal dark or light?"*.

It provides either

* the background color as RGB
* the background color's luma, which varies from 0 (black) to 1 (white)

A use case in a TUI is to determine what set of colors would be more suitable depending on the terminal's background:
```
let should_use_light_skin = terminal_light::luma()
    .map_or(false, |luma| luma > 0.6);
```

*/

use {
    std::io::{self, Read, Write},
};

/// terminal-light error type
#[derive(thiserror::Error, Debug)]
pub enum TlError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("UTF8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Parse Int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Wrong answer format: {0}")]
    WrongFormat(String)
}

/// RGB color, with u16 components
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Rgb {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
    pub fn rp(self) -> f32 {
        self.r as f32 / 65536f32
    }
    pub fn gp(self) -> f32 {
        self.g as f32 / 65536f32
    }
    pub fn bp(self) -> f32 {
        self.b as f32 / 65536f32
    }
    /// Compute the Luma value characterizing the "light" of the color,
    /// going from 0 (black) to 1 (white).
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Luma_(video)>
    pub fn luma(self) -> f32 {
        0.2627 * self.rp() + 0.6780 * self.gp() + 0.0593 * self.bp()
    }
}

/// Query the bg color, assuming the terminal is in raw mode
fn query_bg_color() -> Result<Rgb, TlError> {
    // we use the "dynamic colors" OSC escape sequence. It's sent with a ? for
    // a query and normally answered by the terminal with a color.
    // Reference: https://stackoverflow.com/a/28334701/263525
    let query = "\x1b]11;?\x07";
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", query)?;
    stdout.flush()?;
    let mut buffer = [0;50];
    let n = stdin.read(&mut buffer[..])?;
    let s = std::str::from_utf8(&buffer[..n])?;
    match s.strip_prefix("\x1b]11;rgb:") {
        Some(raw_color) if raw_color.len() >= 14 => {
            Ok(Rgb::new(
                u16::from_str_radix(&raw_color[0..4], 16)?,
                u16::from_str_radix(&raw_color[5..9], 16)?,
                u16::from_str_radix(&raw_color[10..14], 16)?,
            ))
        }
        _ => Err(TlError::WrongFormat(s.to_string()))
    }
}

/// Try to return the RGB background color of the terminal.
///
/// It should work if the terminal implements the "dynamic colors"
/// escape sequence interrogation, which is usually the case in
/// modern terminals.
pub fn bg_color() -> Result<Rgb, TlError> {
    crossterm::terminal::enable_raw_mode()?;
    let bg_color = query_bg_color();
    crossterm::terminal::disable_raw_mode()?;
    bg_color
}

/// Try to return the "luma" value of the terminal's background, characterizing
/// the "light" of the color, going from 0 (black) to 1 (white).
///
/// You can say a terminal is "dark" when the luma is below 0.2 and
/// "light" when it's over 0.9. If you need to choose a pivot between
/// "rather dark" and "rather light" then 0.6 should do.
///
/// It should work if the terminal implements the "dynamic colors"
/// escape sequence interrogation, which is usually the case in
/// modern terminals.
pub fn luma() -> Result<f32, TlError> {
    crossterm::terminal::enable_raw_mode()?;
    let bg_color = query_bg_color();
    crossterm::terminal::disable_raw_mode()?;
    bg_color.map(|c| c.luma())
}
