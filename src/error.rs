use std::fmt::Formatter;
use std::{fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Git(git2::Error),
    NoTagFound,
    NoVersionChange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io(ref err) => write!(f, "IO: {}", err),
            Self::Git(ref err) => write!(f, "Git: {}", err),
            Self::NoTagFound => write!(f, "No valid tag found"),
            Self::NoVersionChange => write!(f, "No change in version"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Self::Git(err)
    }
}
