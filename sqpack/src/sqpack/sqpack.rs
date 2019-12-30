use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::package::Package;

use super::ext::ReadExt;
use super::parser::*;

#[derive(Eq, PartialEq, Hash, Default)]
struct SqPackArchiveId {
    root: u8,
    ex: u8,
    part: u8,
}

#[derive(Default)]
struct SqPackArchive {
    pub folder_segments: Vec<FolderSegment>,
    pub file_segments: Vec<FileSegment>,
    pub data: Vec<File>,
}

struct SqPackFileReference {
    archive_id: SqPackArchiveId,
    path_hash: u32,
    folder_hash: u32,
    file_hash: u32,

    #[cfg(debug_assertions)]
    path: String,
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
    pub fn new(path: &Path) -> Option<SqPackFileReference> {
        let path_str = path.to_str()?.to_ascii_lowercase();
        let folder_str = path.parent()?.to_str()?.to_ascii_lowercase();
        let file_str = path.file_name()?.to_str()?.to_ascii_lowercase();

        let path_hash = SqPackFileReference::hash(&path_str);
        let folder_hash = SqPackFileReference::hash(&folder_str);
        let file_hash = SqPackFileReference::hash(&file_str);

        let mut path_iter = path.iter();

        let root = ROOT_INDICES[path_iter.next()?.to_str()?];
        let mut ex = 0;
        let mut part = 0;

        if root == 2 || root == 3 || root == 12 {
            let ex_path = path_iter.next()?.to_str()?;
            ex = if ex_path == "ffxiv" {
                0
            } else {
                ex_path[2..].parse().unwrap()
            };

            if root == 2 && ex > 0 {
                let part_path = path_iter.next()?.to_str()?;
                if part_path.starts_with(char::is_numeric) {
                    part = part_path[..2].parse().unwrap();
                }
            }
        }

        Some(SqPackFileReference {
            archive_id: SqPackArchiveId { root, ex, part },
            path_hash,
            folder_hash,
            file_hash,
            #[cfg(debug_assertions)]
            path: path_str,
        })
    }

    fn hash(value: &str) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(value.as_bytes());
        hasher.finalize()
    }
}

macro_rules! read_segment {
    ($file: expr, $segment: expr, $type: ty) => {{
        let segment_count = $segment.size / <$type>::SIZE as u32;
        let data = $file.read_to_vec($segment.offset as u64, $segment.size as usize)?;
        let mut result = Vec::with_capacity(segment_count as usize);
        for i in 0..segment_count {
            let begin = (i as usize) * <$type>::SIZE;
            let end = begin + <$type>::SIZE;
            result.push(<$type>::parse(&data[begin..end]).unwrap().1);
        }

        result
    }};
}

pub struct SqPack {
    archives: HashMap<SqPackArchiveId, SqPackArchive>,
}

impl SqPack {
    pub fn new() -> SqPack {
        SqPack {
            archives: HashMap::new(),
        }
    }

    pub fn mount(&mut self, path: &Path) -> io::Result<()> {
        let path_str = path.to_str().unwrap();
        let index_path = format!("{}.index", path_str);
        let mut f = File::open(index_path)?;

        let sqpack_header_data = f.read_to_vec(0, SqPackHeader::SIZE)?;
        let sqpack_header = SqPackHeader::parse(&sqpack_header_data).unwrap().1;

        let index_header_data =
            f.read_to_vec(sqpack_header.header_length as u64, SqPackIndexHeader::SIZE)?;
        let index_header = SqPackIndexHeader::parse(&index_header_data).unwrap().1;

        let folder_segments = read_segment!(f, index_header.folder_segment, FolderSegment);
        let file_segments = read_segment!(f, index_header.file_segment, FileSegment);

        let mut data = Vec::with_capacity(index_header.dat_count as usize);
        for i in 0..index_header.dat_count {
            let dat_path = format!("{}.dat{}", path_str, i);
            data.push(File::open(dat_path)?);
        }

        self.archives.insert(
            SqPack::get_archive_id(path),
            SqPackArchive {
                folder_segments,
                file_segments,
                data,
            },
        );

        Ok(())
    }

    fn do_read_file(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn get_archive_id(path: &Path) -> SqPackArchiveId {
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let archive_id = u32::from_str_radix(file_name, 16).unwrap();

        // ex) 0a0000
        let root = (archive_id / 0x10000) as u8;
        let ex = ((archive_id / 0x100) & 0xff) as u8;
        let part = (archive_id & 0xff) as u8;

        SqPackArchiveId { root, ex, part }
    }
}

impl Package for SqPack {
    fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.do_read_file(&SqPackFileReference::new(path).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::SqPack;
    use crate::package::Package;
    use std::path::Path;
    #[test]
    fn test_read() {
        let mut pack = SqPack::new();
        pack.mount(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ffxiv\\0a0000.win32",
        ))
        .unwrap();
        pack.mount(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ex1\\020101.win32",
        ))
        .unwrap();

        pack.read_file(Path::new("exd/item.exd")).unwrap();
        pack.read_file(Path::new(
            "bg/ex1/01_roc_02/common/bgparts/r200_a0_bari1.mdl",
        ))
        .unwrap();
    }
}
