// after rust 1.44
// #![no_std]

extern crate alloc;

mod ex;
mod ffxiv_string;
mod lgb;
mod lvb;

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
