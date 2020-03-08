use std::io;

use bytes::Buf;
use sqpack_reader::Package;

use super::definition::{ExhColumnHeader, ExhHeader};

#[allow(dead_code)] // WIP
pub struct Ex {
    header: ExhHeader,
    columns: Vec<ExhColumnHeader>,
}

impl Ex {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let mut data = package.read_file(&format!("exd/{}.exh", name)).await?;
        let header = parse!(data, ExhHeader);
        let columns = parse!(data, header.column_count as usize, ExhColumnHeader);

        Ok(Self { header, columns })
    }
}
