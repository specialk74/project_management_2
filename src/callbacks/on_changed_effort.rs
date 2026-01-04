//! Callback handler for effort changes.

use slint::{ComponentHandle, Global, Model, SharedString, VecModel};
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    callbacks::rebuild_project,
    models::{effort_by_date::EffortByDateDataExt, DevId, ProjectId, SovraDto},
    utils::info_cell,
    AppWindow, EffortByDateData, EffortByPrjData, PjmCallback, SovraData,
};

/// Registers the changed effort callback.
///
/// This callback handles changes to effort data, including:
/// - Adding new workers to the worker list
/// - Updating over-allocation (sovra) data
/// - Rebuilding project totals
///
/// # Arguments
/// * `ui` - Reference to the main application window
/// * `vec_model_projects` - Project data model
/// * `vec_model_worker_names` - Worker names model
/// * `vec_model_sovra` - Over-allocation tracking model
pub fn register_on_changed_effort(
    ui: &AppWindow,
    vec_model_projects: Rc<VecModel<EffortByPrjData>>,
    vec_model_worker_names: Rc<VecModel<SharedString>>,
    vec_model_sovra: Rc<VecModel<SovraData>>,
) {
    let ui_weak = ui.as_weak();

    PjmCallback::get(ui).on_changed_effort(move |effort: EffortByDateData| {
        // Add new workers to the list if they don't exist
        for i in 0..effort.persons.row_count() {
            if let Some((person, _)) = info_cell(effort.persons.row_data(i).unwrap().as_str()) {
                if !person.is_empty() {
                    // Evita allocazioni ripetute - cerca direttamente con &str
                    let mut founded = false;
                    for j in 0..vec_model_worker_names.row_count() {
                        if let Some(worker) = vec_model_worker_names.row_data(j) {
                            if person == worker.as_str() {
                                founded = true;
                                break;
                            }
                        }
                    }
                    if !founded {
                        vec_model_worker_names.push(SharedString::from(person));
                        // Ottimizzazione: aggiorna tutti i sovra in un colpo solo
                        for s in 0..vec_model_sovra.row_count() {
                            let mut sovra: SovraDto = vec_model_sovra.row_data(s).unwrap().into();
                            sovra.value.push(0);
                            vec_model_sovra.set_row_data(s, sovra.into());
                        }
                    }
                }
            }
        }

        // Rebuild project to update totals
        rebuild_project(
            &vec_model_projects,
            ProjectId(effort.project as usize),
            DevId(effort.dev as usize),
        );

        // Calculate over-allocation for this week
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

        // Update sovra data for this week
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
