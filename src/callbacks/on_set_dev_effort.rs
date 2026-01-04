//! Callback handler for setting development effort.

use slint::{ComponentHandle, Global, VecModel};
use std::rc::Rc;

use crate::{
    callbacks::rebuild_project,
    models::{DevId, ProjectId},
    AppWindow, EffortByDevData, EffortByPrjData, PjmCallback,
};

/// Registers the set dev effort callback.
///
/// This callback is triggered when the effort value for a development category is changed.
/// It rebuilds the project data to update totals and remaining effort.
///
/// # Arguments
/// * `ui` - Reference to the main application window
/// * `vec_model_projects` - Project data model containing all projects
pub fn register_on_set_dev_effort(
    ui: &AppWindow,
    vec_model_projects: Rc<VecModel<EffortByPrjData>>,
) {
    let ui_weak = ui.as_weak();

    PjmCallback::get(ui).on_set_dev_effort(move |effort: EffortByDevData| {
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
