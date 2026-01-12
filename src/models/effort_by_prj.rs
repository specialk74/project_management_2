use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc, SharedString};

use super::devs::{DevId, Devs};
use super::effort_by_dev::EffortByDevDto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortByPrjDto {
    pub text: String,
    pub start_week: i32,
    pub end_week: i32,
    pub project: i32,
    pub visible: bool,
    pub enable: bool,
    pub efforts: Vec<EffortByDevDto>,
}

impl EffortByPrjDto {
    pub fn new(project: i32) -> Self {
        let (num_weeks, start_date, end_date) = crate::date_utils::get_default_weeks();
        EffortByPrjDto {
            project,
            start_week: start_date,
            end_week: end_date,
            visible: true,
            enable: true,
            text: "New Project".to_string(),
            efforts: vec![
                EffortByDevDto::new(Devs::Mcsw, project, num_weeks),
                EffortByDevDto::new(Devs::Sms, project, num_weeks),
                EffortByDevDto::new(Devs::Mvh, project, num_weeks),
                EffortByDevDto::new(Devs::Hw, project, num_weeks),
                EffortByDevDto::new(Devs::Ele, project, num_weeks),
                EffortByDevDto::new(Devs::TestHw, project, num_weeks),
                EffortByDevDto::new(Devs::TestFw, project, num_weeks),
                EffortByDevDto::new(Devs::TestSys, project, num_weeks),
                EffortByDevDto::new(Devs::Pjm, project, num_weeks),
            ],
        }
    }

    pub fn set_date(&mut self, start_week: i32, end_week: i32) {
        if self.start_week > start_week {
            let diff = (self.start_week - start_week) / 7;
            for dev in self.efforts.iter_mut() {
                dev.prepend_weeks(diff, start_week);
            }
            self.start_week = start_week;
        }

        if self.end_week < end_week {
            let diff = (end_week - self.end_week) / 7;
            for dev in self.efforts.iter_mut() {
                dev.append_weeks(diff, start_week);
            }
            self.end_week = end_week
        }
    }
}

// Conversion implementations for EffortByPrjData (from Slint)
impl From<EffortByPrjDto> for crate::EffortByPrjData {
    fn from(d: EffortByPrjDto) -> Self {
        Self {
            start_week: d.start_week,
            end_week: d.end_week,
            text: SharedString::from(d.text.clone()),
            project: d.project,
            visible: d.visible,
            enable: d.enable,
            efforts: ModelRc::new(slint::VecModel::from(
                d.efforts
                    .into_iter()
                    .map(crate::EffortByDevData::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<crate::EffortByPrjData> for EffortByPrjDto {
    fn from(d: crate::EffortByPrjData) -> Self {
        Self {
            start_week: d.start_week,
            end_week: d.end_week,
            visible: d.visible,
            enable: d.enable,
            text: d.text.clone().into(),
            project: d.project,
            efforts: d.efforts.iter().map(EffortByDevDto::from).collect(),
        }
    }
}

// Extension methods for EffortByPrjData
pub trait EffortByPrjDataExt {
    fn rebuild_dev(&self, dev_id: DevId);
}

impl EffortByPrjDataExt for crate::EffortByPrjData {
    fn rebuild_dev(&self, dev_id: DevId) {
        use super::effort_by_dev::EffortByDevDataExt;

        for dev_index in 0..self.efforts.row_count() {
            if dev_index != dev_id.0 {
                println!("rebuild_dev - Skip dev #{}", dev_index);
                continue;
            }
            println!("rebuild_dev - Manage dev #{:?}", dev_id);

            let mut dev = self.efforts.row_data(dev_id.0).unwrap_or_default();
            dev.total();
            self.efforts.set_row_data(dev_id.0, dev);
            return;
        }
    }
}
