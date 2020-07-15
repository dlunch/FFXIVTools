use alloc::string::String;

use ffxiv_parser::ExRow;

use crate::WrappedExRow;

pub struct ClassJob<'a> {
    raw: ExRow<'a>,
}

impl<'a> ClassJob<'a> {
    pub fn name(&self) -> String {
        self.raw.string(0).decode()
    }
}

impl<'a> WrappedExRow<'a> for ClassJob<'a> {
    fn new(raw: ExRow<'a>) -> Self {
        Self { raw }
    }
}
