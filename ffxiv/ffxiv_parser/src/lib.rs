#[macro_use]
mod util;

mod ex;

use enum_map::Enum;
use num_derive::FromPrimitive;

#[derive(FromPrimitive, Enum, Clone, Copy, PartialEq, Eq, Hash)]
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

pub use ex::{Ex, ExList};
