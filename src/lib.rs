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

use {
    nix::sys::epoll::*,
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
    WrongFormat(String),
    #[error("Timeout waiting for xterm")]
    Timeout,
    #[error("Terminal error code: {0}")]
    TerminalError(i64),
    #[error("Nix error: {0}")]
    NixError(#[from] nix::errno::Errno),
    #[error("Unsupported platform")] // not unix-like
    Unsupported,
}

/// RGB color, with u16 components
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    /// red
    pub r: u16,
    /// green
    pub g: u16,
    /// blue
    pub b: u16,
}

impl Rgb {
    /// Create a new RGB color from its components
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
    /// red part in `[0,1]`
    pub fn rp(self) -> f32 {
        self.r as f32 / 65535f32
    }
    /// green part in `[0,1]`
    pub fn gp(self) -> f32 {
        self.g as f32 / 65535f32
    }
    /// blue part in `[0,1]`
    pub fn bp(self) -> f32 {
        self.b as f32 / 65535f32
    }
    /// Compute the Luma value characterizing the "light" of the color,
    /// going from 0 (black) to 1 (white).
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Luma_(video)>
    pub fn luma(self) -> f32 {
        0.2627 * self.rp() + 0.6780 * self.gp() + 0.0593 * self.bp()
    }
}

#[cfg(not(unix))]
fn query_xterm(query: &str, response: &mut[u8], timeout_ms: isize) -> Result<usize, TlError> {
    Err(TlError::Unsupported)
}

#[cfg(unix)]
fn query_xterm(query: &str, response: &mut[u8], timeout_ms: isize) -> Result<usize, TlError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", query)?;
    stdout.flush()?;
    let poll_fd = epoll_create1(EpollCreateFlags::empty())?;
    let mut event = EpollEvent::new(EpollFlags::EPOLLIN, 0);
    epoll_ctl(
        poll_fd,
        EpollOp::EpollCtlAdd,
        nix::libc::STDIN_FILENO,
        Some(& mut event),
    )?;
    let mut events = [EpollEvent::empty(); 1];
    let fd_count = epoll_wait(poll_fd, &mut events, timeout_ms)?;
    if fd_count == 0 {
        Err(TlError::Timeout) // no file descriptor was ready in time
    } else {
        let bytes_written = stdin.read(response)?;
        Ok(bytes_written)
    }
}

/// Query the bg color, assuming the terminal is in raw mode
fn query_bg_color() -> Result<Rgb, TlError> {
    // we use the "dynamic colors" OSC escape sequence. It's sent with a ? for
    // a query and normally answered by the terminal with a color.
    // Reference: https://stackoverflow.com/a/28334701/263525
    let query = "\x1b]11;?\x07";
    let mut buffer = [0;50];
    let n = query_xterm(query, &mut buffer, 20)?;
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

