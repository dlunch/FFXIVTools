use std::rc::Rc;

use common::WasmPackage;

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

    pub fn get_files(&self, path: &str) -> Vec<String> {
        if path.is_empty() {
            ROOTS.iter().map(|&x| x.into()).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }
}
