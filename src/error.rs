use std::{
    convert::From,
    error::Error,
    fmt::{self, Display},
    io::Error as IOError,
    num::ParseIntError,
    result,
};

#[derive(Debug, Clone)]
pub enum CipherError {
    InvalidParse,
    InvalidInverse(i64, i64),
    InvalidText,
    NotAlpha,
    NotPrime(i64),
    NullShift,
}

pub type Result<T> = result::Result<T, CipherError>;

impl Error for CipherError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidParse => write!(f, "Cannot parse provided value"),
            Self::InvalidInverse(a, m) => write!(f, "Cannot calculate multiplicative inverse of: {} and {}", a, m),
            Self::InvalidText => write!(f, "Cannot parse provided text"),
            Self::NullShift => write!(f, "Shift is out of an acceptable range"),
            Self::NotPrime(a) => write!(f, "Prime number should be used instead of: {}", a),
            Self::NotAlpha => write!(f, "String does contain not alphanumeric characters"),
        }
    }
}

impl From<ParseIntError> for CipherError {
    fn from(_: ParseIntError) -> Self { Self::InvalidParse }
}

impl From<IOError> for CipherError {
    fn from(_: IOError) -> Self { Self::InvalidText }
}
