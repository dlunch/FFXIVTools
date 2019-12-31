#[derive(Eq, PartialEq, Hash, Default)]
pub struct SqPackArchiveId {
    pub root: u8,
    pub ex: u8,
    pub part: u8,
}
