use std::error::Error;
use std::path::Path;

use actix_web::Result;
use dotenv::dotenv;

use sqpack_reader::Package;

pub struct Context {
    pub package: Box<dyn Package>,
}

impl Context {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        #[cfg(unix)]
        let package = Box::new(sqpack_reader::SqPackReaderFile::with_path(sqpack_reader::FileProviderFile::new(
            Path::new("/mnt/i/FFXIVData/data/kor_505"),
        ))?);
        #[cfg(windows)]
        let package = Box::new(sqpack_reader::SqPackReader::new(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack",
        ))?);

        Ok(Self { package })
    }
}
