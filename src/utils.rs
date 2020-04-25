use std::{
    error,
    fmt,
};

#[derive(Debug, Clone)]
pub enum CipherError {
    NullShift,
    NotPrime,
}

impl error::Error for CipherError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NullShift => write!(f, "Shift is out of range"),
            Self::NotPrime => write!(f, "Relatively prime number should be used")
        }
    }
}