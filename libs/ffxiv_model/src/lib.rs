mod character;
mod character_part;
mod model_read_context;
mod shader_holder;
mod type_adapter;

pub use character::Character;
pub use shader_holder::ShaderHolder; // TODO move this to internal state manager

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
    pub fn as_str(&self) -> &'static str {
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
