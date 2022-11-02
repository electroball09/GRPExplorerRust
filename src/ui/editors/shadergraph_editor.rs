use super::{EditorImpl, EditorResponse};
use crate::objects::*;

pub struct ShaderGraphEditor;

impl EditorImpl for ShaderGraphEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> super::EditorResponse {
        if let ObjectArchetype::ShaderGraph(shd) = &obj.archetype {
            ui.label(format!("version: {:#06X}", shd.version));
            ui.label(format!("flags: {:#06X} {:#018b}", shd.flags, shd.flags));

            let mut i = 0;
            egui::ScrollArea::new([false, true]).auto_shrink([false, false]).show(ui, |ui| {
                for graph in &shd.graphs {
                    ui.collapsing(format!("{}", i), |ui| {
                        ui.label(format!("unk_01: {:#010X} {:#034b}", graph.unk_01, graph.unk_01));
                        ui.label(format!("unk_02: {:#010X} {:#034b}", graph.unk_02, graph.unk_02));
                        ui.label(format!("unk_03: {:#010X} {:#034b}", graph.unk_03, graph.unk_03));
                        ui.label(format!("unk_04: {:#010X} {:#034b}", graph.unk_04, graph.unk_04));
                        ui.label(format!("num_nodes: {}", graph.num_nodes));
                        ui.label(format!("unk_06: {:#010X}", graph.unk_06));
                        
                        let mut j = 0;
                        for node in &graph.nodes {
                            ui.collapsing(format!("{:#010X} {}", j, node.get_id()), |ui| {
                                ui.label(format!("{:#010X}", node.i1));
                                ui.label(format!("{:#010X}", node.i2));
                            });
                            j += 1;
                        }
                    });
                    i += 1;
                }
            });
        }

        EditorResponse::default()
    }
}