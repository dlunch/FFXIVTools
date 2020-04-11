use alloc::{str, string::String};
use core::convert::Into;

pub struct FFXIVString<'a> {
    data: &'a [u8],
}

impl<'a> FFXIVString<'a> {
    const MARKUP_START: u8 = b'\x02';
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn decode(&'a self) -> String {
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

    fn next_item(&self, cursor: &mut usize) -> u8 {
        let item = self.data[*cursor];
        *cursor += 1;

        item
    }

    fn next_size(&self, cursor: &mut usize) -> usize {
        let mut next = || self.next_item(cursor) as usize;
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
                let payload = self.next_item(cursor);
                match payload {
                    2 => "<i>",
                    1 => "</i>",
                    _ => panic!(),
                }
                .to_owned()
            }
            0x20 => {
                let payload = self.next_item(cursor);
                (payload - 1).to_string()
            }
            _ => {
                let payload = &self.data[*cursor..*cursor + markup_size];
                format!(
                    "<Unknown payload=\"{}\" />",
                    payload.iter().map(|x| format!("{:x}", x)).collect::<String>()
                )
            }
        };

        let end = self.next_item(cursor);
        debug_assert_eq!(end, 0x03);

        result
    }
}

impl<'a> Into<String> for FFXIVString<'a> {
    fn into(self) -> String {
        self.decode()
    }
}
