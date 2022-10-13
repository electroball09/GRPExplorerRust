use super::*;

pub struct BlankEditor;

impl EditorImpl for BlankEditor {
    fn draw(_obj: &mut YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        ui.label("editor not implemented yet!");
        EditorResponse::default()
    }
}