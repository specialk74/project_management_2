use slint::{Model, ModelRc, SharedString};
use std::collections::HashMap;

use crate::models::*;
use crate::{DayData, EffortByDateData, EffortByDevData, EffortByPrjData, EffortsData, SovraData};

impl From<EffortByDateDto> for EffortByDateData {
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

impl From<EffortByDateData> for EffortByDateDto {
    fn from(d: EffortByDateData) -> Self {
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

impl EffortByDateData {
    pub fn get_sovra(&self, sovra: &mut HashMap<String, i32>) {
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

impl From<EffortByDevDto> for EffortByDevData {
    fn from(d: EffortByDevDto) -> Self {
        let mut max = 0;
        for data in d.datas.iter() {
            if data.persons.len() > max {
                max = data.persons.len();
            }
        }
        Self {
            total: d.total,
            project: d.project,
            visible: d.visible,
            dev: d.dev as i32,
            effort: d.effort,
            remains: d.remains,
            max: max as i32,
            datas: ModelRc::new(slint::VecModel::from(
                d.datas
                    .into_iter()
                    .map(EffortByDateData::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<EffortByDevData> for EffortByDevDto {
    fn from(d: EffortByDevData) -> Self {
        let mut max = 0;
        for data in d.datas.iter() {
            if data.persons.row_count() > max {
                max = data.persons.row_count();
            }
        }

        Self {
            total: d.total,
            project: d.project,
            visible: d.visible,
            dev: d.dev.into(),
            effort: d.effort,
            remains: d.remains,
            max: d.datas.iter().count() as i32,
            datas: d.datas.iter().map(EffortByDateDto::from).collect(),
        }
    }
}

impl EffortByDevData {
    pub fn total(&mut self) {
        let mut total = 0;
        for day_index in 0..self.datas.row_count() {
            let mut day = self.datas.row_data(day_index).unwrap_or_default();
            total += EffortByDateDto::from(day.clone()).get_total();
            day.total = total;
            day.effort = self.effort;
            day.remains = self.effort - day.total;
            self.datas.set_row_data(day_index, day);
        }
        self.total = total;
        self.remains = self.effort - self.total;
    }
}

impl From<EffortByPrjDto> for EffortByPrjData {
    fn from(d: EffortByPrjDto) -> Self {
        Self {
            start_week: d.start_week,
            end_week: d.end_week,
            text: SharedString::from(d.text.clone()),
            project: d.project,
            visible: d.visible,
            efforts: ModelRc::new(slint::VecModel::from(
                d.efforts
                    .into_iter()
                    .map(EffortByDevData::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<EffortByPrjData> for EffortByPrjDto {
    fn from(d: EffortByPrjData) -> Self {
        Self {
            start_week: d.start_week,
            end_week: d.end_week,
            visible: d.visible,
            text: d.text.clone().into(),
            project: d.project,
            efforts: d.efforts.iter().map(EffortByDevDto::from).collect(),
        }
    }
}

impl EffortByPrjData {
    pub fn rebuild_dev(&self, dev_id: DevId) {
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

impl From<SovraDto> for SovraData {
    fn from(d: SovraDto) -> Self {
        Self {
            value: ModelRc::new(slint::VecModel::from(d.value)),
            week: d.week,
        }
    }
}

impl From<SovraData> for SovraDto {
    fn from(d: SovraData) -> Self {
        Self {
            value: d.value.iter().collect(),
            week: d.week,
        }
    }
}

impl From<EffortsDto> for EffortsData {
    fn from(d: EffortsDto) -> Self {
        Self {
            week_off: ModelRc::new(slint::VecModel::from(d.week_off)),
            sovra: ModelRc::new(slint::VecModel::from(
                d.sovra.into_iter().map(SovraData::from).collect::<Vec<_>>(),
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
                    .map(EffortByPrjData::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<EffortsData> for EffortsDto {
    fn from(d: EffortsData) -> Self {
        Self {
            sovra: d.sovra.iter().map(SovraDto::from).collect(),
            week_off: d.week_off.iter().collect(),
            worker_names: d.worker_names.iter().map(|s| s.to_string()).collect(),
            projects: d.projects.iter().map(EffortByPrjDto::from).collect(),
        }
    }
}

impl From<DayData> for DayDto {
    fn from(d: DayData) -> Self {
        Self {
            week: d.week,
            text: d.text.to_string(),
        }
    }
}

impl From<DayDto> for DayData {
    fn from(d: DayDto) -> Self {
        Self {
            week: d.week,
            text: d.text.into(),
        }
    }
}
