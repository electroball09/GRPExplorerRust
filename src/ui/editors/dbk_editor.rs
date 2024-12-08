use crate::objects::ObjectArchetype;
use super::*;

pub struct DbkEditor;

impl EditorImpl for DbkEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Dbk(dbk) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("bank id: {:#04X} ({})", dbk.bank_id, dbk.bank_id));
            ui.label(format!("num entries: {}", dbk.num_bank_entries));
        }
    }
}

pub struct DbrEditor;

impl EditorImpl for DbrEditor {
    fn draw(&mut self, _key: YKey, _ui: &mut egui::Ui, _ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        
    }
}