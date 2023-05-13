use std::fmt;

#[derive(Debug)]
pub enum Error {
    Glob(globset::Error)
}

impl From<globset::Error> for Error {
    fn from(value: globset::Error) -> Error {
        Error::Glob(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Glob(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {  }
