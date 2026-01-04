use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc, SharedString};

use super::effort_by_prj::EffortByPrjDto;
use super::sovra::SovraDto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortsDto {
    pub sovra: Vec<SovraDto>,
    pub week_off: Vec<i32>,
    pub worker_names: Vec<String>,
    pub projects: Vec<EffortByPrjDto>,
}

impl Default for EffortsDto {
    fn default() -> Self {
        let (_, start_week, end_week) = crate::date_utils::get_default_weeks();
        let mut start_week = start_week;
        let mut sovra = Vec::new();

        while start_week < end_week {
            sovra.push(SovraDto {
                value: vec![],
                week: start_week,
            });
            start_week += 7;
        }
        Self {
            sovra,
            week_off: vec![],
            worker_names: vec![],
            projects: vec![EffortByPrjDto::new(0)],
        }
    }
}

impl EffortsDto {
    pub fn start_end_weeks(&self) -> (i32, i32) {
        use chrono::Utc;

        let start_week = self
            .projects
            .iter()
            .map(|d| d.start_week)
            .min()
            .unwrap_or(crate::date_utils::local_to_days(
                &(Utc::now().date_naive() - chrono::Duration::days(30)),
            ));
        let end_week = self
            .projects
            .iter()
            .map(|d| d.end_week)
            .max()
            .unwrap_or(crate::date_utils::local_to_days(
                &(Utc::now().date_naive() + chrono::Duration::days(365)),
            ));

        (start_week, end_week)
    }
}

// Conversion implementations for EffortsData (from Slint)
impl From<EffortsDto> for crate::EffortsData {
    fn from(d: EffortsDto) -> Self {
        Self {
            week_off: ModelRc::new(slint::VecModel::from(d.week_off)),
            sovra: ModelRc::new(slint::VecModel::from(
                d.sovra
                    .into_iter()
                    .map(crate::SovraData::from)
                    .collect::<Vec<_>>(),
            )),
            worker_names: ModelRc::new(slint::VecModel::from(
                d.worker_names
                    .iter()
                    .map(SharedString::from)
                    .collect::<Vec<_>>(),
            )),
            projects: ModelRc::new(slint::VecModel::from(
                d.projects
                    .into_iter()
                    .map(crate::EffortByPrjData::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<crate::EffortsData> for EffortsDto {
    fn from(d: crate::EffortsData) -> Self {
        Self {
            sovra: d.sovra.iter().map(SovraDto::from).collect(),
            week_off: d.week_off.iter().collect(),
            worker_names: d.worker_names.iter().map(|s| s.to_string()).collect(),
            projects: d.projects.iter().map(EffortByPrjDto::from).collect(),
        }
    }
}
