#[macro_use]
mod util;

mod ex;

use enum_map::Enum;

#[derive(Enum, Clone, Copy, PartialEq, Eq, Hash)]
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

pub use ex::{Ex, ExList};
