use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, TextError>;

#[derive(Debug, Eq, PartialEq)]
pub enum TextError {
    IOError(io::ErrorKind),
    FormatError,
    DomainNotFound,
    LocaleNotFound,
}

impl From<io::Error> for TextError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err.kind())
    }
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "IO error: {:?}", err),
            Self::FormatError => write!(f, "unexpected .po format"),
            Self::DomainNotFound => write!(f, "domain not found"),
            Self::LocaleNotFound => write!(f, "locale not found"),
        }
    }
}
