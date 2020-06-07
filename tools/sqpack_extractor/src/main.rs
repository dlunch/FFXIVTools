use std::io;
use std::path::Path;

use async_std::fs;
use clap::{App, Arg};
use futures::{
    future,
    stream::{FuturesUnordered, TryStreamExt},
};

use sqpack_reader::{SqPackArchiveId, SqPackReader};

#[async_std::main]
async fn main() -> io::Result<()> {
    let matches = App::new("sqpack_extractor")
        .arg(Arg::with_name("base_path").takes_value(true).required(true))
        .arg(Arg::with_name("root").takes_value(true).required(true))
        .get_matches();

    let package = SqPackReader::new(Path::new(matches.value_of("base_path").unwrap()))?;

    let archive_id = SqPackArchiveId::from_file_path(matches.value_of("root").unwrap());
    let archive = package.archive(archive_id).await?;

    archive
        .folders()
        .map(|folder_hash| {
            let archive = &archive;
            async move {
                fs::create_dir(folder_hash.to_string()).await?;
                let files = archive
                    .files(folder_hash)
                    .map_err(|x| io::Error::new(io::ErrorKind::NotFound, x.to_string()))?;

                files
                    .map(|file_hash| async move {
                        let data = archive
                            .read_as_compressed(folder_hash, file_hash)
                            .await
                            .map_err(|x| io::Error::new(io::ErrorKind::NotFound, x.to_string()))?;
                        let path = format!("{}/{}", folder_hash, file_hash);

                        println!("{}", path);
                        fs::write(path, data).await?;

                        Ok::<_, io::Error>(())
                    })
                    .collect::<FuturesUnordered<_>>()
                    .try_for_each(|_| future::ready(Ok(())))
                    .await?;

                Ok::<_, io::Error>(())
            }
        })
        .collect::<FuturesUnordered<_>>()
        .try_for_each(|_| future::ready(Ok(())))
        .await?;

    Ok(())
}
