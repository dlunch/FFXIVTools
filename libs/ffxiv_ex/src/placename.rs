use alloc::string::String;

use ffxiv_parser::ExRow;

use crate::{NamedExRow, WrappedExRow};

pub struct PlaceName<'a> {
    raw: ExRow<'a>,
}

impl<'a> NamedExRow<'a> for PlaceName<'a> {
    fn name(&self) -> String {
        self.raw.string(0).decode()
    }
}

impl<'a> WrappedExRow<'a> for PlaceName<'a> {
    fn new(raw: ExRow<'a>) -> Self {
        Self { raw }
    }

    fn ex_name() -> &'static str {
        "placename"
    }
}
