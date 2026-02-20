use super::*;
use crate::objects::{Bone, ObjectArchetype};

pub struct SkeletonEditor {
    hierarchy_view: bool,
}

impl Default for SkeletonEditor {
    fn default() -> Self {
        Self {
            hierarchy_view: true,
        }
    }
}

impl SkeletonEditor{
    fn draw_bone_hierarchy(&self, ui: &mut egui::Ui, bones: &Vec<Bone>, idx: u8) {
        let bone = &bones[idx as usize];

        ui.collapsing(bone.get_name(), |ui| {
            ui.collapsing("     -data", |ui| {
                
                let (scl, rot, pos) = bone.bind_matrix.to_scale_rotation_translation();
                ui.label(format!("bind pos: {}", pos));
                ui.label(format!("bind rot: {}", rot));
                ui.label(format!("bind scl: {}", scl));
                let (scl, rot, pos) = bone.inv_bind_matrix.to_scale_rotation_translation();
                ui.label(format!("inv bind pos: {}", pos));
                ui.label(format!("inv bind rot: {}", rot));
                ui.label(format!("inv bind scl: {}", scl));

                let parent_byte: u8 = bone.parent.0.unwrap_or(0xFF);

                let formatted_str = bone.unk_01.iter().copied()
                    .chain(std::iter::once(parent_byte))
                    .chain(bone.data.iter().copied())
                    .collect::<Vec<u8>>()
                    .chunks_exact(8)
                    .map(|c| {
                        format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
                            c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\r\n");

                ui.label(formatted_str);
            });

            for child in &bone.children {
                self.draw_bone_hierarchy(ui, bones, *child);
            }
        });
    }
}

impl super::EditorImpl for SkeletonEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Skeleton(ske) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("version: {:#04X}", ske.version));
            ui.label(format!("num_bones: {}", ske.num_bones));
            ui.label(format!("unk_01: {:#04X}", ske.unk_01));
            ui.checkbox(&mut self.hierarchy_view, "hierarchy view");

            if ske.num_bones == 0 { 
                ui.label("no bones :( i'm just a sack of meat");
                return; 
            }

            if self.hierarchy_view {
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    self.draw_bone_hierarchy(ui, &ske.bones, 0);
                });
            } else {
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    for bone in &ske.bones {
                        ui.collapsing(bone.get_name(), |ui| {
                            ui.label(format!("parent: {}", bone.parent));

                            let parent_byte: u8 = bone.parent.0.unwrap_or(0xFF);

                            let formatted_str = bone.unk_01.iter().copied()
                                .chain(std::iter::once(parent_byte))
                                .chain(bone.data.iter().copied())
                                .collect::<Vec<u8>>()
                                .chunks_exact(8)
                                .map(|c| {
                                    format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
                                        c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\r\n");
            
                            ui.label(formatted_str);
                        });
                    }
                });
            }
        }
    }
}