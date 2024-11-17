use super::*;
use crate::objects::ObjectArchetype;

use super::{EditorImpl, EditorResponse};

pub struct LayerEditor;

impl EditorImpl for LayerEditor {
    fn draw(&mut self, obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, _ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::Layer(layer) = &obj.archetype {
            ui.horizontal(|ui| {
                ui.label("Layer Name:");
                let mut name = layer.name.clone();
                ui.text_edit_singleline(&mut name);
            });
        }

        EditorResponse::default()
    }
}