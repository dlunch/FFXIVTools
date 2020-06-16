pub struct Equipment {
    pub(crate) model_id: u16,
    pub(crate) variant_id: u8,
    #[allow(dead_code)]
    pub(crate) stain_id: u8,
}

impl Equipment {
    pub fn new(model_id: u16, variant_id: u8, stain_id: u8) -> Self {
        Self {
            model_id,
            variant_id,
            stain_id,
        }
    }
}
