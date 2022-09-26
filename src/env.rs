use crate::*;

/// query the $COLORFGBG env variable and parse
/// the result to extract the background in ANSI.
#[allow(clippy::iter_skip_next)]
pub fn bg_color() -> Result<AnsiColor, TlError> {
    let s = std::env::var("COLORFGBG")?;
    // the value is supposed to be like 17;45 where
    // 17 is the ansi fg and 45 is the ansi bg
    let bg = s.split(';').skip(1).next()
        .ok_or_else(|| TlError::WrongFormat(s.to_string()))?;
    let code = bg.parse()?;
    Ok(AnsiColor { code })
}
