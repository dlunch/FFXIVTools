use alloc::fmt;
use core::result;

#[derive(Debug)]
pub enum SqPackReaderError {
    NoSuchFolder,
    NoSuchFile,
    ReadError,
}

impl fmt::Display for SqPackReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SqPackReaderError::NoSuchFolder => f.write_str("No such folder"),
            SqPackReaderError::NoSuchFile => f.write_str("No such file"),
            SqPackReaderError::ReadError => f.write_str("Read error"),
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for SqPackReaderError {
    fn from(_: std::io::Error) -> SqPackReaderError {
        SqPackReaderError::ReadError
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SqPackReaderError {}

pub type Result<T> = result::Result<T, SqPackReaderError>;
