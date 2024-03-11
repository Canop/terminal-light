use crate::*;

/// Query the $COLORFGBG env variable and parse
/// the result to extract the background in ANSI.
#[allow(clippy::iter_skip_next)]
pub fn bg_color() -> Result<AnsiColor, TlError> {
    let s = std::env::var("COLORFGBG").map_err(|_| TlError::NoColorFgBgEnv)?;
    parse_colorfgbg(&s)
}

/// Parse the content of the COLORFGBG variable, which is supposed
/// to be either like `17;45` or `0;default;15`, with the last token
/// being the ansi code of the background color.
///
/// Note that there doesn't seem to be any authoritive documentation
/// on the exact format.
fn parse_colorfgbg(s: &str) -> Result<AnsiColor, TlError> {
    let token: Vec<&str> = s.split(';').collect();
    let bg = match token.len() {
        2 => &token[1],
        3 => &token[2],
        _ => {
            return Err(TlError::WrongFormat(s.to_string()));
        }
    };
    let code = bg.parse()?;
    Ok(AnsiColor { code })
}

#[test]
fn test_parse_color_fgbg() {
    assert_eq!(parse_colorfgbg("17;45").unwrap(), AnsiColor::new(45));
    assert_eq!(parse_colorfgbg("0;default;15").unwrap(), AnsiColor::new(15));
    assert!(matches!(
        parse_colorfgbg("15").unwrap_err(),
        TlError::WrongFormat(_)
    ));
    assert!(matches!(
        parse_colorfgbg("15;FF").unwrap_err(),
        TlError::ParseInt(_)
    ));
}
