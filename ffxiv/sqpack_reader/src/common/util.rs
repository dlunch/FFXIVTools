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

macro_rules! read_and_parse {
    ($file: expr, $offset: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, <$type>::SIZE as usize).await?;
            Ok::<_, io::Error>(<$type>::parse(&data).unwrap().1)
        }
    };

    ($file: expr, $offset: expr, $count: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, $count as usize * <$type>::SIZE).await?;
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

pub fn round_up(num_to_round: usize, multiple: usize) -> usize {
    if multiple == 0 {
        return num_to_round;
    }

    let remainder = num_to_round % multiple;
    if remainder == 0 {
        num_to_round
    } else {
        num_to_round + multiple - remainder
    }
}
