// after rust 1.44
// #![no_std]

#[cfg(target_endian = "big")]
compile_error!("Not supported on big endian");

extern crate alloc;

mod ex;
mod ffxiv_string;
mod lgb;
mod lvb;
mod mdl;
mod tex;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(u16)]
pub enum Language {
    None = 0,
    Japanese = 1,
    English = 2,
    Deutsch = 3,
    French = 4,
    ChineseSimplified = 5,
    ChineseTraditional = 6,
    Korean = 7,
}

pub use ex::{Ex, ExList, ExRowType};
pub use lgb::{LayerGroupResourceItem, Lgb};
pub use lvb::Lvb;
pub use mdl::{BufferItemType, BufferItemUsage, Mdl};
pub use tex::{Tex, TextureType};
