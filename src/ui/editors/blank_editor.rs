use super::*;

pub struct BlankEditor { }

impl Editor for BlankEditor {
    fn draw(obj: &mut YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("editor not implemented yet!");
    }
}