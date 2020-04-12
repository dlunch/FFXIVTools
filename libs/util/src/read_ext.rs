use std::io;
use std::io::SeekFrom;

use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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

#[macro_export]
macro_rules! read_as {
    ($file: expr, $offset: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, core::mem::size_of::<$type>()).await?;
            Ok::<_, std::io::Error>($crate::cast!(data, $type).clone())
        }
    };

    ($file: expr, $offset: expr, $count: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, $count as usize * core::mem::size_of::<$type>()).await?;

            Ok::<_, std::io::Error>(
                (0..$count as usize)
                    .map(|x| $crate::cast!(&data[x * core::mem::size_of::<$type>()..], $type).clone())
                    .collect::<Vec<_>>(),
            )
        }
    };
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
