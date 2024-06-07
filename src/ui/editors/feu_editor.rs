use crate::objects::ObjectArchetype;
use super::{EditorImpl, EditorResponse};
use crate::export::*;

pub struct FeuEditor;

impl EditorImpl for FeuEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Feu(feu) = &obj.archetype {
            ui.label(format!("unk_01: {}", feu.unk_01));
            ui.label(format!("unk_02: {}", feu.unk_02));
            ui.add_space(5.0);
            ui.label(format!("fire refs: {}", feu.feu_refs.len()));
            egui::ScrollArea::new([false, true]).auto_shrink([false, true]).max_height(350.0).show(ui,|ui| {
                for fref in feu.feu_refs.iter() {
                    ui.label(fref);
                }
            });
            ui.add_space(5.0);
            ui.label(format!("data len: {}", feu.feu_data.len()));

            if ui.button("Export to SWF...").clicked() {
                if let Some(path) = pick_exp_path(obj, ".swf") {
                    exp_feu(path, &feu);
                }
            }
            // if ui.button("Export to FEU...").clicked() {
            //     if let Some(path) = pick_exp_path(obj, ".feu") {
            //         exp_feu(path, &feu);
            //     }
            // }
        }

        EditorResponse::default()
    }
}