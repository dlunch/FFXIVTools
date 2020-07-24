use alloc::string::String;

use ffxiv_parser::ExRow;

use crate::{NamedExRow, WrappedExRow};

pub struct CraftAction<'a> {
    raw: ExRow<'a>,
}

impl<'a> NamedExRow<'a> for CraftAction<'a> {
    fn name(&self) -> String {
        self.raw.string(0).decode()
    }
}

impl<'a> WrappedExRow<'a> for CraftAction<'a> {
    fn new(raw: ExRow<'a>) -> Self {
        Self { raw }
    }

    fn ex_name() -> &'static str {
        "craftaction"
    }
}
