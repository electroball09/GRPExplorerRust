use super::*;
use crate::objects::*;

pub struct ScriptEditor;

impl EditorImpl for ScriptEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Script(script) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            let mut buf = script.buffer.clone();
            buf.pop(); //nul terminator
            let mut script_string = String::from_utf8(buf).unwrap();
            ui.centered_and_justified(|ui| {
                ui.code_editor(&mut script_string);
            });
        }
    }
}