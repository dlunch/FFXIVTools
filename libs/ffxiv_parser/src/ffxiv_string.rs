use alloc::{
    borrow::ToOwned,
    format, str,
    string::{String, ToString},
};

pub struct FfxivString<'a> {
    data: &'a [u8],
}

impl<'a> FfxivString<'a> {
    const MARKUP_START: u8 = b'\x02';
    pub fn new(data: &'a [u8]) -> Self {
        let end = data.iter().position(|&x| x == b'\0').unwrap() + 1;

        Self { data: &data[0..end] }
    }

    pub fn decode(&self) -> String {
        let mut result = String::with_capacity(self.data.len());
        let mut cursor = 0;

        while cursor < self.data.len() && self.data[cursor] != 0 {
            if self.data[cursor] == Self::MARKUP_START {
                result.push_str(&self.next_markup(&mut cursor));
            } else {
                result.push_str(self.next_str(&mut cursor));
            }
        }

        result
    }

    fn next_str(&self, cursor: &mut usize) -> &str {
        let next_offset = self.data[*cursor..].iter().position(|&x| x == Self::MARKUP_START || x == 0).unwrap();
        let result = str::from_utf8(&self.data[*cursor..next_offset + *cursor]).unwrap();
        *cursor += next_offset;

        result
    }

    fn next_byte(&self, cursor: &mut usize) -> u8 {
        let byte = self.data[*cursor];
        *cursor += 1;

        byte
    }

    fn next_size(&self, cursor: &mut usize) -> usize {
        let mut next = || self.next_byte(cursor) as usize;
        let item = next();

        match item {
            0..=0xEF => item - 1,
            0xF0 => next(),
            0xF1 => ((next() << 8) | next()) - 1,
            0xF2 => (next() << 8) | next(),
            0xFA => (next() << 16) | (next() << 8) | next(),
            0xFE => (next() << 24) | (next() << 16) | (next() << 8) | next(),
            _ => panic!(),
        }
    }

    fn next_markup(&self, cursor: &mut usize) -> String {
        let markup_type = self.data[*cursor + 1];
        *cursor += 2;
        let markup_size = self.next_size(cursor);

        let result = match markup_type {
            0x10 => "\n".to_owned(),
            0x16 => "\u{00AD}".to_owned(), // soft hyphen
            0x1A => {
                let payload = self.next_byte(cursor);
                match payload {
                    2 => "<i>",
                    1 => "</i>",
                    _ => panic!(),
                }
                .to_owned()
            }
            0x20 => {
                let payload = self.next_byte(cursor);
                (payload - 1).to_string()
            }
            _ => {
                let payload = &self.data[*cursor..*cursor + markup_size];
                *cursor += markup_size;
                format!("<Unknown type=\"{}\" payload=\"{:?}\" />", markup_type, payload)
            }
        };

        let end = self.next_byte(cursor);
        debug_assert_eq!(end, 0x03);

        result
    }
}

impl<'a> From<FfxivString<'a>> for String {
    fn from(s: FfxivString<'a>) -> String {
        s.decode()
    }
}

#[cfg(test)]
mod test {
    use super::FfxivString;

    #[test]
    fn test_string() {
        let raw = [
            73, 110, 99, 114, 101, 97, 115, 101, 115, 32, 69, 88, 80, 32, 101, 97, 114, 110, 101, 100, 32, 102, 114, 111, 109, 32, 98, 97, 116, 116,
            108, 101, 44, 32, 99, 114, 97, 102, 116, 105, 110, 103, 44, 32, 97, 110, 100, 32, 103, 97, 116, 104, 101, 114, 105, 110, 103, 32, 119,
            104, 101, 110, 32, 108, 101, 118, 101, 108, 32, 49, 48, 32, 111, 114, 32, 98, 101, 108, 111, 119, 46, 2, 16, 1, 3, 2, 72, 4, 242, 1, 248,
            3, 2, 73, 4, 242, 1, 249, 3, 69, 88, 80, 32, 66, 111, 110, 117, 115, 58, 2, 73, 2, 1, 3, 2, 72, 2, 1, 3, 32, 43, 50, 48, 37, 0,
        ];
        let ffxiv_string = FfxivString::new(&raw);
        let result = ffxiv_string.decode();
        assert_eq!(
            result,
            "Increases EXP earned from battle, crafting, and gathering when level 10 or below.\n<Unknown type=\"72\" payload=\"[242, 1, 248]\" /><Unknown type=\"73\" payload=\"[242, 1, 249]\" />EXP Bonus:<Unknown type=\"73\" payload=\"[1]\" /><Unknown type=\"72\" payload=\"[1]\" /> +20%"
        );
    }
}
