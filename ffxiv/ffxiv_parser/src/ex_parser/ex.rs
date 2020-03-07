use std::io;

use bytes::Buf;
use sqpack_reader::Package;

use super::definition::ExhHeader;

pub struct Ex {
    #[allow(dead_code)] // WIP
    header: ExhHeader,
}

impl Ex {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let mut data = package.read_file(&format!("exd/{}.exh", name)).await?;
        let header = parse!(data, 0, ExhHeader);

        Ok(Self { header })
    }
}
