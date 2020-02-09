use std::ffi::OsStr;
use std::path::Path;

#[allow(dead_code)] // rustc bug?
static ROOT_INDICES: phf::Map<&'static str, u8> = phf_map! {
    "common" => 0,
    "bgcommon" => 1,
    "bg" => 2,
    "cut" => 3,
    "chara" => 4,
    "shader" => 5,
    "ui" => 6,
    "sound" => 7,
    "vfx" => 8,
    "ui_script" => 9,
    "exd" => 10,
    "game_script" => 11,
    "music" => 12,
};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct SqPackArchiveId {
    pub root: u8,
    pub ex: u8,
    pub part: u8,
}

impl SqPackArchiveId {
    pub fn with_sqpack_path(path: &Path) -> Self {
        let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
        let archive_id_str = file_name.split('.').next().unwrap();
        let archive_id = u32::from_str_radix(archive_id_str, 16).unwrap();

        // ex) 0a0000
        let root = (archive_id / 0x10000) as u8;
        let ex = ((archive_id / 0x100) & 0xff) as u8;
        let part = (archive_id & 0xff) as u8;

        SqPackArchiveId { root, ex, part }
    }

    pub fn with_file_path(path: &str) -> Self {
        let path_splitted = path.split('/').collect::<Vec<_>>();

        let root = ROOT_INDICES[path_splitted[0]];
        let mut ex = 0;
        let mut part = 0;

        if root == 2 || root == 3 || root == 12 {
            let ex_path = path_splitted[1];
            ex = if ex_path == "ffxiv" { 0 } else { ex_path[2..].parse().unwrap() };

            if root == 2 && ex > 0 {
                let part_path = path_splitted[2];
                if part_path.starts_with(char::is_numeric) {
                    part = part_path[..2].parse().unwrap();
                }
            }
        };

        SqPackArchiveId { root, ex, part }
    }
}
