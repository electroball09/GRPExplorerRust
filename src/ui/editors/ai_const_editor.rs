use crate::objects::{ObjectArchetype, ai_const::ConstTreeNode};

use super::*;

pub struct AIConstEditor;

impl EditorImpl for AIConstEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::ConstList(cst) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            fn recurse(ui: &mut egui::Ui, node: &ConstTreeNode) {
                for sub in node.nodes.iter() {
                    ui.collapsing(sub.get_name(), |ui| {
                        recurse(ui, sub)
                    });
                }
                for value in node.values.iter() {
                    ui.label(format!("{}", value));
                }
            }

            egui::ScrollArea::new([false, true]).auto_shrink([false, false]).show(ui, |ui| {
                recurse(ui, &cst.root_node);
            });
        }
    }
}