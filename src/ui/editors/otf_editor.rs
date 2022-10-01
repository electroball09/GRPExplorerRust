use crate::objects::ObjectArchetype;

use super::EditorResponse;

pub struct OtfEditor;

impl super::Editor for OtfEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Otf(otf) = &obj.archetype { 
            ui.label("Extract to view font");
        }

        EditorResponse::default()
    }
}