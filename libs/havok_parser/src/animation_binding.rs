use std::cell::RefCell;
use std::sync::Arc;

use crate::object::HavokObject;

#[repr(u8)]
pub enum HavokAnimationBlendHint {
    Normal = 0,
    Additive = 1,
}

impl HavokAnimationBlendHint {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Normal,
            1 => Self::Additive,
            _ => panic!(),
        }
    }
}

pub struct HavokAnimationBinding {
    pub transform_track_to_bone_indices: Vec<u16>,
    pub blend_hint: HavokAnimationBlendHint,
}

impl HavokAnimationBinding {
    pub fn new(object: Arc<RefCell<HavokObject>>) -> Self {
        let root = object.borrow();

        let raw_transform_track_to_bone_indices = root.get("transformTrackToBoneIndices").as_array();
        let transform_track_to_bone_indices = raw_transform_track_to_bone_indices.iter().map(|x| *x.as_int() as u16).collect::<Vec<_>>();

        let blend_hint = HavokAnimationBlendHint::from_raw(*root.get("blendHint").as_int() as u8);

        Self {
            transform_track_to_bone_indices,
            blend_hint,
        }
    }
}
