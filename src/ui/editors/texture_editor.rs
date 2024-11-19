use super::*;
use crate::objects::ObjectArchetype;
use crate::export::*;

pub struct TextureMetadataEditor;

impl EditorImpl for TextureMetadataEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        if let ObjectArchetype::TextureMetadata(tga) = &obj.archetype {
            ui.label(format!("width: {}", tga.width));
            ui.label(format!("height: {}", tga.height));
            ui.label(format!("format: {:?}", tga.format));
            ui.label(format!("fmt id: {:#04X}", tga.fmt_id));

            if ui.button("Export...").clicked() {
                ectx.respond(EditorResponse::PerformAction(obj.get_key(), Box::new(|key, bf| {
                    if let Some(path) = pick_exp_path_no_ext(&bf.object_table[&key]) {
                        let txd_key = bf.object_table[&key].references[0];
                        if let Ok(()) = bf.load_file(txd_key) {
                            if let ObjectArchetype::TextureMetadata(tga) = &bf.object_table[&key].archetype {
                                if let ObjectArchetype::TextureData(txd) = &bf.object_table[&txd_key].archetype {
                                    exp_texture(path, &tga, &txd);
                                }
                            }
                        }
                        bf.unload_file(txd_key).unwrap();
                    }
                })));
            }
        }
    }
}

pub struct TextureDataEditor;

impl EditorImpl for TextureDataEditor {
    fn draw(&mut self, obj: &mut YetiObject, ui: &mut egui::Ui, _ectx: &mut EditorContext) {
        if let ObjectArchetype::TextureData(txd) = &obj.archetype {
            ui.label(format!("unk_01: {:#010X}", txd.unk_01));
            ui.label(format!("format: {:?}", txd.format));
            ui.label(format!("fmt_id: {:#04X}", txd.fmt_id));
            ui.label(format!("unk_02: {:#06X}", txd.unk_02));
            ui.label(format!("unk_03: {:#04X}", txd.unk_03));
            ui.label(format!("data_len: {}", txd.texture_data.len()));
        }
    }
}