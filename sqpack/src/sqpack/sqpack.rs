use std::fs::File;
use std::io::{BufReader, Read, Result, Seek, SeekFrom};

use crate::package::Package;

use super::parser::*;

#[derive(Default)]
pub struct SqPack {}

impl SqPack {
    pub fn mount(&self, path: &str) -> Result<()> {
        let index_path = format!("{}.index", path);
        let f = File::open(index_path)?;
        let mut r = BufReader::new(f);

        let sqpack_header = parse!(r, SqPackHeader);
        let index_header = parse!(r, SqPackIndexSegmentHeader, sqpack_header.header_length);

        Ok(())
    }
}

impl Package for SqPack {
    fn read_file(&self, filename: &str) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::SqPack;
    #[test]
    fn test_read() {
        let pack = SqPack {};
        pack.mount("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ffxiv\\0a0000.win32")
            .unwrap();
    }
}
