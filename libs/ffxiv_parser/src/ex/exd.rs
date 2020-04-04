use std::collections::BTreeMap;

use bytes::Bytes;
use sqpack_reader::{Package, Result};
use util::parse;

use super::definition::{ExdData, ExdHeader, ExdRow};
use crate::Language;

pub struct ExData {
    data: Bytes,
    offsets: BTreeMap<u32, u32>,
}

impl ExData {
    pub async fn new(package: &dyn Package, name: &str, page_start: u32, language: Language) -> Result<Self> {
        let path = format!("exd/{}_{}{}.exd", name, page_start, Self::language_to_suffix(language));
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

    pub fn index(&self, index: u32) -> Option<&[u8]> {
        let offset = *self.offsets.get(&index)? as usize;
        let data = parse!(&self.data[offset..], ExdData);

        Some(data.data)
    }

    pub fn all(&self) -> impl Iterator<Item = (u32, &[u8])> {
        self.offsets
            .iter()
            .map(move |(index, offset)| (*index, parse!(&self.data[*offset as usize..], ExdData).data))
    }

    fn language_to_suffix(language: Language) -> &'static str {
        match language {
            Language::None => "",
            Language::Japanese => "_ja",
            Language::English => "_en",
            Language::Deutsch => "_de",
            Language::French => "_fr",
            Language::ChineseSimplified => "_chs",
            Language::ChineseTraditional => "_cht",
            Language::Korean => "_ko",
        }
    }
}
