use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Devs {
    Mcsw = 0,
    Sms = 1,
    Mvh = 2,
    Hw = 3,
    Ele = 4,
    TestHw = 5,
    TestFw = 6,
    TestSys = 7,
    Pjm = 8,
}

impl From<Devs> for i32 {
    fn from(value: Devs) -> Self {
        match value {
            Devs::Mcsw => 0,
            Devs::Sms => 1,
            Devs::Mvh => 2,
            Devs::Hw => 3,
            Devs::Ele => 4,
            Devs::TestHw => 5,
            Devs::TestFw => 6,
            Devs::TestSys => 7,
            Devs::Pjm => 8,
        }
    }
}

impl From<i32> for Devs {
    fn from(value: i32) -> Self {
        match value {
            0 => Devs::Mcsw,
            1 => Devs::Sms,
            2 => Devs::Mvh,
            3 => Devs::Hw,
            4 => Devs::Ele,
            5 => Devs::TestHw,
            6 => Devs::TestFw,
            7 => Devs::TestSys,
            8 => Devs::Pjm,
            _ => Devs::Mcsw,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ProjectId(pub usize);

#[derive(PartialEq, Debug)]
pub struct DevId(pub usize);

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortByDevDto {
    pub project: i32,
    pub total: i32,
    pub visible: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortByPrjDto {
    pub text: String,
    pub start_week: i32,
    pub end_week: i32,
    pub project: i32,
    pub visible: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SovraDto {
    pub value: Vec<i32>,
    pub week: i32,
}

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

#[derive(Clone, Serialize, Deserialize)]
pub struct DayDto {
    pub week: i32,
    pub text: String,
}

impl DayDto {
    pub fn new(week: i32) -> Self {
        Self {
            week,
            text: crate::date_utils::days_to_local(week)
                .format("%y-%m-%d")
                .to_string(),
        }
    }
}
