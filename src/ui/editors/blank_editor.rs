use super::*;

pub struct BlankEditor;

impl EditorImpl for BlankEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        match &obj.archetype {
            ObjectArchetype::NoImpl => {
                ui.label("no object implementation yet!");
            },
            _ => {
                ui.label("this object has has object impl but no editor implemented/hooked up!");
            }
        };
    }
}