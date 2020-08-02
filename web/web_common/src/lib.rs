use ffxiv_parser::Language;
use sqpack_reader::{ExtractedFileProviderWeb, Package, SqPackReaderExtractedFile};

pub struct Region {
    pub name: &'static str,
    pub version: &'static str,
    pub languages: Vec<Language>,
}

impl Region {
    pub fn package(&self) -> impl Package {
        let uri = format!("{}_{}", self.name, self.version);
        let provider = ExtractedFileProviderWeb::new(&format!("https://ffxiv-data.dlunch.net/compressed/{}/", uri));

        SqPackReaderExtractedFile::new(provider)
    }
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
