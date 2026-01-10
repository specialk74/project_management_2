// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use project_app::*;

use chrono::Utc;
use slint::{ModelRc, SharedString, VecModel};
use std::{error::Error, rc::Rc};

use project_app::{callbacks::*, date_utils::*, file_io::*};

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let vec_model_projects = Rc::new(VecModel::<EffortByPrjData>::default());
    let vec_model_week_off = Rc::new(VecModel::<i32>::default());
    let vec_model_worker_names = Rc::new(VecModel::<SharedString>::default());
    let vec_model_sovra = Rc::new(VecModel::<SovraData>::default());

    let mut app_info = load_efforts_from_file("efforts.json");
    let (start_week, end_week) = app_info.start_end_weeks();

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

    ui.set_efforts(EffortsData {
        sovra: vec_model_sovra.into(),
        week_off: vec_model_week_off.into(),
        projects: vec_model_projects.into(),
        worker_names: vec_model_worker_names.into(),
    });
    ui.run()?;

    Ok(())
}
