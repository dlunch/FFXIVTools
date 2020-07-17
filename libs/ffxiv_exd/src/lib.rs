#![no_std]
extern crate alloc;

mod classjob;

pub use classjob::ClassJob;

use alloc::{string::String, collections::BTreeMap};
use core::marker::PhantomData;

use ffxiv_parser::{Ex, ExRow, Language};
use sqpack_reader::{Package, Result};

pub trait WrappedExRow<'a> {
    fn new(raw: ExRow<'a>) -> Self;
    fn ex_name() -> &'static str;
}

pub trait NamedExRow {
    fn name(&self) -> String;
}

pub struct WrappedEx<'a, T: WrappedExRow<'a>> {
    raw: Ex,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: WrappedExRow<'a> + 'a> WrappedEx<'a, T> {
    pub async fn new(pack: &dyn Package) -> Result<WrappedEx<'a, T>> {
        let raw = Ex::new(pack, T::ex_name()).await?;

        Ok(Self { raw, phantom: PhantomData })
    }

    pub fn index(&'a self, index: u32, language: Language) -> Option<T> {
        Some(T::new(self.raw.index(index, language)?))
    }

    pub fn all(&'a self, language: Language) -> Option<BTreeMap<u32, T>> {
        Some(self.raw.all(language)?.into_iter().map(|(key, value)| (key, T::new(value))).collect::<BTreeMap<_, _>>())
    }
}
