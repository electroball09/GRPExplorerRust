use super::*;

#[derive(Default)]
pub struct WorldEditor;

impl EditorImpl for WorldEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if ui.button("Export to .glb...").clicked() {
            ectx.respond(EditorResponse::GltfExport(key));
        }

        // ectx.ctx.show_viewport_immediate(
        //     egui::ViewportId::from_hash_of("test!"), 
        //     egui::ViewportBuilder::default()
        //     .with_title("test!")
        //     .with_always_on_top()
        //     .with_maximize_button(false)
        //     .with_minimize_button(false)
        //     .with_close_button(false)
        //     .with_resizable(false)
        //     .with_taskbar(false)
        //     .with_position([500.0, 200.0])
        //     .with_inner_size([300.0, 200.0]), 
        //     |ctx, _class| {
        //         egui::CentralPanel::default().show(ctx, |ui| {
        //             ui.label("test!")
        //         });
        //     }
        // );

        ui.columns(2, |cols| {
            egui::ScrollArea::vertical().auto_shrink(false).id_salt("mat_col").show(&mut cols[0], |ui| {
                for mat in _tctx.load_set.loaded_by_type(ObjectType::mat).unwrap() {
                    ui.horizontal(|ui| {
                        if ui.button(format!("{:#010X}", mat)).clicked() {
                            ectx.respond(EditorResponse::OpenNewTab(*mat));
                        }
                        ui.label(ectx.bf.file_table.get(mat).unwrap().get_name());
                    });
                }
            });

            egui::ScrollArea::vertical().auto_shrink(false).id_salt("shd_col").show(&mut cols[1], |ui| {
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