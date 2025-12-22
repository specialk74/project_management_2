// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use slint::{Model, ModelRc, SharedString};
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
            Devs::Ele => 4,
            Devs::Hw => 3,
            Devs::Mcsw => 0,
            Devs::Mvh => 2,
            Devs::Pjm => 8,
            Devs::Sms => 1,
            Devs::TestFw => 6,
            Devs::TestHw => 5,
            Devs::TestSys => 7,
        }
    }
}

impl From<i32> for Devs {
    fn from(value: i32) -> Self {
        match value {
            4 => Devs::Ele,
            3 => Devs::Hw,
            0 => Devs::Mcsw,
            2 => Devs::Mvh,
            8 => Devs::Pjm,
            1 => Devs::Sms,
            6 => Devs::TestFw,
            5 => Devs::TestHw,
            7 => Devs::TestSys,
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
    pub dev: Devs,
    pub effort: i32,
    pub remains: i32,
    pub datas: Vec<EffortByDateDto>,
}

impl From<EffortByDevDto> for EffortByDevData {
    fn from(d: EffortByDevDto) -> Self {
        EffortByDevData {
            dev: d.dev as i32,
            effort: d.effort,
            remains: d.remains,
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
        EffortByDevDto {
            dev: d.dev.into(),
            effort: d.effort,
            remains: d.remains,
            datas: d.datas.iter().map(EffortByDateDto::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EffortByPrjDto {
    text: String,
    project: i32,
    efforts: Vec<EffortByDevDto>,
}

impl From<EffortByPrjDto> for EffortByPrjData {
    fn from(d: EffortByPrjDto) -> Self {
        EffortByPrjData {
            text: SharedString::from(d.text),
            project: d.project,
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
            text: d.text.clone().into(),
            project: d.project,
            efforts: d.efforts.iter().map(EffortByDevDto::from).collect(),
        }
    }
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

async fn get_todos() -> i32 {
    smol::Timer::after(std::time::Duration::from_secs(15)).await;

    3
}

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
            dev: dev.clone(),
            effort: 0,
            remains: 0,
            datas: one_year,
        }
    }
}

impl Default for EffortByPrjDto {
    fn default() -> Self {
        let project = 0;
        EffortByPrjDto {
            text: "New Project".to_string(),
            project,
            efforts: vec![
                EffortByDevDto::new(Devs::Mcsw, project),
                EffortByDevDto::new(Devs::Sms, project),
                EffortByDevDto::new(Devs::Mvh, project),
                EffortByDevDto::new(Devs::Hw, project),
                EffortByDevDto::new(Devs::Ele, project),
                EffortByDevDto::new(Devs::TestFw, project),
                EffortByDevDto::new(Devs::TestHw, project),
                EffortByDevDto::new(Devs::TestSys, project),
                EffortByDevDto::new(Devs::Pjm, project),
            ],
        }
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
        efforts.push(EffortByPrjDto::default());
    }

    for dto in efforts {
        model.push(dto.into()); // EffortByDate::from(dto)
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

    {
        let model = model.clone();
        PjmCallback::get(&ui).on_save_file(move || {
            let mut efforts = Vec::new();
            for i in 0..model.row_count() {
                let e = model.row_data(i).unwrap_or(EffortByPrjData::default());
                efforts.push(EffortByPrjDto::from(&e)); // tua conversione
            }

            let _ = save_efforts_to_file(&efforts, "efforts.json");
            println!("Saved!")
        });
    }

    // {
    //     let ui_weak = ui.as_weak();
    //     let model = model.clone();
    //     PjmCallback::get(&ui).on_changed_effort(move |effort: EffortByDateData| {
    //         let ui = ui_weak.unwrap();

    //     });
    // }

    ui.set_efforts(model.into());

    ui.run()?;

    Ok(())
}
