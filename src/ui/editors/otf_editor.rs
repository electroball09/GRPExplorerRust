use super::*;
use crate::objects::ObjectArchetype;

pub struct OtfEditor;

impl super::EditorImpl for OtfEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::Otf(_otf) = &ectx.bf.object_table.get(&key).unwrap().archetype { 
            ui.label("Extract to view font");
        }
    }
}