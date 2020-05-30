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

    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        let result = &self.data[self.cursor..self.cursor + size];
        self.cursor += size;

        result
    }
}
