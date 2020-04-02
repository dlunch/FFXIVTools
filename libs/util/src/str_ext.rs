use std::str;
use std::str::Utf8Error;

pub trait StrExt {
    fn from_null_terminated_utf8(buf: &[u8]) -> Result<&str, Utf8Error>;
}

impl StrExt for str {
    fn from_null_terminated_utf8(buf: &[u8]) -> Result<&str, Utf8Error> {
        let end = buf.iter().position(|&x| x == b'\0').unwrap();

        str::from_utf8(&buf[..end])
    }
}
