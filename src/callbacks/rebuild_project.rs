//! Project rebuild utility for recalculating totals and remaining effort.

use slint::{Model, VecModel};

use crate::models::{effort_by_prj::EffortByPrjDataExt, DevId, ProjectId};
use crate::EffortByPrjData;

/// Rebuilds a specific development category within a project.
///
/// This function recalculates the totals and remaining effort for a specific
/// development category (dev_id) within a project (project_id).
///
/// # Arguments
/// * `model` - Reference to the VecModel containing all projects
/// * `project_id` - ID of the project to rebuild
/// * `dev_id` - ID of the development category to rebuild
pub fn rebuild_project(
    model: &VecModel<EffortByPrjData>,
    project_id: ProjectId,
    dev_id: DevId,
) {
    for project_index in 0..model.row_count() {
        if project_index != project_id.0 {
            println!("rebuild_project - Skip project #{}", project_index);
            continue;
        }
        println!("rebuild_project - Manage project #{:?}", project_id);

        let project = model.row_data(project_id.0).unwrap_or_default();
        project.rebuild_dev(dev_id);
        return;
    }
}
