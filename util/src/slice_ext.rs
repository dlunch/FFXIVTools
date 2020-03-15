use std::convert::TryInto;

pub trait SliceByteOrderExt {
    fn read_u32_be(&self) -> u32;
}

impl SliceByteOrderExt for &[u8] {
    fn read_u32_be(&self) -> u32 {
        let sliced = &self[..std::mem::size_of::<u32>()];

        u32::from_be_bytes(sliced.try_into().unwrap())
    }
}
