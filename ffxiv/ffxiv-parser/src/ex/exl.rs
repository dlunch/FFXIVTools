use std::io;
use std::io::{BufRead, Cursor};

use sqpack::Package;

pub struct ExList {
    pub ex_names: Vec<String>,
}

impl ExList {
    pub async fn new(package: &dyn Package) -> io::Result<Self> {
        let data = package.read_file("exd/root.exl").await?;

        let cursor = Cursor::new(data);
        let ex_names = cursor
            .lines()
            .skip(1)
            .map(|x| Ok(x?.split(',').nth(0).unwrap().to_owned()))
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Self { ex_names })
    }
}
