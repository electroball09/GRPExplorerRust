use std::fs::File;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use image::ColorType;

use crate::objects::texture::TextureFormat;
use crate::objects::{ObjectArchetype, YetiObject};
use crate::util::dds_header::*;

use super::{EditorImpl, EditorResponse};

pub struct TextureMetadataEditor;

impl EditorImpl for TextureMetadataEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> super::EditorResponse {
        if let ObjectArchetype::TextureMetadata(tga) = &obj.archetype {
            ui.label(format!("width: {}", tga.width));
            ui.label(format!("height: {}", tga.height));
            ui.label(format!("format: {:?}", tga.format));
            ui.label(format!("fmt id: {:#04X}", tga.fmt_id));

            if ui.button("Export...").clicked() {
                return EditorResponse::PerformAction(obj.get_key(), Box::new(|key, bf| {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let path = path.to_str().unwrap();

                        let txd_key = bf.object_table[&key].references[0];
                        if let Ok(()) = bf.load_file(txd_key) {
                            if let ObjectArchetype::TextureMetadata(tga) = &bf.object_table[&key].archetype {
                                if let ObjectArchetype::TextureData(txd) = &bf.object_table[&txd_key].archetype {
                                    if let TextureFormat::Dxt5 = &txd.format {
                                        let path = format!("{}\\{}.dds", path, bf.file_table[&key].get_name());
                                        let mut file = File::create(path).unwrap();
                                        let dds = DdsHeader::dxt5(tga.height.into(), tga.width.into());
                                        dds.write_to(&mut file).unwrap();
                                        file.write(&txd.texture_data[..]).unwrap();
                                    } else if let TextureFormat::Dxt1 = &txd.format {
                                        let path = format!("{}\\{}.dds", path, bf.file_table[&key].get_name());
                                        let mut file = File::create(path).unwrap();
                                        let dds = DdsHeader::dxt1(tga.height.into(), tga.width.into());
                                        dds.write_to(&mut file).unwrap();
                                        file.write(&txd.texture_data[..]).unwrap();
                                    } else if let TextureFormat::Rgba32 = &txd.format {
                                        let path = format!("{}\\{}.bmp", path, bf.file_table[&key].get_name());
                                        image::save_buffer(path, &txd.texture_data, tga.width as u32, tga.height as u32, ColorType::Rgba8).unwrap();
                                    } else if let TextureFormat::Bgra32 = &txd.format {
                                        let path = format!("{}\\{}.bmp", path, bf.file_table[&key].get_name());
                                        image::save_buffer(path, &txd.texture_data, tga.width as u32, tga.height as u32, ColorType::Rgba8).unwrap();
                                    } else {
                                        println!("unsupported texture format");
                                    }
                                }
                            }


                        }
                        bf.unload_file(txd_key).unwrap();
                    }
                }));
            }
        }

        EditorResponse::None
    }
}

pub struct TextureDataEditor;

impl EditorImpl for TextureDataEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> EditorResponse {
        if let ObjectArchetype::TextureData(txd) = &obj.archetype {
            ui.label(format!("unk_01: {:#010X}", txd.unk_01));
            ui.label(format!("format: {:?}", txd.format));
            ui.label(format!("fmt_id: {:#04X}", txd.fmt_id));
            ui.label(format!("unk_02: {:#06X}", txd.unk_02));
            ui.label(format!("unk_03: {:#04X}", txd.unk_03));
            ui.label(format!("data_len: {}", txd.texture_data.len()));
        }

        EditorResponse::None
    }
}