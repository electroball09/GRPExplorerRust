use super::*;
use crate::objects::ObjectArchetype;

pub struct SkeletonEditor;

impl super::EditorImpl for SkeletonEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Skeleton(ske) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("version: {:#04X}", ske.version));
            ui.label(format!("num_bones: {}", ske.num_bones));
            ui.label(format!("unk_01: {:#04X}", ske.unk_01));

            egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui| {
                let mut i = 0;
                for bone in &ske.bones {
                    ui.collapsing(format!("{} - {}", i, bone.get_name()), |ui| {
                        ui.label(format!("unk_00: {}", bone.unk_00));
                        ui.label(format!("unk_01: {:#04X}", bone.unk_01));
                        ui.label(format!("unk_02: {:#04X}", bone.unk_02));
                        ui.label(format!("unk_03: {:#04X}", bone.unk_03));
                        ui.collapsing("Floats", |ui| {
                            for chunk in bone.floats.chunks_exact(4) {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{:10.5}", chunk[0]));
                                    ui.label(format!("{:10.5}", chunk[1]));
                                    ui.label(format!("{:10.5}", chunk[2]));
                                    ui.label(format!("{:10.5}", chunk[3]));
                                });
                            }
                            // let mut j = 0;
                            // while j < 48 {
                            //     ui.label(format!("{}: {}", j, bone.floats[j]));
                            //     j += 1;
                            // }
                        });
                    });
                    i += 1;
                }
            });
        }
    }
}