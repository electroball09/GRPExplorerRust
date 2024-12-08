use super::*;
use crate::objects::ObjectArchetype;

pub struct LayerEditor;

impl EditorImpl for LayerEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        if let ObjectArchetype::Layer(layer) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.horizontal(|ui| {
                ui.label("Layer Name:");
                let mut name = layer.name.clone();
                ui.text_edit_singleline(&mut name);
            });
        }
    }
}