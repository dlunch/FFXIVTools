mod definition;
mod exd;
mod exd_map;
mod exh;
mod exl;

pub use exl::ExList;

use std::io;

use sqpack_reader::Package;

use exd_map::ExdMap;
use exh::ExHeader;

#[allow(dead_code)] // WIP
pub struct Ex {
    header: ExHeader,
    data: ExdMap,
}

impl Ex {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let header = ExHeader::new(package, name).await?;
        let data = ExdMap::new(package, name, &header).await?;

        Ok(Self { header, data })
    }
}
