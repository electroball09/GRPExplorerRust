use super::*;

#[derive(Default)]
pub struct WorldEditor;

impl EditorImpl for WorldEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }

        ui.columns(2, |cols| {
            egui::ScrollArea::vertical().id_salt("mat_col").show(&mut cols[0], |ui| {
                for mat in _tctx.load_set.loaded_by_type(ObjectType::mat).unwrap() {
                    ui.horizontal(|ui| {
                        if ui.button(format!("{:#010X}", mat)).clicked() {
                            ectx.respond(EditorResponse::OpenNewTab(*mat));
                        }
                        ui.label(ectx.bf.file_table.get(mat).unwrap().get_name());
                    });
                }
            });

            egui::ScrollArea::vertical().id_salt("shd_col").show(&mut cols[1], |ui| {
                for shd in _tctx.load_set.loaded_by_type(ObjectType::shd).unwrap() {
                    ui.horizontal(|ui| {
                        if ui.button(format!("{:#010X}", shd)).clicked() {
                            ectx.respond(EditorResponse::OpenNewTab(*shd));
                        }
                        ui.label(ectx.bf.file_table.get(shd).unwrap().get_name());
                    });
                }
            })
        });
    }
}