// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

mod models;
mod utils;
mod date_utils;
mod conversions;
mod file_io;
mod callbacks;

use chrono::Utc;
use slint::{Model, ModelRc, SharedString, VecModel};
use std::collections::HashMap;
use std::{error::Error, rc::Rc};

use models::*;
use date_utils::*;
use file_io::*;
use callbacks::*;
use utils::*;


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let vec_model_projects = Rc::new(VecModel::default());
    let vec_model_week_off = Rc::new(VecModel::default());
    let vec_model_worker_names = Rc::new(VecModel::default());
    let vec_model_sovra = Rc::new(VecModel::default());

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

    for sovra in app_info.sovra {
        vec_model_sovra.push(sovra.into());
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
        let vec_model_sovra = vec_model_sovra.clone();

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

            let mut sovra = Vec::new();
            for i in 0..vec_model_sovra.row_count() {
                let e = vec_model_sovra.row_data(i).unwrap_or(SovraData::default());
                sovra.push(SovraDto::from(e));
            }

            let updated = EffortsData {
                sovra: ModelRc::new(VecModel::from(
                    sovra.into_iter().map(SovraData::from).collect::<Vec<_>>(),
                )),
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
        let vec_model_sovra = vec_model_sovra.clone();
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
                            for s in 0..vec_model_sovra.row_count() {
                                let mut sovra: SovraDto =
                                    vec_model_sovra.row_data(s).unwrap().into();
                                sovra.value.push(0);
                                vec_model_sovra.set_row_data(s, sovra.into());
                            }
                        }
                    } else {
                        println!("person is empty!!!!!");
                    }
                }
            }

            rebuild_project(
                &vec_model_projects,
                ProjectId(effort.project as usize),
                DevId(effort.dev as usize),
            );

            let mut sovra_hash = HashMap::new();
            for p in 0..vec_model_projects.row_count() {
                let effort_by_prj_data = vec_model_projects.row_data(p).unwrap();
                for d in 0..effort_by_prj_data.efforts.row_count() {
                    let effort_by_dev_data = effort_by_prj_data.efforts.row_data(d).unwrap();
                    for data in 0..effort_by_dev_data.datas.row_count() {
                        let effort_by_date_data = effort_by_dev_data.datas.row_data(data).unwrap();
                        if effort_by_date_data.week == effort.week {
                            effort_by_date_data.get_sovra(&mut sovra_hash);
                            break;
                        }
                    }
                }
            }

            println!("sovra: {:?}", sovra_hash);

            for s in 0..vec_model_sovra.row_count() {
                let mut sovra: SovraDto = vec_model_sovra.row_data(s).unwrap().into();
                if sovra.week == effort.week {
                    for n in 0..vec_model_worker_names.row_count() {
                        let name = vec_model_worker_names.row_data(n).unwrap().to_string();
                        if let Some(val) = sovra.value.get_mut(n) {
                            *val = *sovra_hash.get(&name).unwrap_or(&0);
                        }
                    }

                    vec_model_sovra.set_row_data(s, sovra.into());
                    break;
                }
            }

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
        sovra: vec_model_sovra.into(),
        week_off: vec_model_week_off.into(),
        projects: vec_model_projects.into(),
        worker_names: vec_model_worker_names.into(),
    });
    ui.run()?;

    Ok(())
}
