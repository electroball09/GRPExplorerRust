use super::*;

#[derive(Default)]
pub struct WorldEditor;

impl EditorImpl for WorldEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }
    }
}