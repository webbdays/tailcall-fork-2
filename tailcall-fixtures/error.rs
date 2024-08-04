use std::fmt::Display;

use derive_more::{DebugCustom, From};

#[derive(From, DebugCustom)]
pub enum Error {
    #[debug(fmt = "Std Fmt Error: {}", _0)]
    StdFmt(std::fmt::Error),

    #[debug(fmt = "Std IO Error: {}", _0)]
    IO(std::io::Error),

    #[debug(fmt = "Failed to resolve filename: {}", _0)]
    FilenameNotResolved(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StdFmt(error) => write!(f, "Std Fmt Error: {}", error),
            Error::IO(error) => write!(f, "Std IO Error: {}", error),
            Error::FilenameNotResolved(file_name) => {
                write!(f, "Failed to resolve filename: {}", file_name)
            }
        }
    }
}

pub type Result<A> = std::result::Result<A, Error>;
