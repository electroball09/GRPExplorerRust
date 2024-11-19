use crate::objects::ObjectArchetype;
use super::*;

pub struct DbkEditor;

impl EditorImpl for DbkEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        if let ObjectArchetype::Dbk(dbk) = &obj.archetype {
            ui.label(format!("bank id: {:#04X} ({})", dbk.bank_id, dbk.bank_id));
            ui.label(format!("num entries: {}", dbk.num_bank_entries));
        }
    }
}

pub struct DbrEditor;

impl EditorImpl for DbrEditor {
    fn draw(&mut self, _obj: &mut YetiObject, _ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        
    }
}