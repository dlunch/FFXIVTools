use std::io;
use std::io::SeekFrom;

use async_trait::async_trait;
use bytes::Bytes;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[async_trait]
pub trait ReadExt {
    async fn read_bytes(&mut self, offset: u64, size: usize) -> io::Result<Bytes>;
}

#[async_trait]
impl ReadExt for File {
    async fn read_bytes(&mut self, offset: u64, size: usize) -> io::Result<Bytes> {
        let mut data = vec![0; size];
        self.seek(SeekFrom::Start(offset)).await?;
        self.read_exact(&mut data).await?;

        Ok(Bytes::from(data))
    }
}

#[macro_export]
macro_rules! read_and_parse {
    ($file: expr, $offset: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, <$type>::SIZE as usize).await?;
            Ok::<_, std::io::Error>($crate::parse!(data, $type))
        }
    };

    ($file: expr, $offset: expr, $count: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, $count as usize * <$type>::SIZE).await?;
            Ok::<_, std::io::Error>($crate::parse!(data, $count, $type))
        }
    };
}
