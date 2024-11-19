use super::*;
use crate::objects::ObjectArchetype;

pub struct OtfEditor;

impl super::EditorImpl for OtfEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        if let ObjectArchetype::Otf(_otf) = &obj.archetype { 
            ui.label("Extract to view font");
        }
    }
}