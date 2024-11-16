use super::*;
use crate::objects::ObjectArchetype;

use super::EditorResponse;

pub struct OtfEditor;

impl super::EditorImpl for OtfEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Otf(_otf) = &obj.archetype { 
            ui.label("Extract to view font");
        }

        EditorResponse::default()
    }
}