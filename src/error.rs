
/// terminal-light error type
#[derive(thiserror::Error, Debug)]
pub enum TlError {

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[cfg(unix)]
    #[error("Xterm-query error: {0}")]
    XtermQuery(#[from] xterm_query::XQError),

    #[error("Parse Int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Wrong answer format: {0}")]
    WrongFormat(String),

    #[error("No $COLORFGBG env variable")]
    NoColorFgBgEnv,

    #[error("Var error: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Unsupported platform")] // nothing works
    Unsupported,
}


