use super::*;

pub struct BlankEditor;

impl EditorImpl for BlankEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        match &ectx.bf.object_table.get(&key).unwrap().archetype {
            ObjectArchetype::NoImpl => {
                ui.label("no object implementation yet!");
            },
            _ => {
                ui.label("this object has has object impl but no editor implemented/hooked up!");
            }
        };
    }
}