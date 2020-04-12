use core::mem::size_of;
use std::io;
use std::path::Path;

use tokio::fs;
use zerocopy::LayoutVerified;

use util::cast;

use super::definition::{FileSegment, FolderSegment, SqPackHeader, SqPackIndexHeader};
use crate::error::{Result, SqPackReaderError};

pub struct SqPackIndex {
    pub dat_count: u32,
    folder_segments: Vec<FolderSegment>,
    file_segments: Vec<FileSegment>,
    file_segment_base: u32,
}

macro_rules! get_segment {
    ($data: expr, $segment: expr, $type: ty) => {{
        let segment_count = $segment.size as usize / size_of::<$type>();
        (0..segment_count)
            .map(|x| cast!($data[$segment.offset as usize + x * size_of::<$type>()..], $type).clone())
            .collect::<Vec<_>>()
    }};
}

impl SqPackIndex {
    pub async fn new(path: &Path) -> io::Result<Self> {
        let data = fs::read(path).await?;

        let sqpack_header = cast!(data, SqPackHeader);
        let index_header = cast!(&data[sqpack_header.header_length as usize..], SqPackIndexHeader);

        let folder_segments = get_segment!(data, index_header.folder_segment, FolderSegment);
        let file_segments = get_segment!(data, index_header.file_segment, FileSegment);

        Ok(Self {
            folder_segments,
            file_segments,
            file_segment_base: index_header.file_segment.offset,
            dat_count: index_header.dat_count,
        })
    }

    pub fn find_offset(&self, folder_hash: u32, file_hash: u32) -> Result<u32> {
        let folder_index = self
            .folder_segments
            .binary_search_by_key(&folder_hash, |x| x.folder_hash)
            .map_err(|_| SqPackReaderError::NoSuchFolder)?;
        let folder = &self.folder_segments[folder_index];

        let file_begin = (folder.file_list_offset - self.file_segment_base) as usize / size_of::<FileSegment>();
        let file_end = file_begin + folder.file_list_size as usize / size_of::<FileSegment>();
        let file_index = self.file_segments[file_begin..file_end]
            .binary_search_by_key(&file_hash, |x| x.file_hash)
            .map_err(|_| SqPackReaderError::NoSuchFile)?;
        let file = &self.file_segments[file_index + file_begin];

        Ok(file.data_offset)
    }

    pub fn folders<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.folder_segments.iter().map(|x| x.folder_hash)
    }

    pub fn files<'a>(&'a self, folder_hash: u32) -> Result<impl Iterator<Item = u32> + 'a> {
        let folder_index = self
            .folder_segments
            .binary_search_by_key(&folder_hash, |x| x.folder_hash)
            .map_err(|_| SqPackReaderError::NoSuchFolder)?;
        let folder = &self.folder_segments[folder_index];

        let file_begin = (folder.file_list_offset - self.file_segment_base) as usize / size_of::<FileSegment>();
        let file_end = file_begin + folder.file_list_size as usize / size_of::<FileSegment>();

        Ok(self.file_segments[file_begin..file_end].iter().map(|x| x.file_hash))
    }
}
