use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SovraDto {
    pub value: Vec<i32>,
    pub week: i32,
}

// Conversion implementations for SovraData (from Slint)
impl From<SovraDto> for crate::SovraData {
    fn from(d: SovraDto) -> Self {
        Self {
            value: ModelRc::new(slint::VecModel::from(d.value)),
            week: d.week,
        }
    }
}

impl From<crate::SovraData> for SovraDto {
    fn from(d: crate::SovraData) -> Self {
        Self {
            value: d.value.iter().collect(),
            week: d.week,
        }
    }
}
