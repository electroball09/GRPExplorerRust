use super::Editor;
use crate::objects::*;

pub struct ScriptEditor {

}

impl ScriptEditor {

}

impl Editor for ScriptEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let ObjectArchetype::Script(script) = &obj.archetype {
            let mut buf = script.buffer.clone();
            buf.pop(); //nul terminator
            let mut script_string = String::from_utf8(buf).unwrap();
            ui.text_edit_multiline(&mut script_string);
        } else {
            ui.label(format!("wrong editor ???"));
        }
    }
}