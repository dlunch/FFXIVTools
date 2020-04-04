mod archive_id;
mod error;
mod extracted_file_provider;
mod package;
mod raw_file;
mod reference;
mod sqpack_reader_extracted_file;

pub use self::archive_id::SqPackArchiveId;
pub use self::error::{Result, SqPackReaderError};
pub use self::extracted_file_provider::ExtractedFileProvider;
pub use self::package::Package;
pub use self::reference::SqPackFileHash;
pub use self::sqpack_reader_extracted_file::SqPackReaderExtractedFile;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod sqpack_reader;
        pub use self::extracted_file_provider::{ExtractedFileProviderLocal, ExtractedFileProviderWeb};
        pub use self::sqpack_reader::SqPackReader;
    }
}
