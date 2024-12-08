use super::*;
use crate::objects::{ObjectArchetype, SnkType};

pub struct SnkEditor;

impl EditorImpl for SnkEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::SoundBank(snk) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            match &snk.snk_type {
                SnkType::Unknown(v) => {
                    ui.label(format!("Unknown SnkType: {:#04X}, this probably means the bin_name is wrong!", v));
                },
                SnkType::Type0 => {
                    ui.label("SnkType 0");
                },
                SnkType::Type1(v) => {
                    ui.label(format!("SnkType 1: {:#010X}", v));
                },
                SnkType::Type2 => {
                    ui.label("SnkType 2");
                },
                SnkType::Type3(v) => {
                    ui.label(format!("SnkType 3: {:#010X}", v));
                },
                SnkType::Type8 => {
                    ui.label("SnkType 8");
                }
            }

            ui.label(format!("bin_name: {}", &snk.bin_name));

            ui.collapsing(format!("num entries: {}", snk.entries.len()), |ui| {
                egui::ScrollArea::new([false, true]).auto_shrink([false, false]).show(ui, |ui| {
                    for ent in &snk.entries {
                        ui.collapsing(format!("{:#04X} {}", ent.id, &ent.name), |ui| {
                            ui.label(format!("id: {:#04X}", ent.id));
                            ui.label(format!("data: {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}",
                                    ent.unk00, ent.unk01, ent.unk02, ent.unk03, ent.unk04, ent.unk05, ent.unk06, ent.unk07, ent.unk08, ent.unk09));
                        });
                    }
                })
            });
        }
    }
}