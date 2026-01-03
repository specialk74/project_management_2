// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc, SharedString, VecModel};
use std::fs::File;
use std::io::Write;
use std::{error::Error, rc::Rc};

slint::include_modules!();

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
struct ProjectId(usize);
#[derive(PartialEq, Debug)]
struct DevId(usize);

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

fn info_cell(person: &str) -> Option<(&str, i32)> {
    let mut split = person.split("|");
    if split.clone().count() != 2 {
        return None;
    }
    Some((
        split.next().unwrap(),
        split
            .next()
            .map_or(0, |f| f.parse::<i32>().map_or(0, |f| f * 40 / 100)),
    ))
}

impl EffortByDateDto {
    fn get_total(&self) -> i32 {
        let mut total = 0;
        for item in self.persons.iter() {
            if let Some((person, value)) = info_cell(item) {
                println!("person: {} - value: {}", person, value);
                total += value;
            }
        }

        total
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EffortByDevDto {
    pub project: i32,
    pub total: i32,
    pub visible: bool,
    pub dev: Devs,
    pub effort: i32,
    pub remains: i32,
    pub max: i32,
    pub datas: Vec<EffortByDateDto>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EffortByPrjDto {
    text: String,
    start_week: i32,
    end_week: i32,
    project: i32,
    visible: bool,
    efforts: Vec<EffortByDevDto>,
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
    fn rebuild_dev(&self, dev_id: DevId) {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EffortsDto {
    week_off: Vec<i32>,
    worker_names: Vec<String>,
    projects: Vec<EffortByPrjDto>,
}

impl Default for EffortsDto {
    fn default() -> Self {
        Self {
            week_off: vec![],
            worker_names: vec![],
            projects: vec![EffortByPrjDto::new(0)],
        }
    }
}

impl From<EffortsDto> for EffortsData {
    fn from(d: EffortsDto) -> Self {
        Self {
            week_off: ModelRc::new(slint::VecModel::from(d.week_off)),
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
            week_off: d.week_off.iter().collect(),
            worker_names: d.worker_names.iter().map(|s| s.to_string()).collect(),
            projects: d.projects.iter().map(EffortByPrjDto::from).collect(),
        }
    }
}

impl EffortsDto {
    /// Retreive the minimum date from all projects or, in case no projects, 30 days from today in the past.
    fn start_end_weeks(&self) -> (i32, i32) {
        let start_week = self
            .projects
            .iter()
            .map(|d| d.start_week)
            .min()
            .unwrap_or(local_to_days(
                &(Utc::now().date_naive() - chrono::Duration::days(30)),
            ));
        let end_week = self
            .projects
            .iter()
            .map(|d| d.end_week)
            .max()
            .unwrap_or(local_to_days(
                &(Utc::now().date_naive() + chrono::Duration::days(365)),
            ));

        (start_week, end_week)
    }
}

fn weeks_list(start_date: &chrono::NaiveDate, end_date: &chrono::NaiveDate) -> Vec<DayDto> {
    let mut weeks: Vec<DayDto> = Vec::new();

    // Recupera il primo giorno della settimana della data passata
    let mut start_week = primo_giorno_settimana_corrente(start_date);
    let end_week = primo_giorno_settimana_corrente(end_date);

    while start_week < end_week {
        weeks.push(DayDto::new(local_to_days(&start_week)));
        start_week += chrono::Duration::days(7);
        start_week = primo_giorno_settimana_corrente(&start_week);
    }

    weeks
}

fn primo_giorno_settimana_corrente(data: &chrono::NaiveDate) -> chrono::NaiveDate {
    let giorni_da_lunedi = data.weekday().num_days_from_monday();
    // println!(
    //     "data: {:?} - giorni_da_lunedi: {} - diff: {}",
    //     data,
    //     giorni_da_lunedi,
    //     *data - chrono::Duration::days(giorni_da_lunedi as i64)
    // );
    *data - chrono::Duration::days(giorni_da_lunedi as i64)
}

fn save_efforts_to_file(efforts: &EffortsDto, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(efforts).unwrap(); // oppure to_string
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_efforts_from_file(path: &str) -> EffortsDto {
    let json_str = std::fs::read_to_string(path);
    if let Ok(json_str) = json_str {
        let efforts_dto: Result<EffortsDto, serde_json::Error> = serde_json::from_str(&json_str);
        if let Ok(efforts_dto) = efforts_dto {
            return efforts_dto;
        } else {
            println!(
                "{}",
                efforts_dto
                    .expect_err(format!("Error during parse the file \"{}\"", path).as_str())
            );
        }
    } else {
        println!(
            "{}",
            json_str.expect_err(format!("Error during load the file \"{}\"", path).as_str())
        );
    }
    println!("Create a default EffortsDto");
    // file non esiste, ritorna vuoto
    EffortsDto::default()
}

// async fn get_todos() -> i32 {
//     smol::Timer::after(std::time::Duration::from_secs(15)).await;

//     3
// }

fn get_weeks(dev: &Devs, project: i32, num_weeks: i64) -> Vec<EffortByDateDto> {
    let mut ret = vec![];
    let mut start_week = local_to_days(&primo_giorno_settimana_corrente(&Utc::now().date_naive()));
    for _ in 0..num_weeks {
        ret.push(EffortByDateDto {
            week: start_week,
            effort: 0,
            dev: *dev,
            project,
            remains: 0,
            total: 0,
            persons: vec!["".to_string()],
        });
        start_week += 7;
    }
    ret
}

impl EffortByDevDto {
    fn new(dev: Devs, project: i32, num_weeks: i64) -> Self {
        let weeks = get_weeks(&dev, project, num_weeks);
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

    fn prepend_weeks(&mut self, weeks: i32, mut start_week: i32) {
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

    fn append_weeks(&mut self, weeks: i32, mut start_week: i32) {
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

fn local_to_days(dt: &NaiveDate) -> i32 {
    dt.to_epoch_days()
    // let epoch = NaiveDate.timestamp_opt(0, 0).unwrap();
    // let seconds = dt.timestamp() - epoch.timestamp();

    // println!(
    //     "local_to_days - dt: {} - epoch: {} - seconds: {} - ret: {}",
    //     dt,
    //     epoch,
    //     seconds,
    //     (seconds / 86_400) as i32
    // );
    // (seconds / 86_400) as i32
}

fn days_to_local(days: i32) -> NaiveDate {
    //let seconds = (days as i64) * 86_400;
    //NaiveDate.timestamp_opt(seconds, 0).unwrap()
    NaiveDate::from_epoch_days(days).unwrap()
}

impl EffortByPrjDto {
    fn new(project: i32) -> Self {
        let num_weeks = 52;
        let start_date = local_to_days(&Utc::now().date_naive());
        let end_date =
            local_to_days(&(Utc::now().date_naive() + chrono::Duration::weeks(num_weeks)));
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

    fn set_date(&mut self, start_week: i32, end_week: i32) {
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

#[derive(Clone, Serialize, Deserialize)]
struct DayDto {
    week: i32,
    text: String,
}

impl DayDto {
    fn new(week: i32) -> Self {
        //let text = days_to_local(week).format("%y-%m-%d").to_string();
        //println!("week: {} - text: {}", week, text);
        Self {
            week,
            text: days_to_local(week).format("%y-%m-%d").to_string(),
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

fn rebuild_project(model: &VecModel<EffortByPrjData>, project_id: ProjectId, dev_id: DevId) {
    for project_index in 0..model.row_count() {
        if project_index != project_id.0 {
            println!("rebuild_project - Skip project #{}", project_index);
            continue;
        }
        println!("rebuild_project - Manage project #{:?}", project_id);

        let project = model.row_data(project_id.0).unwrap_or_default();
        project.rebuild_dev(dev_id);
        return;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let vec_model_projects = Rc::new(slint::VecModel::default());
    let vec_model_week_off = Rc::new(slint::VecModel::default());
    let vec_model_worker_names = Rc::new(slint::VecModel::default());

    let mut app_info = load_efforts_from_file("efforts.json");
    let (start_week, end_week) = app_info.start_end_weeks();

    println!("start_week: {} - end_week: {}", start_week, end_week);

    let weeks_day_dto = weeks_list(&days_to_local(start_week), &days_to_local(end_week));
    let weeks_day_data = ModelRc::new(slint::VecModel::from(
        weeks_day_dto
            .into_iter()
            .map(DayData::from)
            .collect::<Vec<_>>(),
    ));

    ui.as_weak().upgrade().unwrap().set_weeks(weeks_day_data);

    for dto in app_info.projects.iter_mut() {
        dto.set_date(start_week, end_week);
    }

    for dto in app_info.projects.into_iter() {
        vec_model_projects.push(dto.into());
    }

    for person in app_info.worker_names {
        vec_model_worker_names.push(person.into());
    }

    let this_week = local_to_days(&primo_giorno_settimana_corrente(&Utc::now().date_naive()));
    PjmCallback::get(&ui).set_this_week(this_week);
    println!("this_week: {}", this_week);

    // Save Projects
    {
        let ui_weak = ui.as_weak();
        let vec_model_projects = vec_model_projects.clone();
        let vec_model_week_off = vec_model_week_off.clone();
        let vec_model_worker_names = vec_model_worker_names.clone();

        PjmCallback::get(&ui).on_save_file(move || {
            let mut projects = Vec::new();
            for i in 0..vec_model_projects.row_count() {
                let e = vec_model_projects
                    .row_data(i)
                    .unwrap_or(EffortByPrjData::default());
                projects.push(EffortByPrjDto::from(e)); // tua conversione
            }

            let mut week_off = Vec::new();
            for i in 0..vec_model_week_off.row_count() {
                let e = vec_model_week_off.row_data(i).unwrap_or(0);
                week_off.push(e);
            }

            let mut worker_names = Vec::new();
            for i in 0..vec_model_worker_names.row_count() {
                let e = vec_model_worker_names
                    .row_data(i)
                    .unwrap_or(SharedString::default());
                worker_names.push(e);
            }

            let updated = EffortsData {
                week_off: ModelRc::new(VecModel::from(week_off)),
                projects: ModelRc::new(VecModel::from(
                    projects
                        .into_iter()
                        .map(EffortByPrjData::from)
                        .collect::<Vec<_>>(),
                )),
                worker_names: ModelRc::new(VecModel::from(worker_names)),
            };
            let _ = save_efforts_to_file(&updated.into(), "efforts.json");
            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_changed(false);
            }
        });
    }

    // New Project
    {
        let vec_model_projects = vec_model_projects.clone();
        let ui_weak = ui.as_weak();

        PjmCallback::get(&ui).on_new_project(move || {
            println!("New Project");
            let prj = EffortByPrjDto::new(vec_model_projects.row_count() as i32);
            vec_model_projects.push(prj.into());
            ui_weak.upgrade().unwrap();
        });
    }

    // Set Dev Effort
    {
        let ui_weak = ui.as_weak();
        let vec_model_projects = vec_model_projects.clone();
        PjmCallback::get(&ui).on_set_dev_effort(move |effort: EffortByDevData| {
            rebuild_project(
                &vec_model_projects,
                ProjectId(effort.project as usize),
                DevId(effort.dev as usize),
            );
            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_changed(true);
            }
        });
    }

    // Change effort
    {
        let ui_weak = ui.as_weak();
        let vec_model_projects = vec_model_projects.clone();
        let vec_model_worker_names = vec_model_worker_names.clone();
        PjmCallback::get(&ui).on_changed_effort(move |effort: EffortByDateData| {
            for i in 0..effort.persons.row_count() {
                if let Some((person, _)) = info_cell(effort.persons.row_data(i).unwrap().as_str()) {
                    if !person.is_empty() {
                        let mut founded = false;
                        for j in 0..vec_model_worker_names.row_count() {
                            let worker = vec_model_worker_names.row_data(j).unwrap().to_string();
                            if person == worker {
                                founded = true;
                                break;
                            }
                        }
                        if !founded {
                            vec_model_worker_names.push(SharedString::from(person));
                        }
                    }
                }
            }

            rebuild_project(
                &vec_model_projects,
                ProjectId(effort.project as usize),
                DevId(effort.dev as usize),
            );
            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_changed(true);
            }
        });
    }

    // Search Worker
    {
        let vec_model_projects = vec_model_projects.clone();
        PjmCallback::get(&ui).on_search(move |text: SharedString| {
            println!("on_search {:?}", text);

            for project_index in 0..vec_model_projects.row_count() {
                let mut project = vec_model_projects
                    .row_data(project_index)
                    .unwrap_or_default();
                let mut visible_prj = false;

                for effort_index in 0..project.efforts.row_count() {
                    let mut dev = project.efforts.row_data(effort_index).unwrap_or_default();
                    let mut visible_dev = false;

                    if text.is_empty() {
                        visible_prj = true;
                        visible_dev = true;
                    } else {
                        for data_index in 0..dev.datas.row_count() {
                            let data = dev.datas.row_data(data_index).unwrap_or_default();
                            for person_index in 0..data.persons.row_count() {
                                let person =
                                    data.persons.row_data(person_index).unwrap_or_default();
                                if person.to_string().contains(&text.to_string()) {
                                    visible_prj = true;
                                    visible_dev = true;
                                    break;
                                }
                            }
                            if visible_dev {
                                break;
                            }
                        }
                    }

                    dev.visible = visible_dev;
                    project.efforts.set_row_data(effort_index, dev); // ✅ NOTIFICA
                }

                project.visible = visible_prj;
                vec_model_projects.set_row_data(project_index, project); // ✅ NOTIFICA principale
            }
        });
    }

    // Add Row
    {
        let vec_model_projects = vec_model_projects.clone();

        PjmCallback::get(&ui).on_add_row(move |project_id: i32, dev_id: i32| {
            println!("on_add_row - project: {} - dev: {}", project_id, dev_id);

            for project_index in 0..vec_model_projects.row_count() {
                let project = vec_model_projects
                    .row_data(project_index)
                    .unwrap_or_default();
                if project.project != project_id {
                    continue;
                }

                for effort_index in 0..project.efforts.row_count() {
                    let mut dev = project.efforts.row_data(effort_index).unwrap_or_default();
                    if dev.dev != dev_id {
                        continue;
                    }

                    let mut max = 0;
                    for data_index in 0..dev.datas.row_count() {
                        let data = dev.datas.row_data(data_index).unwrap_or_default();
                        let persons_model: ModelRc<SharedString> = data.persons; /* dal tuo model */

                        let vec_model = persons_model
                            .as_any()
                            .downcast_ref::<VecModel<SharedString>>() // ← downcast_ref (NON mut!)
                            .expect("Deve essere VecModel");

                        vec_model.push("".into());
                        if max < vec_model.row_count() {
                            max = vec_model.row_count();
                        }
                    }
                    dev.max = max as i32;
                    project.efforts.set_row_data(effort_index, dev);
                }
            }
        });
    }

    ui.set_efforts(EffortsData {
        week_off: vec_model_week_off.into(),
        projects: vec_model_projects.into(),
        worker_names: vec_model_worker_names.into(),
    });
    ui.run()?;

    Ok(())
}
