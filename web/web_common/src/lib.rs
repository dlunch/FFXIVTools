#![no_std]
extern crate alloc;

mod wasm_package;

pub use wasm_package::WasmPackage;

use alloc::{vec, vec::Vec};

use ffxiv_parser::Language;

pub struct Region {
    pub name: &'static str,
    pub version: &'static str,
    pub languages: Vec<Language>,
}

pub fn regions() -> [Region; 3] {
    [
        Region {
            name: "global",
            version: "525",
            languages: vec![Language::Japanese, Language::English, Language::Deutsch, Language::French],
        },
        Region {
            name: "chn",
            version: "520",
            languages: vec![Language::ChineseSimplified],
        },
        Region {
            name: "kor",
            version: "510",
            languages: vec![Language::Korean],
        },
    ]
}
