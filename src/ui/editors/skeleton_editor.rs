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
                for (i, bone) in ske.bones.iter().enumerate() {
                    ui.collapsing(format!("{} - {}", i, bone.get_name()), |ui| {
                        let str = bone.data.chunks_exact(8).map(|c| format!("{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])).collect::<Vec<String>>().join("\r\n");
                        ui.label(str);
                    });
                }
            });
        }
    }
}