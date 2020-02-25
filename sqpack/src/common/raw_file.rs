use super::block::SqPackDataBlock;

pub struct SqPackRawFile {
    additional_header: Vec<u8>,
    blocks: Vec<SqPackDataBlock>,
}

impl SqPackRawFile {
    pub fn new(additional_header: Vec<u8>, blocks: Vec<SqPackDataBlock>) -> Self {
        SqPackRawFile { additional_header, blocks }
    }

    pub fn decode(mut self) -> Vec<u8> {
        self.additional_header
            .into_iter()
            .chain(self.blocks.drain(..).flat_map(|x| x.decode()))
            .collect()
    }
}
