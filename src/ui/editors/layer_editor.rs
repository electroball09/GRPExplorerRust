use super::*;
use crate::objects::ObjectArchetype;

pub struct LayerEditor;

impl EditorImpl for LayerEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        if let ObjectArchetype::Layer(layer) = &obj.archetype {
            ui.horizontal(|ui| {
                ui.label("Layer Name:");
                let mut name = layer.name.clone();
                ui.text_edit_singleline(&mut name);
            });
        }
    }
}