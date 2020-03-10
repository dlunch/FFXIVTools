use std::collections::BTreeMap;
use std::io;

use bytes::{Buf, Bytes};
use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;

use sqpack_reader::Package;

use super::definition::{ExdHeader, ExdRow};
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

pub struct ExData {
    data: Bytes,
    offsets: BTreeMap<u32, u32>,
}

impl ExData {
    pub async fn new(package: &dyn Package, name: &str, page_start: u32, language: Language) -> io::Result<Self> {
        let path = format!("exd/{}_{}{}.exd", name, page_start, LANGUAGE_SUFFIX[language]);
        let data = package.read_file(&path).await?;

        let mut cursor = data.clone();
        let header = parse!(cursor, ExdHeader);
        let item_count = header.row_size / ExdRow::SIZE as u32;
        let mut offsets = BTreeMap::new();
        for _ in 0..item_count {
            let row = parse!(cursor, ExdRow);
            offsets.insert(row.index, row.offset);
        }

        Ok(Self { data, offsets })
    }

    pub fn read_row(&self, index: u32) -> Option<Bytes> {
        let offset = *self.offsets.get(&index)? as usize;
        let mut data = self.data.slice(offset..);

        let length = data.get_u32() as usize;
        data.get_u16(); // unk

        Some(data.slice(..length))
    }
}
