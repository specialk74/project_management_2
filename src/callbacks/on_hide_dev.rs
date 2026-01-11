//! Callback handler for adding worker rows.

use slint::{ComponentHandle, Global, Model, ModelRc, SharedString, VecModel};
use std::rc::Rc;

use crate::{AppWindow, EffortByPrjData, PjmCallback};

/// Registers the add row callback.
///
/// This callback adds a new empty worker slot to a specific development category
/// in a project. It updates the maximum row count for that development category.
///
/// # Arguments
/// * `ui` - Reference to the main application window
/// * `vec_model_projects` - Project data model containing all projects
pub fn register_on_hide_dev(ui: &AppWindow, vec_model_projects: Rc<VecModel<EffortByPrjData>>) {
    let ui_weak = ui.as_weak();

    PjmCallback::get(ui).on_hide_dev(move |project_id: i32, dev_id: i32, visible: bool| {
        println!(
            "on_hide_dev - project: {} - dev: {} - visible: {}",
            project_id, dev_id, visible
        );

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

                for person_index in 0..dev.max {
                    for data_index in 0..dev.datas.row_count() {
                        let data = dev.datas.row_data(data_index).unwrap_or_default();
                        let persons_model: ModelRc<SharedString> = data.persons;
                        if persons_model
                            .row_data(person_index as usize)
                            .is_some_and(|x| !x.is_empty())
                        {
                            return;
                        }
                    }
                }

                dev.visible = visible;
                project.efforts.set_row_data(effort_index, dev);
                break;
            }
        }

        if let Some(ui) = ui_weak.upgrade() {
            PjmCallback::get(&ui).set_changed(true);
        }
    });
}
