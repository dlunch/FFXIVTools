use alloc::{str, string::String};
use core::convert::Into;

pub struct FFXIVString<'a> {
    data: &'a [u8],
}

impl<'a> FFXIVString<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn decode(&'a self) -> String {
        self.iter().collect::<String>()
    }

    pub fn iter(&'a self) -> impl Iterator<Item = String> + 'a {
        FFXIVStringIterator::new(self.data)
    }
}

impl<'a> Into<String> for FFXIVString<'a> {
    fn into(self) -> String {
        self.decode()
    }
}

struct FFXIVStringIterator<'a> {
    data: &'a [u8],
    cursor: usize,
}

impl<'a> FFXIVStringIterator<'a> {
    const MARKUP_START: u8 = b'\x02';

    pub fn new(data: &'a [u8]) -> Self {
        Self { data, cursor: 0 }
    }

    fn next_markup(&mut self) -> String {
        let markup_type = self.data[self.cursor + 1];
        self.cursor += 2;
        let markup_size = self.next_size();

        let result = match markup_type {
            0x10 => "\n".to_owned(),
            0x16 => "\u{00AD}".to_owned(), // soft hyphen
            0x1A => {
                let payload = self.next_item();
                match payload {
                    2 => "<i>",
                    1 => "</i>",
                    _ => panic!(),
                }
                .to_owned()
            }
            0x20 => {
                let payload = self.next_item();
                (payload - 1).to_string()
            }
            _ => {
                let payload = &self.data[self.cursor..self.cursor + markup_size];
                format!(
                    "<Unknown payload=\"{}\" />",
                    payload.iter().map(|x| format!("{:x}", x)).collect::<String>()
                )
            }
        };

        self.next_item(); // 0x03

        result
    }

    fn next_size(&mut self) -> usize {
        let mut next = || self.next_item() as usize;
        let item = next();

        match item {
            0..=0xEF => item - 1,
            0xF0 => next() as usize,
            0xF1 => ((next() << 8) | next()) - 1,
            0xF2 => (next() << 8) | next(),
            0xFA => (next() << 16) | (next() << 8) | next(),
            0xFE => (next() << 24) | (next() << 16) | (next() << 8) | next(),
            _ => panic!(),
        }
    }

    fn next_item(&mut self) -> u8 {
        let item = self.data[self.cursor];
        self.cursor += 1;

        item
    }
}

impl<'a> Iterator for FFXIVStringIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.data.len() || self.data[self.cursor] == 0 {
            return None;
        }

        if self.data[self.cursor] == Self::MARKUP_START {
            return Some(self.next_markup());
        }

        let cursor = self.cursor;
        let next_offset = self.data[cursor..].iter().position(|&x| x == Self::MARKUP_START || x == 0);
        if let Some(next_offset) = next_offset {
            self.cursor += next_offset;
            Some(str::from_utf8(&self.data[cursor..next_offset + cursor]).unwrap().to_owned())
        } else {
            self.cursor = self.data.len();
            Some(str::from_utf8(&self.data[cursor..]).unwrap().to_owned())
        }
    }
}
