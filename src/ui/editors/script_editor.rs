use super::{EditorImpl, EditorResponse};
use crate::objects::*;

pub struct ScriptEditor;

impl EditorImpl for ScriptEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Script(script) = &obj.archetype {
            let mut buf = script.buffer.clone();
            buf.pop(); //nul terminator
            let mut script_string = String::from_utf8(buf).unwrap();
            ui.centered_and_justified(|ui| {
                ui.code_editor(&mut script_string);
            });
        }

        EditorResponse::default()
    }
}