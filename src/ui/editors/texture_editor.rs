use egui::Image;

use super::*;
use crate::objects::{ObjectArchetype, TextureMetaType};
use crate::export::*;

pub struct TextureMetadataEditor<'a> {
    image: Option<Image<'a>>
}

impl Default for TextureMetadataEditor<'_> {
    fn default() -> Self {
        Self { 
            image: None
        }
    }
}

impl EditorImpl for TextureMetadataEditor<'_> {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let obj = &ectx.bf.object_table.get(&key).unwrap();
        if let ObjectArchetype::TextureMetadata(tga) = &obj.archetype {
            match &tga.meta {
                TextureMetaType::None => {
                    ui.label("unloaded texture!");
                },
                TextureMetaType::Passthrough => {
                    ui.label("this texture is a passthrough, you can export the texture by using one of the references on the left");
                },
                TextureMetaType::Metadata(meta) => {
                    ui.label(format!("width: {}", meta.width));
                    ui.label(format!("height: {}", meta.height));
                    ui.label(format!("format: {:?}", meta.format));
                    ui.label(format!("fmt id: {:#04X}", meta.fmt_id));
        
                    if ui.button("Export...").clicked() {
                        let key = obj.get_key();
                        if let Some(path) = pick_exp_path_no_ext(&ectx.bf.object_table[&key]) {
                            let txd_key = ectx.bf.object_table[&key].references[0];
                            if let Ok(_) = ectx.bf.load_file(txd_key) {
                                if let ObjectArchetype::TextureMetadata(tga) = &ectx.bf.object_table[&key].archetype {
                                    if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table[&txd_key].archetype {
                                        exp_texture(path, &tga, &txd);
                                    }
                                }
                            }
                            ectx.bf.unload_file(txd_key).unwrap();
                        }
                    }
                }
            }
        }
    }
}

pub struct TextureDataEditor;

impl EditorImpl for TextureDataEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("unk_01: {:#010X}", txd.unk_01));
            ui.label(format!("format: {:?}", txd.format));
            ui.label(format!("fmt_id: {:#04X}", txd.fmt_id));
            ui.label(format!("unk_02: {:#06X}", txd.unk_02));
            ui.label(format!("unk_03: {:#04X}", txd.unk_03));
            ui.label(format!("data_len: {}", txd.texture_data.len()));
        }
    }
}