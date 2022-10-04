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
        if let ObjectArchetype::TextureData(txd) = &obj.archetype {
            ui.label(format!("unk_01: {:#010X}", txd.unk_01));
            ui.label(format!("format: {:?}", txd.format));
            ui.label(format!("fmt_id: {:#04X}", txd.fmt_id));
            ui.label(format!("unk_02: {:#06X}", txd.unk_02));
            ui.label(format!("unk_03: {:#04X}", txd.unk_03));
            ui.label(format!("data_len: {}", txd.texture_data.len()));
        }

        EditorResponse::default()
    }
}