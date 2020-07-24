#![no_std]
extern crate alloc;

mod action;
mod bnpcname;
mod classjob;
mod craftaction;
mod enpcresident;
mod item;
mod placename;
mod quest;

pub use action::Action;
pub use bnpcname::BNpcName;
pub use classjob::ClassJob;
pub use craftaction::CraftAction;
pub use enpcresident::ENpcResident;
pub use item::Item;
pub use placename::PlaceName;
pub use quest::Quest;

use alloc::string::String;
use core::marker::PhantomData;

use ffxiv_parser::{Ex, ExRow, Language};
use sqpack_reader::{Package, Result};

pub trait WrappedExRow<'a> {
    fn new(raw: ExRow<'a>) -> Self;
    fn ex_name() -> &'static str;
}

pub trait NamedExRow<'a>: WrappedExRow<'a> {
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

    pub fn all(&'a self, language: Language) -> Option<impl Iterator<Item = (u32, T)> + 'a> {
        Some(self.raw.all(language)?.map(|(key, value)| (key, T::new(value))))
    }
}
