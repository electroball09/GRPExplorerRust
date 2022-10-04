use crate::objects::ObjectArchetype;

use super::{Editor, EditorResponse};

pub struct TextureMetadataEditor;

impl Editor for TextureMetadataEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> super::EditorResponse {
        if let ObjectArchetype::TextureMetadata(tga) = &obj.archetype {
            ui.label(format!("width: {}", tga.width));
            ui.label(format!("height: {}", tga.height));
            ui.label(format!("format: {:?}", tga.format));
            ui.label(format!("fmt id: {:#04X}", tga.fmt_id));
        }

        EditorResponse::default()
    }
}

pub struct TextureDataEditor;

impl Editor for TextureDataEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        EditorResponse::default()
    }
}