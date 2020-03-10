use std::collections::BTreeMap;
use std::io;

use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;

use sqpack_reader::Package;

use super::definition::{ExdData, ExdHeader, ExdRow};
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
    data: Vec<u8>,
    offsets: BTreeMap<u32, u32>,
}

impl ExData {
    pub async fn new(package: &dyn Package, name: &str, page_start: u32, language: Language) -> io::Result<Self> {
        let path = format!("exd/{}_{}{}.exd", name, page_start, LANGUAGE_SUFFIX[language]);
        let data = package.read_file(&path).await?;

        let header = parse!(data, ExdHeader);

        let item_count = header.row_size as usize / ExdRow::SIZE;
        let items_base = ExdHeader::SIZE;
        let offsets = (0..item_count)
            .map(|x| parse!(&data[items_base + x * ExdRow::SIZE..], ExdRow))
            .map(|x| (x.index, x.offset))
            .collect::<BTreeMap<_, _>>();

        Ok(Self { data, offsets })
    }

    pub fn read_row(&self, index: u32) -> Option<&[u8]> {
        let offset = *self.offsets.get(&index)? as usize;
        let data = parse!(&self.data[offset..], ExdData);

        Some(data.data)
    }
}
