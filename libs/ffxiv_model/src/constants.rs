use enum_iterator::IntoEnumIterator;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
#[repr(u8)]
pub enum ModelPart {
    Met,
    Top,
    Glv,
    Dwn,
    Sho,
    Ear,
    Nek,
    Wrs,
    Rir,
    Ril,
    Hir,
    Fac,
}

impl ModelPart {
    pub fn as_path_str(&self) -> &'static str {
        match self {
            ModelPart::Met => "met",
            ModelPart::Top => "top",
            ModelPart::Glv => "glv",
            ModelPart::Dwn => "dwn",
            ModelPart::Sho => "sho",
            ModelPart::Ear => "ear",
            ModelPart::Nek => "nek",
            ModelPart::Wrs => "wrs",
            ModelPart::Rir => "rir",
            ModelPart::Ril => "ril",
            ModelPart::Hir => "hir",
            ModelPart::Fac => "fac",
        }
    }
}

#[derive(Clone, Copy, IntoEnumIterator, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum BodyId {
    MidlanderMale = 101,
    MidlanderFemale = 201,
    HighlanderMale = 301,
    HighlanderFemale = 401,
    ElezenMale = 501,
    ElezenFemale = 601,
    MiqoteMale = 701,
    MiqoteFemale = 801,
    RoegadynMale = 901,
    RoegadynFemale = 1001,
    LalafellMale = 1101,
    LalafellFemale = 1201,
    AuRaMale = 1301,
    AuRaFemale = 1401,
    HrothgarMale = 1501,
    HrothgarFemale = 1601,
    VieraMale = 1701,
    VieraFemale = 1801,

    ChildHyurMale = 104,
    ChildHyurFemale = 204,
    ChildElezenMale = 504,
    ChildElezenFemale = 604,
    ChildMiqoteMale = 704,
    ChildMiqoteFemale = 804,
    ChildAuRaMale = 1304,
    ChildAuRaFemale = 1404,
    Unk9104 = 9104,
    Unk9204 = 9204,
}

impl BodyId {
    pub fn is_male(self) -> bool {
        (self as u16 % 100) == 1
    }

    pub fn is_child(self) -> bool {
        (self as u16 % 100) == 4
    }
}
