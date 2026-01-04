use slint::{Model, VecModel};

use crate::models::{DevId, ProjectId};
use crate::models::effort_by_prj::EffortByPrjDataExt;
use crate::EffortByPrjData;

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
