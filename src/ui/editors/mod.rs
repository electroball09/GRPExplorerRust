mod script_editor;
mod blank_editor;

pub use script_editor::*;
pub use blank_editor::*;

pub trait Editor {
    fn draw(&self, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub fn get_editor_for_type(obj_type: crate::ObjectType) -> impl Editor {
    match obj_type {
        _ => blank_editor::BlankEditor { }
    }
}