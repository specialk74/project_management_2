// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use project_app::*;

use chrono::{NaiveDate, Utc};
use slint::{Model, ModelRc, SharedString, VecModel};
use std::{cell::RefCell, error::Error, rc::Rc};

use project_app::{callbacks::*, date_utils::*, file_io::*};

struct AppArgs {
    file: String,
    start_date: Option<NaiveDate>,
}

fn parse_args() -> AppArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut file = "efforts.json".to_string();
    let mut start_date = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" | "-f" => {
                i += 1;
                if i < args.len() {
                    file = args[i].clone();
                }
            }
            "--start-date" | "-d" => {
                i += 1;
                if i < args.len() {
                    match NaiveDate::parse_from_str(&args[i], "%Y-%m-%d") {
                        Ok(date) => start_date = Some(date),
                        Err(e) => eprintln!("Data non valida '{}': {} (formato atteso: YYYY-MM-DD)", args[i], e),
                    }
                }
            }
            arg if !arg.starts_with('-') => {
                file = arg.to_string();
            }
            arg => {
                eprintln!("Opzione non riconosciuta: {}", arg);
            }
        }
        i += 1;
    }

    AppArgs { file, start_date }
}

fn populate_models(
    mut app_info: project_app::models::EffortsDto,
    start_date_override: Option<NaiveDate>,
    vec_model_projects: &Rc<VecModel<EffortByPrjData>>,
    vec_model_week_off: &Rc<VecModel<i32>>,
    vec_model_worker_names: &Rc<VecModel<SharedString>>,
    vec_model_sovra: &Rc<VecModel<SovraData>>,
    ui: &AppWindow,
) {
    // Clear existing data
    for i in (0..vec_model_projects.row_count()).rev() {
        vec_model_projects.remove(i);
    }
    for i in (0..vec_model_week_off.row_count()).rev() {
        vec_model_week_off.remove(i);
    }
    for i in (0..vec_model_worker_names.row_count()).rev() {
        vec_model_worker_names.remove(i);
    }
    for i in (0..vec_model_sovra.row_count()).rev() {
        vec_model_sovra.remove(i);
    }

    let (mut start_week, end_week) = app_info.start_end_weeks();

    if let Some(date) = start_date_override {
        start_week = local_to_days(&primo_giorno_settimana_corrente(&date));
    }

    println!("start_week: {} - end_week: {}", start_week, end_week);

    let weeks_day_dto = weeks_list(&days_to_local(start_week), &days_to_local(end_week));
    let weeks_day_data = ModelRc::new(VecModel::from(
        weeks_day_dto
            .into_iter()
            .map(DayData::from)
            .collect::<Vec<_>>(),
    ));
    ui.set_weeks(weeks_day_data);

    for dto in app_info.projects.iter_mut() {
        dto.set_date(start_week, end_week);
    }

    for dto in app_info.projects.into_iter() {
        vec_model_projects.push(dto.into());
    }

    for person in app_info.worker_names {
        vec_model_worker_names.push(person.into());
    }

    for sovra in app_info.sovra {
        vec_model_sovra.push(sovra.into());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    let ui = AppWindow::new()?;

    let vec_model_projects = Rc::new(VecModel::<EffortByPrjData>::default());
    let vec_model_week_off = Rc::new(VecModel::<i32>::default());
    let vec_model_worker_names = Rc::new(VecModel::<SharedString>::default());
    let vec_model_sovra = Rc::new(VecModel::<SovraData>::default());

    let current_file = Rc::new(RefCell::new(args.file.clone()));

    let app_info = load_efforts_from_file(&args.file);

    // Show filename (without full path) in title bar
    let display_name = std::path::Path::new(&args.file)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or(args.file.clone());
    PjmCallback::get(&ui).set_current_file(display_name.into());

    populate_models(
        app_info,
        args.start_date,
        &vec_model_projects,
        &vec_model_week_off,
        &vec_model_worker_names,
        &vec_model_sovra,
        &ui,
    );

    let this_week = local_to_days(&primo_giorno_settimana_corrente(&Utc::now().date_naive()));
    PjmCallback::get(&ui).set_this_week(this_week);
    println!("this_week: {}", this_week);

    // Register all callbacks
    register_on_save_file(
        &ui,
        vec_model_projects.clone(),
        vec_model_week_off.clone(),
        vec_model_worker_names.clone(),
        vec_model_sovra.clone(),
        current_file.clone(),
    );

    register_on_new_project(&ui, vec_model_projects.clone());

    register_on_set_dev_effort(&ui, vec_model_projects.clone());

    register_on_changed_effort(
        &ui,
        vec_model_projects.clone(),
        vec_model_worker_names.clone(),
        vec_model_sovra.clone(),
    );

    register_on_search(&ui, vec_model_projects.clone());

    register_on_del_row(&ui, vec_model_projects.clone());
    register_on_add_row(&ui, vec_model_projects.clone());
    register_on_hide_dev(&ui, vec_model_projects.clone());

    // Register open file callback (Ctrl+O)
    {
        let ui_weak = ui.as_weak();
        let vm_projects = vec_model_projects.clone();
        let vm_week_off = vec_model_week_off.clone();
        let vm_worker_names = vec_model_worker_names.clone();
        let vm_sovra = vec_model_sovra.clone();
        let cf = current_file.clone();

        PjmCallback::get(&ui).on_open_file(move || {
            let path = rfd::FileDialog::new()
                .add_filter("JSON files", &["json"])
                .set_title("Apri file effort")
                .pick_file();

            if let Some(path) = path {
                let path_str = path.to_string_lossy().to_string();
                let app_info = load_efforts_from_file(&path_str);
                *cf.borrow_mut() = path_str.clone();

                if let Some(ui) = ui_weak.upgrade() {
                    let display_name = std::path::Path::new(&path_str)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or(path_str.clone());
                    PjmCallback::get(&ui).set_current_file(display_name.into());

                    populate_models(
                        app_info,
                        None,
                        &vm_projects,
                        &vm_week_off,
                        &vm_worker_names,
                        &vm_sovra,
                        &ui,
                    );
                    PjmCallback::get(&ui).set_changed(false);
                }
            }
        });
    }

    ui.set_efforts(EffortsData {
        sovra: vec_model_sovra.into(),
        week_off: vec_model_week_off.into(),
        projects: vec_model_projects.into(),
        worker_names: vec_model_worker_names.into(),
    });
    ui.run()?;

    Ok(())
}
