use super::*;

#[derive(Default)]
pub struct CollisionObjectEditor;

impl EditorImpl for CollisionObjectEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }
    }
}