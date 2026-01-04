use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc, SharedString};
use std::collections::HashMap;

use super::devs::Devs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortByDateDto {
    pub total: i32,
    pub remains: i32,
    pub dev: Devs,
    pub project: i32,
    pub effort: i32,
    pub week: i32,
    pub persons: Vec<String>,
}

impl EffortByDateDto {
    pub fn get_total(&self) -> i32 {
        let mut total = 0;
        for item in self.persons.iter() {
            if let Some((_, value)) = crate::utils::info_cell(item) {
                let value = crate::utils::get_hours(value, 40);
                total += value;
            }
        }
        total
    }
}

// Conversion implementations for EffortByDateData (from Slint)
impl From<EffortByDateDto> for crate::EffortByDateData {
    fn from(d: EffortByDateDto) -> Self {
        Self {
            total: d.total,
            remains: d.remains,
            dev: d.dev.into(),
            project: d.project,
            effort: d.effort,
            week: d.week,
            persons: ModelRc::new(slint::VecModel::from(
                d.persons.iter().map(SharedString::from).collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<crate::EffortByDateData> for EffortByDateDto {
    fn from(d: crate::EffortByDateData) -> Self {
        Self {
            total: d.total,
            remains: d.remains,
            dev: d.dev.into(),
            project: d.project,
            effort: d.effort,
            week: d.week,
            persons: d.persons.iter().map(|s| s.to_string()).collect(),
        }
    }
}

// Extension methods for EffortByDateData
pub trait EffortByDateDataExt {
    fn get_sovra(&self, sovra: &mut HashMap<String, i32>);
}

impl EffortByDateDataExt for crate::EffortByDateData {
    fn get_sovra(&self, sovra: &mut HashMap<String, i32>) {
        for item in self.persons.iter() {
            if let Some((person, value)) = crate::utils::info_cell(item.as_str()) {
                if let Some(old_value) = sovra.get(person) {
                    sovra.insert(String::from(person), old_value + value);
                } else {
                    sovra.insert(String::from(person), value);
                }
            }
        }
    }
}
