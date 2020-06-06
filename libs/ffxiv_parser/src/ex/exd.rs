use alloc::{collections::BTreeMap, format, vec::Vec};
use core::mem::size_of;

use sqpack_reader::{Package, Result};
use util::cast;

use super::definition::{ExdHeader, ExdRow};
use crate::Language;

pub struct ExData {
    data: Vec<u8>,
    offsets: BTreeMap<u32, u32>,
}

impl ExData {
    pub async fn new(package: &dyn Package, name: &str, page_start: u32, language: Language) -> Result<Self> {
        let path = format!("exd/{}_{}{}.exd", name, page_start, Self::language_to_suffix(language));
        let data = package.read_file(&path).await?;

        let header = cast::<ExdHeader>(&data);

        let item_count = header.row_size.get() as usize / size_of::<ExdRow>();
        let items_base = size_of::<ExdHeader>();
        let offsets = (0..item_count)
            .map(|x| cast::<ExdRow>(&data[items_base + x * size_of::<ExdRow>()..]))
            .map(|x| (x.index.get(), x.offset.get()))
            .collect::<BTreeMap<_, _>>();

        Ok(Self { data, offsets })
    }

    pub fn index(&self, index: u32) -> Option<&[u8]> {
        let offset = *self.offsets.get(&index)? as usize;

        Some(&self.data[offset..])
    }

    pub fn all(&self) -> impl Iterator<Item = (u32, &[u8])> {
        self.offsets.iter().map(move |(index, offset)| (*index, &self.data[*offset as usize..]))
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
