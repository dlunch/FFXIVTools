use std::io;

use bytes::Bytes;
use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;

use sqpack_reader::Package;

use crate::Language;

lazy_static! {
    static ref LANGUAGE_SUFFIX: EnumMap<Language, &'static str> = enum_map! {
        Language::None => "",
        Language::Japanese => "_ja",
        Language::English => "_en",
        Language::Deutsch => "_de",
        Language::French => "_fr",
        Language::ChineseSimplified => "_chs",
        Language::ChineseTraditional => "_cht",
        Language::Korean => "_ko",
    };
}

#[allow(dead_code)] // WIP
pub struct ExData {
    data: Bytes,
}

impl ExData {
    pub async fn new(package: &dyn Package, name: &str, page_start: u32, language: Language) -> io::Result<Self> {
        let path = format!("exd/{}_{}{}.exd", name, page_start, LANGUAGE_SUFFIX[language]);
        let data = package.read_file(&path).await?;

        Ok(Self { data })
    }
}
