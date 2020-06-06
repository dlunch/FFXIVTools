#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod archive_id;
mod batched_package;
mod error;
mod extracted_file_provider;
mod package;
mod raw_file;
mod reference;
mod sqpack_reader_extracted_file;

pub use archive_id::SqPackArchiveId;
pub use batched_package::BatchedPackage;
pub use error::{Result, SqPackReaderError};
pub use extracted_file_provider::ExtractedFileProvider;
pub use package::{BatchablePackage, Package};
pub use reference::{SqPackFileHash, SqPackFileReference};
pub use sqpack_reader_extracted_file::SqPackReaderExtractedFile;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod sqpack_reader;
        pub use extracted_file_provider::{ExtractedFileProviderLocal, ExtractedFileProviderWeb};
        pub use sqpack_reader::SqPackReader;
    }
}
