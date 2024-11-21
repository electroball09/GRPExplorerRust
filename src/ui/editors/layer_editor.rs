use super::*;
use crate::objects::ObjectArchetype;

pub struct LayerEditor;

impl EditorImpl for LayerEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::Layer(layer) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.horizontal(|ui| {
                ui.label("Layer Name:");
                let mut name = layer.name.clone();
                ui.text_edit_singleline(&mut name);
            });
        }
    }
}