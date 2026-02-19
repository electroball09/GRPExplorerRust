use std::io::Cursor;

use byteorder::ReadBytesExt;

use super::*;
use crate::objects::{Bone, MATRIX_SEARCH_MAX, ObjectArchetype};
use crate::util::load_util::*;

pub struct SkeletonEditor {
    hierarchy_view: bool,
    matrix_search_offset: usize,
    found_valid_matrix_offsets: Option<Vec<usize>>,
}

impl Default for SkeletonEditor {
    fn default() -> Self {
        Self {
            hierarchy_view: true,
            matrix_search_offset: 0,
            found_valid_matrix_offsets: None,
        }
    }
}

impl SkeletonEditor{
    fn draw_bone_hierarchy(&self, ui: &mut egui::Ui, bones: &Vec<Bone>, idx: u8) {
        let bone = &bones[idx as usize];

        ui.collapsing(bone.get_name(), |ui| {
            ui.collapsing("     -data", |ui| {

                let mut cursor = Cursor::new(bone.data.as_slice());
                for _ in 0..self.matrix_search_offset {
                    cursor.read_u8().unwrap();
                }
                let mat = read_mat4(&mut cursor);

                let (scl, rot, pos) = mat.unwrap().to_scale_rotation_translation();
                ui.label(format!("pos: {}", pos));
                ui.label(format!("rot: {}", rot));
                ui.label(format!("scl: {}", scl));

                // holy fuck
                let str = bone.unk_01.iter().chain([Into::<Option<u8>>::into(bone.parent).unwrap_or(0xFF)].iter()).chain(bone.data.iter()).map(|b| *b).collect::<Vec<u8>>()
                    .chunks_exact(8).map(|c| format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])).collect::<Vec<String>>().join("\r\n");

                ui.label(str);
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

            if let None = self.found_valid_matrix_offsets {
                self.found_valid_matrix_offsets = Some(ske.find_valid_matrix_offsets());
            }

            let valid_offsets = self.found_valid_matrix_offsets.clone().unwrap();
            if valid_offsets.is_empty() {
                ui.label("No valid matrix offsets found!");
                return;
            } else {
                ui.label(format!("valid offsets: {}", valid_offsets.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")));
                self.found_valid_matrix_offsets = Some(valid_offsets);
            }

            ui.add(egui::Slider::new(&mut self.matrix_search_offset, 0..=MATRIX_SEARCH_MAX).integer());

            if ske.num_bones == 0 { return; }

            if self.hierarchy_view {
                egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                    self.draw_bone_hierarchy(ui, &ske.bones, 0);
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