#[macro_use]
mod util;

mod archive_id;
mod block;
mod raw_file;
mod reference;

pub use archive_id::SqPackArchiveId;
pub use block::SqPackDataBlock;
pub use raw_file::SqPackRawFile;
pub use reference::SqPackFileReference;
pub use util::ReadExt;

pub const MODEL_HEADER_SIZE: usize = 0x44;
