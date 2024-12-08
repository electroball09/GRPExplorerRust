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

impl super::EditorImpl for SkeletonEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Skeleton(ske) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("version: {:#04X}", ske.version));
            ui.label(format!("num_bones: {}", ske.num_bones));
            ui.label(format!("unk_01: {:#04X}", ske.unk_01));
            ui.checkbox(&mut self.hierarchy_view, "hierarchy view");

            if ske.num_bones == 0 { return; }

            if self.hierarchy_view {
                fn draw_bone(ui: &mut egui::Ui, bones: &Vec<Bone>, idx: u8) {
                    let bone = &bones[idx as usize];
    
                    ui.collapsing(bone.get_name(), |ui| {
                        ui.collapsing("     -data", |ui| {
                            // holy fuck
                            let str = bone.unk_01.iter().chain([Into::<Option<u8>>::into(bone.parent).unwrap_or(0xFF)].iter()).chain(bone.data.iter()).map(|b| *b).collect::<Vec<u8>>()
                                .chunks_exact(8).map(|c| format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])).collect::<Vec<String>>().join("\r\n");
            
                            ui.label(str);
                        });
    
                        for child in &bone.children {
                            draw_bone(ui, bones, *child);
                        }
                    });
                }
    
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    draw_bone(ui, &ske.bones, 0);
                });
            } else {
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    for bone in &ske.bones {
                        ui.collapsing(bone.get_name(), |ui| {
                            ui.label(format!("parent: {}", bone.parent));

                            // holy fuck
                            let str = bone.unk_01.iter().chain([Into::<Option<u8>>::into(bone.parent).unwrap_or(0xFF)].iter()).chain(bone.data.iter()).map(|b| *b).collect::<Vec<u8>>()
                                .chunks_exact(8).map(|c| format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])).collect::<Vec<String>>().join("\r\n");
            
                            ui.label(str);
                        });
                    }
                });
            }
        }
    }
}