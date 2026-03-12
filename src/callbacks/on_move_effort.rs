//! Callback handler for moving selected person-cells between weeks.

use slint::{ComponentHandle, Global, Model, SharedString, VecModel};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::{
    AppWindow, EffortByPrjData, PjmCallback, SovraData,
    callbacks::rebuild_project,
    models::{DevId, ProjectId, SovraDto, effort_by_date::EffortByDateDataExt},
};

/// Registers the move_effort callback.
///
/// Moves the `persons` content of a rectangular selection
/// (start_week..=end_week × start_row..=end_row) by `offset_weeks` columns.
/// Source cells are cleared; totals and sovra are fully recalculated.
pub fn register_on_move_effort(
    ui: &AppWindow,
    vec_model_projects: Rc<VecModel<EffortByPrjData>>,
    vec_model_sovra: Rc<VecModel<SovraData>>,
    vec_model_worker_names: Rc<VecModel<SharedString>>,
) {
    let ui_weak = ui.as_weak();

    PjmCallback::get(ui).on_move_effort(
        move |project: i32,
              dev: i32,
              start_week: i32,
              end_week: i32,
              start_row: i32,
              end_row: i32,
              offset_weeks: i32| {
            if offset_weeks == 0 {
                return;
            }

            let Some(prj) = vec_model_projects.row_data(project as usize) else {
                return;
            };
            let Some(dev_data) = prj.efforts.row_data(dev as usize) else {
                return;
            };

            // week → datas-index
            let mut week_to_idx: HashMap<i32, usize> = HashMap::new();
            for i in 0..dev_data.datas.row_count() {
                if let Some(d) = dev_data.datas.row_data(i) {
                    week_to_idx.insert(d.week, i);
                }
            }

            let n_weeks = (end_week - start_week) / 7 + 1;
            let sel_weeks: Vec<i32> = (0..n_weeks).map(|i| start_week + i * 7).collect();

            // --- Snapshot: [week_offset][row_offset] = value ---
            let snapshot: Vec<Vec<SharedString>> = sel_weeks
                .iter()
                .map(|&week| {
                    week_to_idx
                        .get(&week)
                        .and_then(|&idx| dev_data.datas.row_data(idx))
                        .map(|d| {
                            (start_row..=end_row)
                                .map(|row| {
                                    d.persons
                                        .row_data(row as usize)
                                        .unwrap_or_default()
                                })
                                .collect()
                        })
                        .unwrap_or_default()
                })
                .collect();

            // --- Clear source cells ---
            for &week in &sel_weeks {
                if let Some(&idx) = week_to_idx.get(&week) {
                    let d = dev_data.datas.row_data(idx).unwrap();
                    let pvm = d
                        .persons
                        .as_any()
                        .downcast_ref::<VecModel<SharedString>>()
                        .expect("persons must be VecModel");
                    for row in start_row..=end_row {
                        if (row as usize) < pvm.row_count() {
                            pvm.set_row_data(row as usize, SharedString::default());
                        }
                    }
                    dev_data.datas.set_row_data(idx, d);
                }
            }

            // --- Write snapshot into target cells ---
            for (wi, &week) in sel_weeks.iter().enumerate() {
                let target_week = week + offset_weeks * 7;
                if let Some(&tidx) = week_to_idx.get(&target_week) {
                    let d = dev_data.datas.row_data(tidx).unwrap();
                    let pvm = d
                        .persons
                        .as_any()
                        .downcast_ref::<VecModel<SharedString>>()
                        .expect("persons must be VecModel");
                    for (ri, val) in snapshot[wi].iter().enumerate() {
                        let target_row = start_row as usize + ri;
                        while pvm.row_count() <= target_row {
                            pvm.push(SharedString::default());
                        }
                        pvm.set_row_data(target_row, val.clone());
                    }
                    dev_data.datas.set_row_data(tidx, d);
                }
            }

            // --- Recalculate max persons across all weeks for this dev ---
            let new_max = (0..dev_data.datas.row_count())
                .filter_map(|i| dev_data.datas.row_data(i))
                .map(|d| d.persons.row_count())
                .max()
                .unwrap_or(1) as i32;
            let mut dev_mut = dev_data;
            dev_mut.max = new_max;
            prj.efforts.set_row_data(dev as usize, dev_mut);

            // --- Rebuild totals / remains ---
            rebuild_project(
                &vec_model_projects,
                ProjectId(project as usize),
                DevId(dev as usize),
            );

            // --- Recalculate sovra for every affected week ---
            let affected: HashSet<i32> = sel_weeks
                .iter()
                .flat_map(|&w| [w, w + offset_weeks * 7])
                .collect();

            for aw in &affected {
                let mut sovra_hash: HashMap<String, i32> = HashMap::new();
                for p in 0..vec_model_projects.row_count() {
                    let prj = vec_model_projects.row_data(p).unwrap();
                    for d in 0..prj.efforts.row_count() {
                        let dr = prj.efforts.row_data(d).unwrap();
                        for di in 0..dr.datas.row_count() {
                            let date = dr.datas.row_data(di).unwrap();
                            if date.week == *aw {
                                date.get_sovra(&mut sovra_hash);
                                break;
                            }
                        }
                    }
                }
                for s in 0..vec_model_sovra.row_count() {
                    let mut sovra: SovraDto = vec_model_sovra.row_data(s).unwrap().into();
                    if sovra.week == *aw {
                        for n in 0..vec_model_worker_names.row_count() {
                            let name = vec_model_worker_names.row_data(n).unwrap().to_string();
                            if let Some(v) = sovra.value.get_mut(n) {
                                *v = *sovra_hash.get(&name).unwrap_or(&0);
                            }
                        }
                        vec_model_sovra.set_row_data(s, sovra.into());
                        break;
                    }
                }
            }

            // --- Clear selection, mark changed ---
            if let Some(ui) = ui_weak.upgrade() {
                PjmCallback::get(&ui).set_sel_project(-1);
                PjmCallback::get(&ui).set_sel_dev(-1);
                PjmCallback::get(&ui).set_sel_anchor_week(-1);
                PjmCallback::get(&ui).set_sel_anchor_row(-1);
                PjmCallback::get(&ui).set_sel_start_week(-1);
                PjmCallback::get(&ui).set_sel_end_week(-1);
                PjmCallback::get(&ui).set_sel_start_row(-1);
                PjmCallback::get(&ui).set_sel_end_row(-1);
                PjmCallback::get(&ui).set_changed(true);
            }
        },
    );
}
