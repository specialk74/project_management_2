//! Callback handler for worker search functionality.

use slint::{Global, Model, SharedString, VecModel};
use std::rc::Rc;

use crate::{AppWindow, EffortByPrjData, PjmCallback};

/// Registers the search callback.
///
/// This callback filters the display of projects and development categories based on
/// whether they contain workers matching the search text. The search is case-sensitive
/// and uses substring matching.
///
/// # Arguments
/// * `ui` - Reference to the main application window
/// * `vec_model_projects` - Project data model to search within
pub fn register_on_search(ui: &AppWindow, vec_model_projects: Rc<VecModel<EffortByPrjData>>) {
    PjmCallback::get(ui).on_search(move |text: SharedString| {
        println!("on_search {:?}", text);

        // Ottimizzazione: evita allocazioni ripetute convertendo una sola volta
        let search_text = text.as_str();
        let is_empty = search_text.is_empty();

        for project_index in 0..vec_model_projects.row_count() {
            let mut project = vec_model_projects
                .row_data(project_index)
                .unwrap_or_default();
            let mut visible_prj = false;

            for effort_index in 0..project.efforts.row_count() {
                let mut dev = project.efforts.row_data(effort_index).unwrap_or_default();
                let mut visible_dev = false;

                if is_empty {
                    visible_prj = true;
                    visible_dev = true;
                } else {
                    'outer: for data_index in 0..dev.datas.row_count() {
                        let data = dev.datas.row_data(data_index).unwrap_or_default();
                        for person_index in 0..data.persons.row_count() {
                            if let Some(person) = data.persons.row_data(person_index) {
                                // Evita allocazioni - confronta direttamente &str
                                if person.as_str().contains(search_text) {
                                    visible_prj = true;
                                    visible_dev = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                dev.visible = visible_dev;
                project.efforts.set_row_data(effort_index, dev);
            }

            project.visible = visible_prj;
            vec_model_projects.set_row_data(project_index, project);
        }
    });
}
