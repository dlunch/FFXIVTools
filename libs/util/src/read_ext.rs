use std::io;
use std::io::SeekFrom;

use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::prelude::ReadExt as async_std_read_ext;
use tokio::io::prelude::SeekExt;

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
