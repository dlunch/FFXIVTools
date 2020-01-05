use super::archive::SqPackArchiveId;

pub struct SqPackFileReference {
    pub archive_id: SqPackArchiveId,
    pub path_hash: u32,
    pub folder_hash: u32,
    pub file_hash: u32,

    #[cfg(debug_assertions)]
    pub path: String,
}

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

impl SqPackFileReference {
    pub fn new(path: &str) -> Self {
        let path_str = path.to_ascii_lowercase();
        let folder_separator = path_str.rfind('/').unwrap();
        let folder_str = &path_str[..folder_separator];
        let file_str = &path_str[folder_separator + 1..];

        let path_hash = !Self::hash(&path_str);
        let folder_hash = !Self::hash(&folder_str);
        let file_hash = !Self::hash(&file_str);

        let path_splitted = path_str.split('/').collect::<Vec<&str>>();

        let root = ROOT_INDICES[path_splitted[0]];
        let mut ex = 0;
        let mut part = 0;

        if root == 2 || root == 3 || root == 12 {
            let ex_path = path_splitted[1];
            ex = if ex_path == "ffxiv" {
                0
            } else {
                ex_path[2..].parse().unwrap()
            };

            if root == 2 && ex > 0 {
                let part_path = path_splitted[2];
                if part_path.starts_with(char::is_numeric) {
                    part = part_path[..2].parse().unwrap();
                }
            }
        }

        Self {
            archive_id: SqPackArchiveId { root, ex, part },
            path_hash,
            folder_hash,
            file_hash,
            #[cfg(debug_assertions)]
            path: path_str,
        }
    }

    fn hash(value: &str) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(value.as_bytes());
        hasher.finalize()
    }
}
