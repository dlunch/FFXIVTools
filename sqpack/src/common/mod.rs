#[macro_use]
mod util;

mod archive_id;
mod decoder;
mod reference;

pub use archive_id::SqPackArchiveId;
pub use decoder::{decode_block, decode_compressed_data};
pub use reference::SqPackFileReference;
pub use util::ReadExt;

pub const MODEL_HEADER_SIZE: usize = 4;
