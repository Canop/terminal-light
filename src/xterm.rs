use {
    crate::*,
};

#[cfg(not(unix))]
fn query(_query: &str, _response: &mut[u8], _timeout_ms: isize) -> Result<usize, TlError> {
    Err(TlError::Unsupported)
}

/// query the xterm interface, assuming the terminal is in raw mode
/// (or we would block waiting for a newline)
#[cfg(unix)]
fn query_in_raw(query: &str, response: &mut[u8], timeout_ms: isize) -> Result<usize, TlError> {
    use nix::sys::epoll::*;
    use std::io::{self, Read, Write};
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

#[cfg(unix)]
fn query(query: &str, response: &mut[u8], timeout_ms: isize) -> Result<usize, TlError> {
    use crossterm::terminal::*;
    let switch_to_raw = !is_raw_mode_enabled()?;
    if switch_to_raw {
        enable_raw_mode()?;
    }
    let res = query_in_raw(query, response, timeout_ms);
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
    let mut buffer = [0;50];
    let n = query( "\x1b]11;?\x07", &mut buffer, 20)?;
    let s = std::str::from_utf8(&buffer[..n])?;
    // the string we receive is quite strange.
    // For example, supposing the background is in #38A4C9 (blue),
    // then we receive "\u{1b}]11;rgb:3838/a4a4/c9c9\u{1b}\\"
    match s.strip_prefix("\x1b]11;rgb:") {
        Some(raw_color) if raw_color.len() >= 14 => {
            Ok(Rgb::new(
                u8::from_str_radix(&raw_color[2..4], 16)?,
                u8::from_str_radix(&raw_color[7..9], 16)?,
                u8::from_str_radix(&raw_color[12..14], 16)?,
            ))
        }
        _ => Err(TlError::WrongFormat(s.to_string()))
    }
}

