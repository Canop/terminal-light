/*!
This crate answers the question *"Is the terminal dark or light?"*.

It provides

* the background color as RGB
* the background color's luma, which varies from 0 (black) to 1 (white)

A use case in a TUI is to determine what set of colors would be most suitable depending on the terminal's background:
```
let should_use_light_skin = terminal_light::luma()
    .map_or(false, |luma| luma > 0.6);
```

The behavior and result of the [bg_color] and [luma] functions depend:

On non unix-like platforms, you'll receive a [TlError::Unsupported] error (help welcome to improve this).

On unix-like (linux, Darwin, etc.), a terminal sequence is sent to stdout and the response is read (with timeout) on stdin.

If the terminal is somehow modern, the answer is received and analyzed to give you the color.

If the terminal doesn't answer fast enough, a [TlError::Timeout] error is returned (current timeout is 20ms).

If the terminal's answer isn't understood, an other type of error is returned.

*/

mod ansi_to_rgb;
mod color;
mod env;
mod error;
mod xterm;

pub use {
    color::*,
    error::*,
};

/// Try to determine the background color of the terminal.
///
/// The result may come as Ansi or Rgb, depending on where
/// the information has been found.
///
/// If you want it as RGB:
///
/// ```
/// let backround_color_rgb = terminal_ligh::background_color()
///     .map(|c| c.rgb()); // may be an error
/// ```
pub fn background_color() -> Result<Color, TlError> {
    let env_color = env::bg_color();
    dbg!(&env_color);
    if let Ok(env_color) = env_color {
        return Ok(Color::Ansi(env_color));
    }
    let xterm_color = xterm::query_bg_color();
    dbg!(&xterm_color);
    if let Ok(xterm_color) = xterm_color {
        return Ok(Color::Rgb(xterm_color));
    }
    Err(TlError::Unsupported)
}

/// Try to return the "luma" value of the terminal's background, characterizing
/// the "light" of the color, going from 0 (black) to 1 (white).
///
/// You can say a terminal is "dark" when the luma is below 0.2 and
/// "light" when it's over 0.9. If you need to choose a pivot between
/// "rather dark" and "rather light" then 0.6 should do.
pub fn luma() -> Result<f32, TlError> {
    background_color().map(|c| c.luma())
}

