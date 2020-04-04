use std::io::{BufRead, Cursor};

use sqpack_reader::{Package, Result};

pub struct ExList {
    pub ex_names: Vec<String>,
}

impl ExList {
    pub async fn new(package: &dyn Package) -> Result<Self> {
        let data = package.read_file("exd/root.exl").await?;

        let cursor = Cursor::new(data);
        let ex_names = cursor
            .lines()
            .skip(1)
            .map(|x| Ok(x.unwrap().split(',').next().unwrap().to_owned()))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { ex_names })
    }
}
