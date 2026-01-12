use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc};

use super::devs::Devs;
use super::effort_by_date::EffortByDateDto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortByDevDto {
    pub project: i32,
    pub total: i32,
    pub visible: bool,
    pub enable: bool,
    pub dev: Devs,
    pub effort: i32,
    pub remains: i32,
    pub max: i32,
    pub datas: Vec<EffortByDateDto>,
}

impl EffortByDevDto {
    pub fn new(dev: Devs, project: i32, num_weeks: i64) -> Self {
        let weeks = crate::date_utils::get_weeks(&dev, project, num_weeks);
        Self {
            total: 0,
            project,
            visible: true,
            enable: true,
            dev,
            effort: 0,
            remains: 0,
            max: 1,
            datas: weeks,
        }
    }

    pub fn prepend_weeks(&mut self, weeks: i32, mut start_week: i32) {
        let data = self.datas.first().unwrap().clone();

        for _ in 0..weeks {
            self.datas.insert(
                0,
                EffortByDateDto {
                    total: 0,
                    remains: 0,
                    dev: data.dev,
                    project: data.project,
                    effort: data.effort,
                    week: 0,
                    persons: vec!["".to_string()],
                },
            );
        }

        for data in self.datas.iter_mut() {
            data.week = start_week;
            start_week += 7;
        }
    }

    pub fn append_weeks(&mut self, weeks: i32, mut start_week: i32) {
        let data = self.datas.last().unwrap().clone();

        for _ in 0..weeks {
            self.datas.push(EffortByDateDto {
                total: data.total,
                remains: data.remains,
                dev: data.dev,
                project: data.project,
                effort: data.effort,
                week: 0,
                persons: vec!["".to_string()],
            });
        }

        for data in self.datas.iter_mut() {
            data.week = start_week;
            start_week += 7;
        }
    }
}

// Conversion implementations for EffortByDevData (from Slint)
impl From<EffortByDevDto> for crate::EffortByDevData {
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
            enable: d.enable,
            dev: d.dev as i32,
            effort: d.effort,
            remains: d.remains,
            max: max as i32,
            datas: ModelRc::new(slint::VecModel::from(
                d.datas
                    .into_iter()
                    .map(crate::EffortByDateData::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<crate::EffortByDevData> for EffortByDevDto {
    fn from(d: crate::EffortByDevData) -> Self {
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
            enable: d.enable,
            dev: d.dev.into(),
            effort: d.effort,
            remains: d.remains,
            max: d.datas.iter().count() as i32,
            datas: d.datas.iter().map(EffortByDateDto::from).collect(),
        }
    }
}

// Extension methods for EffortByDevData
pub trait EffortByDevDataExt {
    fn total(&mut self);
}

impl EffortByDevDataExt for crate::EffortByDevData {
    fn total(&mut self) {
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
