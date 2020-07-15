#![no_std]
extern crate alloc;

mod classjob;

pub use classjob::ClassJob;

use core::marker::PhantomData;

use ffxiv_parser::{Ex, ExRow, Language};

pub trait WrappedExRow<'a> {
    fn new(raw: ExRow<'a>) -> Self;
}

pub struct WrappedEx<'a, T: WrappedExRow<'a>> {
    raw: Ex,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: WrappedExRow<'a>> WrappedEx<'a, T> {
    // reading ex here causes https://github.com/rust-lang/rust/issues/63033
    pub fn new(raw: Ex) -> Self {
        Self { raw, phantom: PhantomData }
    }

    pub fn index(&'a self, index: u32, language: Language) -> Option<T> {
        Some(T::new(self.raw.index(index, language)?))
    }
}
