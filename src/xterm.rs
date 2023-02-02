use crate::*;

fn query(query: &str, timeout_ms: u16) -> Result<String, TlError> {
    use crossterm::terminal::*;
    let switch_to_raw = !is_raw_mode_enabled()?;
    if switch_to_raw {
        enable_raw_mode()?;
    }
    let res = xterm_query::query(query, timeout_ms)
        .map_err(|e| e.into());
    if switch_to_raw {
        disable_raw_mode()?;
    }
    res
}

/// Query the bg color, assuming the terminal is in raw mode,
/// using the "dynamic colors" OSC escape sequence.
pub fn query_bg_color() -> Result<Rgb, TlError> {
    // we use the "dynamic colors" OSC escape sequence. It's sent with a ? for
    // a query and normally answered by the terminal with a color.
    // References:
    // - https://stackoverflow.com/a/28334701/263525
    // - https://invisible-island.net/xterm/ctlseqs/ctlseqs.html
    let s = query("\x1b]11;?\x07", 100)?;
    // The string we receive is like `"\u{1b}]11;rgb:<red>/<green>/<blue>\u{1b}\\"`
    // where `<red>`, `<green>`, and `<blue>` are 4 hex digits.
    // Most terminals don't support such precision so they fill the 4 digits
    // by repeating their 2 digits precision.
    // For example, supposing the background is in #38A4C9 (blue),
    // then we receive `"\u{1b}]11;rgb:3838/a4a4/c9c9\u{1b}\\"`.
    // We read only the most significant hex digits which are good enough
    // in all cases.
    match s.strip_prefix("\x1b]11;rgb:") {
        Some(raw_color) if raw_color.len() >= 14 => {
            Ok(Rgb::new(
                u8::from_str_radix(&raw_color[0..2], 16)?,
                u8::from_str_radix(&raw_color[5..7], 16)?,
                u8::from_str_radix(&raw_color[10..12], 16)?,
            ))
        }
        _ => Err(TlError::WrongFormat(s.to_string()))
    }
}

