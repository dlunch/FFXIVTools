use std::error::Error;
use std::path::Path;

use clap::{App, Arg};

use sqpack_reader::{SqPackArchiveId, SqPackReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("sqpack_extractor")
        .arg(Arg::with_name("base_path").takes_value(true).required(true))
        .arg(Arg::with_name("root").takes_value(true).required(true))
        .get_matches();

    let package = SqPackReader::new(Path::new(matches.value_of("base_path").unwrap()))?;
    let file_list = package.all(SqPackArchiveId::from_file_path(matches.value_of("root").unwrap())).await?;

    for (folder_hash, file_hash) in file_list {
        println!("{}/{}", folder_hash, file_hash);
    }

    Ok(())
}
