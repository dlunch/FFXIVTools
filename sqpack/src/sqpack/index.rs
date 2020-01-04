use std::fs::File;
use std::io;
use std::path::Path;

use super::ext::ReadExt;
use super::parser::*;
use super::reference::SqPackFileReference;

pub struct SqPackIndex {
    pub dat_count: u32,
    folder_segments: Vec<FolderSegment>,
    file_segments: Vec<FileSegment>,
    file_segment_base: u32,
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

impl SqPackIndex {
    pub fn new(path: &Path) -> io::Result<SqPackIndex> {
        let mut f = File::open(path)?;

        let sqpack_header_data = f.read_to_vec(0, SqPackHeader::SIZE)?;
        let sqpack_header = SqPackHeader::parse(&sqpack_header_data).unwrap().1;

        let index_header_data =
            f.read_to_vec(sqpack_header.header_length as u64, SqPackIndexHeader::SIZE)?;
        let index_header = SqPackIndexHeader::parse(&index_header_data).unwrap().1;

        let folder_segments = read_segment!(f, index_header.folder_segment, FolderSegment);
        let file_segments = read_segment!(f, index_header.file_segment, FileSegment);

        Ok(SqPackIndex {
            folder_segments,
            file_segments,
            file_segment_base: index_header.file_segment.offset,
            dat_count: index_header.dat_count,
        })
    }

    pub fn find_offset(&self, reference: &SqPackFileReference) -> io::Result<u32> {
        let folder_index = self
            .folder_segments
            .binary_search_by_key(&reference.folder_hash, |x| x.folder_hash)
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "No such folder"))?;
        let folder = &self.folder_segments[folder_index];

        let file_begin =
            (folder.file_list_offset - self.file_segment_base) as usize / FileSegment::SIZE;
        let file_end = file_begin + folder.file_list_size as usize / FileSegment::SIZE;
        let file_index = self.file_segments[file_begin..file_end]
            .binary_search_by_key(&reference.file_hash, |x| x.file_hash)
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "No such file"))?;
        let file = &self.file_segments[file_index + file_begin];

        Ok(file.data_offset)
    }
}
