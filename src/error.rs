
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
    #[error("No $COLORFGBG env variable")]
    NoColorFgBgEnv,
    #[error("Var error: {0}")]
    VarError(#[from] std::env::VarError),
    #[error("Unsupported platform")] // nothing works
    Unsupported,
}


