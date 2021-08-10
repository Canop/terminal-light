/*!

This crate answers the question *"Is the terminal dark or light?"*.

It provides

* the background color
* the background color's luma, which varies from 0 (black) to 1 (white)

A use case in a TUI is to determine what set of colors would be most suitable depending on the terminal's background:

```
let should_use_light_skin = terminal_light::luma()
    .map_or(false, |luma| luma > 0.6);
```

If you have very specialized skins, you may choose a more precise switch:

```
match terminal_light::luma() {
    Ok(luma) if luma > 0.85 => {
        // Use a "dark mode" skin.
    }
    Ok(luma) if luma < 0.2 => {
        // Use a "light mode" skin.
    }
    _ => {
        // Either we couldn't determine the mode or it's kind of medium.
        // We should use an itermediate skin, or one defining the background.
    }
}
```

# Strategies

## `$COLORFGBG` strategy

This environment variable is set by some terminals, like konsole or the rxvt family.
It can also be set by users.
Its value is like `15;0` where the second number is the ANSI code for the background color.

Bonus:

* querying an env variable is a fast operation

Malus:

* this env variable isn't usually immediately updated when you change the color of the terminal, a new shell session might be needed to get the up to date value
* the value isn't precise: `0` is "dark" and `15` is "light" but the real RGB color is uncertain as the low ANSI codes are often modified by the user

## "Dynamic colors" OSC escape sequence strategy

Modern terminals implement this xterm extension: a query making it possible to know the background color as RGB.

Terminal-light sends the query to `stdout`, waits for the answer on `stdin` with a timeout of 20ms, then analyses this answer.

Bonus:

* this works well on all tested linux terminals
* the value is precise and up to date when it's available

Malus:

* waiting for stdin with a timeout isn't implemented on Windows in this crate (help welcome)
* this isn't instant, a delay of 10 ms to get the answer isn't unusual
* if a not compatible terminal doesn't answer at all, we're waiting for 20ms

## Global strategy used by Terminal-light

1. if we're on a unix-like platform, we try the escape sequence strategy
2. if it failed or we're not on unix, we try the `$COLORFGBG` strategy
3. without a solution, we return a `TlError::Unsupported` error
*/

mod ansi_to_rgb;
mod color;
pub mod env;
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
/// let backround_color_rgb = terminal_light::background_color()
///     .map(|c| c.rgb()); // may be an error
/// ```
pub fn background_color() -> Result<Color, TlError> {
    let xterm_color = xterm::query_bg_color();
    if let Ok(xterm_color) = xterm_color {
        return Ok(Color::Rgb(xterm_color));
    }
    let env_color = env::bg_color();
    if let Ok(env_color) = env_color {
        return Ok(Color::Ansi(env_color));
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

