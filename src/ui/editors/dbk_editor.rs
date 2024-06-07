use crate::objects::ObjectArchetype;

use super::{EditorImpl, EditorResponse};

pub struct DbkEditor;

impl EditorImpl for DbkEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Dbk(dbk) = &obj.archetype {
            ui.label(format!("bank id: {:#04X} ({})", dbk.bank_id, dbk.bank_id));
            ui.label(format!("num entries: {}", dbk.num_bank_entries));
        }

        EditorResponse::default()
    }
}

pub struct DbrEditor;

impl EditorImpl for DbrEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        EditorResponse::default()
    }
}