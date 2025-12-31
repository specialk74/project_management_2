// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Datelike, Local, TimeZone};
use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc, SharedString, VecModel};
use std::fs::File;
use std::io::Write;
use std::{error::Error, rc::Rc};

slint::include_modules!();

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EffortByDateDto {
    pub total: i32,
    pub remains: i32,
    pub dev: Devs,
    pub project: i32,
    pub effort: i32,
    pub date: i32,
    pub persons: Vec<String>,
}

impl EffortByDateDto {
    fn get_total(&self) -> i32 {
        let mut total = 0;
        for person in self.persons.iter() {
            let mut split = person.split("|");
            total += split
                .nth(1)
                .map_or(0, |f| f.parse::<i32>().map_or(0, |f| f * 40 / 100));
        }

        total
    }
}

impl From<EffortByDateDto> for EffortByDateData {
    fn from(d: EffortByDateDto) -> Self {
        EffortByDateData {
            total: d.total,
            remains: d.remains,
            dev: d.dev.into(),
            project: d.project,
            effort: d.effort,
            date: d.date,
            persons: ModelRc::new(slint::VecModel::from(
                d.persons
                    .into_iter()
                    .map(SharedString::from)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl From<EffortByDateData> for EffortByDateDto {
    fn from(d: EffortByDateData) -> Self {
        EffortByDateDto {
            total: d.total,
            remains: d.remains,
            dev: d.dev.into(),
            project: d.project,
            effort: d.effort,
            date: d.date,
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
        EffortByDevData {
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

        EffortByDevDto {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EffortByPrjDto {
    text: String,
    start: i32,
    project: i32,
    visible: bool,
    efforts: Vec<EffortByDevDto>,
}

impl From<EffortByPrjDto> for EffortByPrjData {
    fn from(d: EffortByPrjDto) -> Self {
        EffortByPrjData {
            start: d.start,
            text: SharedString::from(d.text),
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

impl From<&EffortByPrjData> for EffortByPrjDto {
    fn from(d: &EffortByPrjData) -> Self {
        EffortByPrjDto {
            start: d.start,
            visible: d.visible,
            text: d.text.clone().into(),
            project: d.project,
            efforts: d.efforts.iter().map(EffortByDevDto::from).collect(),
        }
    }
}

fn dates(start_date: &chrono::DateTime<Local>) -> Vec<SharedString> {
    let mut dates: Vec<SharedString> = Vec::new();

    // Recupera il primo giorno della settimana della data passata
    let mut start_date = primo_giorno_settimana_corrente(start_date);

    // Quale settimana dell'anno è?
    let week_number = 52 - start_date.iso_week().week();

    // Dalla data di partenza per oltre 1 anno
    for _ in 0..52 + week_number {
        let date_str = start_date.format("%y-%m-%d").to_string();
        dates.push(date_str.clone().into());
        start_date += chrono::Duration::days(7);
        start_date = primo_giorno_settimana_corrente(&start_date);
    }
    dates
}

fn primo_giorno_settimana_corrente(data: &chrono::DateTime<Local>) -> chrono::DateTime<Local> {
    let giorni_da_lunedi = data.weekday().num_days_from_monday();
    *data - chrono::Duration::days(giorni_da_lunedi as i64)
}

fn save_efforts_to_file(efforts: &Vec<EffortByPrjDto>, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(efforts).unwrap(); // oppure to_string
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_efforts_from_file(path: &str) -> Vec<EffortByPrjDto> {
    if let Ok(json_str) = std::fs::read_to_string(path) {
        let efforts_dto: Result<Vec<EffortByPrjDto>, serde_json::Error> =
            serde_json::from_str(&json_str);
        if let Ok(efforts_dto) = efforts_dto {
            return efforts_dto;
        }
    }
    // file non esiste, ritorna vuoto
    Vec::new()
}

// async fn get_todos() -> i32 {
//     smol::Timer::after(std::time::Duration::from_secs(15)).await;

//     3
// }

fn get_one_year(dev: &Devs, project: i32) -> Vec<EffortByDateDto> {
    let mut ret = vec![];

    for index in 0..52 {
        ret.push(EffortByDateDto {
            date: index,
            effort: 0,
            dev: dev.clone(),
            project,
            remains: 0,
            total: 0,
            persons: vec!["".to_string()],
        });
    }
    ret
}

impl EffortByDevDto {
    fn new(dev: Devs, project: i32) -> Self {
        let one_year = get_one_year(&dev, project);
        Self {
            total: 0,
            project,
            visible: true,
            dev: dev.clone(),
            effort: 0,
            remains: 0,
            max: 1,
            datas: one_year,
        }
    }

    fn add_days(&mut self, days: i32) {
        let data = self.datas.first().unwrap().clone();

        for day in 0..days {
            self.datas.insert(
                0,
                EffortByDateDto {
                    total: 0,
                    remains: 0,
                    dev: data.dev.clone(),
                    project: data.project,
                    effort: data.effort,
                    date: days - day,
                    persons: vec!["".to_string()],
                },
            );
        }
    }
}

fn local_to_days(dt: &DateTime<Local>) -> i32 {
    let epoch = Local.timestamp_opt(0, 0).unwrap();
    let seconds = dt.timestamp() - epoch.timestamp();
    (seconds / 86_400) as i32
}

fn days_to_local(days: i32) -> DateTime<Local> {
    let seconds = (days as i64) * 86_400;
    Local.timestamp_opt(seconds, 0).unwrap()
}

impl EffortByPrjDto {
    fn new(project: i32) -> Self {
        EffortByPrjDto {
            project,
            start: local_to_days(&Local::now()),
            visible: true,
            text: "New Project".to_string(),
            efforts: vec![
                EffortByDevDto::new(Devs::Mcsw, project),
                EffortByDevDto::new(Devs::Sms, project),
                EffortByDevDto::new(Devs::Mvh, project),
                EffortByDevDto::new(Devs::Hw, project),
                EffortByDevDto::new(Devs::Ele, project),
                EffortByDevDto::new(Devs::TestHw, project),
                EffortByDevDto::new(Devs::TestFw, project),
                EffortByDevDto::new(Devs::TestSys, project),
                EffortByDevDto::new(Devs::Pjm, project),
            ],
        }
    }

    fn set_start_date(&mut self, start: i32) {
        if self.start <= start {
            return;
        }

        let diff = (self.start - start) / 7;
        for dev in self.efforts.iter_mut() {
            dev.add_days(diff);
        }
        self.start = start;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    //let old_model = Rc::new(slint::VecModel::default());
    let model = Rc::new(slint::VecModel::default());

    // {
    //     let model = old_model.clone();
    //     let ui_weak = ui.as_weak();
    //     slint::spawn_local(async move {
    //         model.set_vec(get_todos().await);
    //         ui_weak.upgrade().unwrap().set_loading(false);
    //     })
    //     .unwrap();
    // }

    let mut efforts = load_efforts_from_file("efforts.json");
    if efforts.is_empty() {
        println!("Creo un progetto di default");
        efforts.push(EffortByPrjDto::new(0));
    }

    //let mut start_date = local_to_days(&(Local::now() - chrono::Duration::days(30)));

    let start_date = efforts
        .iter()
        .map(|d| d.start)
        .min()
        .unwrap_or(local_to_days(&(Local::now() - chrono::Duration::days(30))));

    println!(
        "start_date: {:?} -> {}",
        start_date,
        days_to_local(start_date)
    );

    for dto in efforts.iter() {
        println!(
            "Project {} start_date: {}",
            dto.project,
            days_to_local(dto.start)
        );
    }

    for dto in efforts.iter_mut() {
        dto.set_start_date(start_date);
    }

    for dto in efforts {
        model.push(dto.into());
    }

    let weeks = std::rc::Rc::new(slint::VecModel::from(dates(&days_to_local(start_date))));

    {
        ui.as_weak()
            .upgrade()
            .unwrap()
            .set_weeks(weeks.clone().into());
    }

    // {
    //     let ui_weak = ui.as_weak();
    //     let model = model.clone();
    //     slint::spawn_local(async move {
    //         ui_weak
    //             .upgrade()
    //             .unwrap()
    //             .set_num_visible(get_todos().await);

    //         let mut efforts = Vec::new();
    //         for i in 0..model.row_count() {
    //             let e = model.row_data(i).unwrap_or(EffortByPrjData::default());
    //             efforts.push(EffortByPrjDto::from(&e)); // tua conversione
    //         }

    //         let _ = save_efforts_to_file(&efforts, "efforts.json");
    //         println!("Saved!")

    //         // for dto in efforts {
    //         //     model.push(dto.into()); // EffortByDate::from(dto)
    //         // }

    //         // // poi:
    //     })
    //     .unwrap();
    // }

    // Save Projects
    {
        let ui_weak = ui.as_weak();
        let model = model.clone();
        PjmCallback::get(&ui).on_save_file(move || {
            let mut efforts = Vec::new();
            for i in 0..model.row_count() {
                let e = model.row_data(i).unwrap_or(EffortByPrjData::default());
                efforts.push(EffortByPrjDto::from(&e)); // tua conversione
            }

            let _ = save_efforts_to_file(&efforts, "efforts.json");
            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_changed(false);
            }
        });
    }

    // New Project
    {
        let model = model.clone();
        let ui_weak = ui.as_weak();

        PjmCallback::get(&ui).on_new_project(move || {
            println!("New Project");
            let prj = EffortByPrjDto::new(model.row_count() as i32);
            model.push(prj.into());
            ui_weak.upgrade().unwrap();
        });
    }

    // Set Dev Effort
    {
        let ui_weak = ui.as_weak();
        let model = model.clone();
        PjmCallback::get(&ui).on_set_dev_effort(move |effort: EffortByDevData| {
            for project_index in 0..model.row_count() {
                if project_index != effort.project as usize {
                    println!("Skip project #{}", project_index);
                    continue;
                }
                println!("Manage project #{}", effort.project);

                let project = model.row_data(project_index).unwrap_or_default();
                for dev_index in 0..project.efforts.row_count() {
                    if dev_index != effort.dev as usize {
                        println!("Skip dev #{}", dev_index);
                        continue;
                    }
                    println!("Manage dev #{}", dev_index);

                    let mut dev = project.efforts.row_data(dev_index).unwrap_or_default();
                    dev.remains = dev.effort - dev.total;

                    for day_index in 0..dev.datas.row_count() {
                        let mut day = dev.datas.row_data(day_index).unwrap_or_default();
                        day.effort = dev.effort;
                        day.remains = dev.remains;
                        dev.datas.set_row_data(day_index, day);
                    }
                    project.efforts.set_row_data(dev_index, dev);
                }
            }
            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_changed(true);
            }
        });
    }

    // Change effort
    {
        let ui_weak = ui.as_weak();
        let model = model.clone();
        PjmCallback::get(&ui).on_changed_effort(move |effort: EffortByDateData| {
            for project_index in 0..model.row_count() {
                if project_index != effort.project as usize {
                    println!("Skip project #{}", project_index);
                    continue;
                }
                println!("Manage project #{}", effort.project);

                let project = model.row_data(project_index).unwrap_or_default();
                for dev_index in 0..project.efforts.row_count() {
                    if dev_index != effort.dev as usize {
                        println!("Skip dev #{}", dev_index);
                        continue;
                    }
                    println!("Manage dev #{}", dev_index);

                    let mut dev = project.efforts.row_data(dev_index).unwrap_or_default();

                    let mut total = 0;
                    for day_index in 0..dev.datas.row_count() {
                        let mut day = dev.datas.row_data(day_index).unwrap_or_default();
                        total += EffortByDateDto::from(day.clone()).get_total();
                        //println!("total: {}", total);
                        day.total = total;
                        dev.datas.set_row_data(day_index, day);
                    }
                    dev.total = total;
                    dev.remains = dev.effort - dev.total;

                    for day_index in 0..dev.datas.row_count() {
                        let mut day = dev.datas.row_data(day_index).unwrap_or_default();
                        day.remains = dev.effort - day.total;
                        dev.datas.set_row_data(day_index, day);
                    }

                    project.efforts.set_row_data(dev_index, dev);
                }
            }

            // if let Some(project) = model.row_data(effort.project as usize) {
            //     if let Some(dev) = project.efforts.row_data(effort.dev as usize) {
            //         if let Some(mut day) = dev.datas.row_data(effort.date as usize) {
            //             day.total = EffortByDateDto::from(day.clone()).get_total();
            //             dev.datas.set_row_data(effort.date as usize, day);
            //             //project.efforts.set_row_data(effort.dev as usize, dev);
            //             //model.set_row_data(effort.project as usize, project);
            //         }
            //     }
            // }

            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_changed(true);
            }
        });
    }

    // Search Worker
    {
        let model = model.clone();
        PjmCallback::get(&ui).on_search(move |text: SharedString| {
            println!("on_search {:?}", text);

            for project_index in 0..model.row_count() {
                let mut project = model.row_data(project_index).unwrap_or_default();
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

                        // if (!visible_dev) {
                        //     for data_index in 0..dev.datas.row_count() {
                        //         let data = dev.datas.row_data(data_index).unwrap_or_default();
                        //         data.visible = false;
                        //         dev.datas.set_row_data(data_index, data);
                        //     }
                        // }
                    }

                    dev.visible = visible_dev;
                    project.efforts.set_row_data(effort_index, dev); // ✅ NOTIFICA
                }

                project.visible = visible_prj;
                model.set_row_data(project_index, project); // ✅ NOTIFICA principale
            }
        });
    }

    // Add Row
    {
        let model = model.clone();
        //let ui_weak = ui.as_weak();

        PjmCallback::get(&ui).on_add_row(move |project_id: i32, dev_id: i32| {
            println!("on_add_row - project: {} - dev: {}", project_id, dev_id);

            for project_index in 0..model.row_count() {
                let project = model.row_data(project_index).unwrap_or_default();
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

    ui.set_efforts(model.into());

    ui.run()?;

    Ok(())
}
