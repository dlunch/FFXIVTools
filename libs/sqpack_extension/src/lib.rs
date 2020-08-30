#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod batchable_package;
mod batched_package;
mod extracted_file_provider;
mod extracted_raw_file;
mod sqpack_reader_extracted_file;

pub use batchable_package::BatchablePackage;
pub use batched_package::BatchedPackage;
pub use extracted_file_provider::ExtractedFileProvider;
pub use extracted_file_provider::ExtractedFileProviderWeb;
pub use extracted_raw_file::ExtractedSqPackRawFile;
pub use sqpack_reader_extracted_file::SqPackReaderExtractedFile;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub use extracted_file_provider::ExtractedFileProviderLocal;
    }
}
