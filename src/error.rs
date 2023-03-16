use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct TryFromBEBytesError;

impl fmt::Display for TryFromBEBytesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not read struct from provided BE bytes")
    }
}

impl Error for TryFromBEBytesError {}