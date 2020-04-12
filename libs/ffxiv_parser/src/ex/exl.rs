use alloc::{borrow::ToOwned, str, string::String, vec::Vec};

use sqpack_reader::{Package, Result};

pub struct ExList {
    pub ex_names: Vec<String>,
}

impl ExList {
    pub async fn new(package: &dyn Package) -> Result<Self> {
        let data = package.read_file("exd/root.exl").await?;
        let data_str = str::from_utf8(&data).unwrap();

        let ex_names = data_str
            .lines()
            .skip(1)
            .map(|x| Ok(x.split(',').next().unwrap().to_owned()))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { ex_names })
    }
}
