use std::io;
use std::io::SeekFrom;

use async_std::fs::File;
use async_std::io::prelude::ReadExt as async_std_read_ext;
use async_std::io::prelude::SeekExt;
use async_trait::async_trait;

#[async_trait]
pub trait ReadExt {
    async fn read_bytes(&mut self, offset: u64, size: usize) -> io::Result<Vec<u8>>;
}

#[async_trait]
impl ReadExt for File {
    async fn read_bytes(&mut self, offset: u64, size: usize) -> io::Result<Vec<u8>> {
        let mut data = vec![0; size];
        self.seek(SeekFrom::Start(offset)).await?;
        self.read_exact(&mut data).await?;

        Ok(data)
    }
}
