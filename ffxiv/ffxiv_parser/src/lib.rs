mod ex;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
