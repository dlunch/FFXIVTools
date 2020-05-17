use alloc::vec::Vec;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct SqPackArchiveId {
    pub root: u8,
    pub ex: u8,
    pub part: u8,
}

impl SqPackArchiveId {
    pub fn from_sqpack_file_name(file_name: &str) -> Self {
        let archive_id_str = file_name.split('.').next().unwrap();
        let archive_id = u32::from_str_radix(archive_id_str, 16).unwrap();

        // ex) 0a0000
        let root = (archive_id / 0x10000) as u8;
        let ex = ((archive_id / 0x100) & 0xff) as u8;
        let part = (archive_id & 0xff) as u8;

        Self { root, ex, part }
    }

    pub fn from_file_path(path: &str) -> Self {
        let path_splitted = path.split('/').collect::<Vec<_>>();

        let root = Self::path_to_root_index(path_splitted[0]);
        let mut ex = 0;
        let mut part = 0;

        if root == Self::path_to_root_index("bg") || root == Self::path_to_root_index("cut") || root == Self::path_to_root_index("music") {
            let ex_path = path_splitted[1];
            ex = if ex_path == "ffxiv" { 0 } else { ex_path[2..].parse().unwrap() };

            if root == 2 && ex > 0 {
                let part_path = path_splitted[2];
                if part_path.starts_with(char::is_numeric) {
                    part = part_path[..2].parse().unwrap();
                }
            }
        };

        Self { root, ex, part }
    }

    fn path_to_root_index(root: &str) -> u8 {
        match root {
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
            _ => panic!(),
        }
    }
}
