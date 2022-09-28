use crate::objects::ObjectArchetype;

use super::Editor;

pub struct LayerEditor;

impl Editor for LayerEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let ObjectArchetype::Layer(layer) = &obj.archetype {
            ui.horizontal(|ui| {
                ui.label("Layer Name:");
                let mut name = layer.name.clone();
                ui.text_edit_singleline(&mut name);
            });
        }
    }
}