//! Callback handler for creating new projects.

use slint::{ComponentHandle, Global, Model, VecModel};
use std::rc::Rc;

use crate::{models::EffortByPrjDto, AppWindow, EffortByPrjData, PjmCallback};

/// Registers the new project callback.
///
/// This callback creates a new project with default values and adds it to the project list.
///
/// # Arguments
/// * `ui` - Reference to the main application window
/// * `vec_model_projects` - Project data model to add the new project to
pub fn register_on_new_project(
    ui: &AppWindow,
    vec_model_projects: Rc<VecModel<EffortByPrjData>>,
) {
    let ui_weak = ui.as_weak();

    PjmCallback::get(ui).on_new_project(move || {
        println!("New Project");
        let prj = EffortByPrjDto::new(vec_model_projects.row_count() as i32);
        vec_model_projects.push(prj.into());
        ui_weak.upgrade().unwrap();
    });
}
