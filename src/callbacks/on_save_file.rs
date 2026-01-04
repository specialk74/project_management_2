//! Callback handler for saving efforts to file.

use slint::{ComponentHandle, Global, Model, SharedString, VecModel};
use std::rc::Rc;

use crate::{
    file_io::save_efforts_to_file,
    models::{EffortByPrjDto, EffortsDto, SovraDto},
    AppWindow, EffortByPrjData, PjmCallback, SovraData,
};

/// Registers the save file callback.
///
/// This callback collects all data from the UI models and saves it to a JSON file.
///
/// # Arguments
/// * `ui` - Reference to the main application window
/// * `vec_model_projects` - Project data model
/// * `vec_model_week_off` - Week off data model
/// * `vec_model_worker_names` - Worker names model
/// * `vec_model_sovra` - Over-allocation tracking model
pub fn register_on_save_file(
    ui: &AppWindow,
    vec_model_projects: Rc<VecModel<EffortByPrjData>>,
    vec_model_week_off: Rc<VecModel<i32>>,
    vec_model_worker_names: Rc<VecModel<SharedString>>,
    vec_model_sovra: Rc<VecModel<SovraData>>,
) {
    let ui_weak = ui.as_weak();

    PjmCallback::get(ui).on_save_file(move || {
        // Usa iteratori invece di loop manuali - più efficiente
        let projects: Vec<EffortByPrjDto> = (0..vec_model_projects.row_count())
            .filter_map(|i| vec_model_projects.row_data(i))
            .map(EffortByPrjDto::from)
            .collect();

        let week_off: Vec<i32> = (0..vec_model_week_off.row_count())
            .filter_map(|i| vec_model_week_off.row_data(i))
            .collect();

        let worker_names: Vec<SharedString> = (0..vec_model_worker_names.row_count())
            .filter_map(|i| vec_model_worker_names.row_data(i))
            .collect();

        let sovra: Vec<SovraDto> = (0..vec_model_sovra.row_count())
            .filter_map(|i| vec_model_sovra.row_data(i))
            .map(SovraDto::from)
            .collect();

        // Ottimizzazione: converti direttamente in EffortsDto senza passaggio intermedio
        let dto = EffortsDto {
            sovra,
            week_off,
            worker_names: worker_names.iter().map(|s| s.to_string()).collect(),
            projects,
        };
        let _ = save_efforts_to_file(&dto, "efforts.json");
        if let Some(ui) = ui_weak.upgrade() {
            PjmCallback::get(&ui).set_changed(false);
        }
    });
}
