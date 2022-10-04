use crate::objects::{ObjectArchetype, ConstTreeNode};

use super::{Editor, EditorResponse};

pub struct AIConstEditor;

impl Editor for AIConstEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::ConstList(cst) = &obj.archetype {
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

        EditorResponse::default()
    }
}