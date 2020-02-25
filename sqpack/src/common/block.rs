use compression::prelude::DecodeExt;
use compression::prelude::Deflater;
use nom::number::complete::le_u32;
use nom::{do_parse, named};

pub struct BlockHeader {
    pub header_size: u32,
    pub compressed_length: u32, // 32000 if not compressed
    pub uncompressed_length: u32,
}

impl BlockHeader {
    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            header_size:            le_u32  >>
            _unk:                   le_u32  >>
            compressed_length:      le_u32  >>
            uncompressed_length:    le_u32  >>
            (Self {
                header_size,
                compressed_length,
                uncompressed_length,
            })
        )
    );
}

pub fn decode_block_into<'a>(data: &'a [u8], result: &mut Vec<u8>) -> usize {
    let header = BlockHeader::parse(data).unwrap().1;

    if header.compressed_length >= 32000 {
        let end = header.header_size as usize + header.uncompressed_length as usize;
        let data = &data[header.header_size as usize..end];
        result.extend(data);

        end
    } else {
        let end = header.header_size as usize + header.compressed_length as usize;
        let data = &data[header.header_size as usize..end];
        result.extend(data.iter().cloned().decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap());

        header.header_size as usize + header.compressed_length as usize
    }
}
