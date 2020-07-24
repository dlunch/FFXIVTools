use alloc::string::String;

use ffxiv_parser::ExRow;

use crate::{NamedExRow, WrappedExRow};

pub struct Quest<'a> {
    raw: ExRow<'a>,
}

impl<'a> NamedExRow<'a> for Quest<'a> {
    fn name(&self) -> String {
        self.raw.string(0).decode()
    }
}

impl<'a> WrappedExRow<'a> for Quest<'a> {
    fn new(raw: ExRow<'a>) -> Self {
        Self { raw }
    }

    fn ex_name() -> &'static str {
        "quest"
    }
}
