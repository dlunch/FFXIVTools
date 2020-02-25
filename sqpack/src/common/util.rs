use std::io;
use std::io::SeekFrom;

use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[async_trait]
pub trait ReadExt {
    async fn read_to_vec(&mut self, offset: u64, size: usize) -> io::Result<Vec<u8>>;
}

#[async_trait]
impl ReadExt for File {
    async fn read_to_vec(&mut self, offset: u64, size: usize) -> io::Result<Vec<u8>> {
        let mut data = vec![0; size];
        self.seek(SeekFrom::Start(offset)).await?;
        self.read_exact(data.as_mut_slice()).await?;

        Ok(data)
    }
}

macro_rules! read_and_parse {
    ($file: expr, $offset: expr, $type: ty) => {
        async {
            let data = $file.read_to_vec($offset as u64, <$type>::SIZE as usize).await?;
            Ok::<_, io::Error>(<$type>::parse(&data).unwrap().1)
        }
    };

    ($file: expr, $offset: expr, $count: expr, $type: ty) => {
        async {
            let data = $file.read_to_vec($offset as u64, $count as usize * <$type>::SIZE).await?;
            Ok::<_, io::Error>(
                (0..$count)
                    .map(|x| {
                        let begin = (x as usize) * <$type>::SIZE;
                        let end = begin + <$type>::SIZE;
                        <$type>::parse(&data[begin..end]).unwrap().1
                    })
                    .collect::<Vec<_>>(),
            )
        }
    };
}
