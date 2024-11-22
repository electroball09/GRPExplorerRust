use crate::objects::{ColumnData, ObjectArchetype};
use super::*;

pub struct DataTableEditor {

}

impl EditorImpl for DataTableEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let dtb = match ectx.bf.object_table.get(&key).unwrap().archetype {
            ObjectArchetype::DataTable(ref dtb2) => dtb2,
            _ => { return; }
        };

        let mut open_new_tab = None;
        egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
            egui::Grid::new("data_table").striped(true).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("");
                });
                for col in &dtb.columns {
                    ui.horizontal_centered(|ui| {
                        ui.allocate_space([15.0, 0.0].into());
                        ui.label(&col.name);
                        ui.allocate_space([15.0, 0.0].into());
                    });
                }
                ui.end_row();
    
                let mut ridx = 0;
                for row in &dtb.rows {
                    ui.horizontal(|ui| {
                        ui.label(format!("Row {}", ridx));
                    });
                    for value in &row.data {
                        ui.horizontal_centered(|ui| {
                            ui.allocate_space([15.0, 0.0].into());
                            match value {
                                ColumnData::Asset(v) => {
                                    if ectx.bf.is_key_valid_to_load(*v) {
                                        if ui.button(format!("{}", value)).clicked() {
                                            open_new_tab = Some(*v);
                                        }
                                    } else {
                                        ui.label(format!("{}", value));
                                    }
                                },
                                _ => { ui.label(format!("{}", value)); }
                            };
                            ui.allocate_space([15.0, 0.0].into());
                        });
                    }
                    ui.end_row();
                    ridx += 1;
                }
            });
        });

        if let Some(v) = open_new_tab {
            ectx.respond(EditorResponse::OpenNewTab(v));
        }
    }
}