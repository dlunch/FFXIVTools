#![no_std]

#[cfg(target_endian = "big")]
compile_error!("Not supported on big endian");

extern crate alloc;

mod eqdp;
mod ex;
mod ffxiv_string;
mod lgb;
mod lvb;
mod mdl;
mod mtrl;
mod pap;
mod pbd;
mod sklb;
mod stm;
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

impl Language {
    pub fn from_raw(raw: u16) -> Self {
        match raw {
            0 => Language::None,
            1 => Language::Japanese,
            2 => Language::English,
            3 => Language::Deutsch,
            4 => Language::French,
            5 => Language::ChineseSimplified,
            6 => Language::ChineseTraditional,
            7 => Language::Korean,

            _ => panic!(),
        }
    }
}

pub use eqdp::Eqdp;
pub use ex::{Ex, ExList, ExRow, ExRowType};
pub use ffxiv_string::FFXIVString;
pub use lgb::{LayerGroupResourceItem, Lgb};
pub use lvb::Lvb;
pub use mdl::{BufferItemType, BufferItemUsage, Mdl};
pub use mtrl::{Mtrl, MtrlParameterType};
pub use pap::Pap;
pub use pbd::Pbd;
pub use sklb::Sklb;
pub use stm::Stm;
pub use tex::{Tex, TextureType};
