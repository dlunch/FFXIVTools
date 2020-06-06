use core::mem::size_of;

use util::round_up;

#[derive(Clone)]
pub struct ByteReader<'a> {
    data: &'a [u8],
    cursor: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn read(&mut self) -> u8 {
        let result = self.data[self.cursor];
        self.cursor += 1;

        result
    }

    pub fn read_u16_le(&mut self) -> u16 {
        let result = u16::from_le_bytes([self.data[self.cursor], self.data[self.cursor + 1]]);
        self.cursor += size_of::<u16>();

        result
    }

    pub fn read_f32_le(&mut self) -> f32 {
        let result = f32::from_le_bytes([
            self.data[self.cursor],
            self.data[self.cursor + 1],
            self.data[self.cursor + 2],
            self.data[self.cursor + 3],
        ]);
        self.cursor += size_of::<f32>();

        result
    }

    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        let result = &self.data[self.cursor..self.cursor + size];
        self.cursor += size;

        result
    }

    pub fn align(&mut self, align: usize) {
        self.cursor = round_up(self.cursor, align)
    }

    pub fn raw(&self) -> &[u8] {
        &self.data[self.cursor..]
    }

    pub fn seek(&mut self, offset: usize) {
        self.cursor += offset;
    }
}
