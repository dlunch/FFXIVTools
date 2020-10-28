use std::rc::Rc;

use common::WasmPackage;

use ffxiv_parser::ExList;
use sqpack::Result;

const ROOTS: [&str; 13] = [
    "common",
    "bgcommon",
    "bg",
    "cut",
    "chara",
    "shader",
    "ui",
    "sound",
    "vfx",
    "ui_script",
    "exd",
    "game_script",
    "music",
];

pub struct FileList {
    package: Rc<WasmPackage>,
}

impl FileList {
    pub fn new(package: Rc<WasmPackage>) -> Self {
        Self { package }
    }

    pub async fn get_files(&self, path: &str) -> Result<Vec<String>> {
        if path.is_empty() {
            Ok(ROOTS.iter().map(|&x| x.into()).collect::<Vec<_>>())
        } else if path.starts_with("exd") {
            self.get_exd_files(path).await
        } else {
            Ok(Vec::new())
        }
    }

    async fn get_exd_files(&self, path: &str) -> Result<Vec<String>> {
        if path == "exd" {
            let exl = ExList::new(&*self.package).await?;

            Ok(exl.ex_names)
        } else {
            Ok(Vec::new())
        }
    }
}
