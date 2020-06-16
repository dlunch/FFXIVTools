use crate::constants::BodyId;

pub struct Customization {
    pub(crate) body_id: BodyId,
    pub(crate) body_type: u8,
    pub(crate) body_variant_id: u8,
    pub(crate) face_id: u8,
    pub(crate) hair_id: u8,
    pub(crate) hair_variant_id: u8,
}

impl Customization {
    pub fn new(body_id: BodyId, body_type: u8, body_variant_id: u8, face_id: u8, hair_id: u8, hair_variant_id: u8) -> Self {
        Self {
            body_id,
            body_type,
            body_variant_id,
            face_id,
            hair_id,
            hair_variant_id,
        }
    }
}
