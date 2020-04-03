use std::error::Error;
use std::io;
use std::path::Path;

use clap::{App, Arg};
use futures::future;
use tokio::fs;

use sqpack_reader::{SqPackArchiveId, SqPackReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("sqpack_extractor")
        .arg(Arg::with_name("base_path").takes_value(true).required(true))
        .arg(Arg::with_name("root").takes_value(true).required(true))
        .get_matches();

    let package = SqPackReader::new(Path::new(matches.value_of("base_path").unwrap()))?;

    let archive_id = SqPackArchiveId::from_file_path(matches.value_of("root").unwrap());
    let archive = package.archive(archive_id).await?;

    future::join_all(archive.folders().map(|folder_hash| {
        let archive = &archive;
        async move {
            fs::create_dir(folder_hash.to_string()).await?;
            let files = archive.files(folder_hash)?;

            future::join_all(files.map(|file_hash| async move {
                let data = archive.read_as_compressed(folder_hash, file_hash).await?;
                let path = format!("{}/{}", folder_hash, file_hash);

                println!("{}", path);
                fs::write(path, data).await?;

                Ok::<_, io::Error>(())
            }))
            .await
            .into_iter()
            .collect::<io::Result<Vec<_>>>()?;

            Ok::<_, io::Error>(())
        }
    }))
    .await;

    Ok(())
}
