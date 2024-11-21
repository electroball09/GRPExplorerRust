use super::*;
use crate::objects::{ObjectArchetype, TextureFormat, TextureMetaType};
use crate::export::*;

pub struct TextureMetadataEditor {
    texture: Option<egui::TextureHandle>,
}

impl Default for TextureMetadataEditor {
    fn default() -> Self {
        Self { 
            texture: None
        }
    }
}

impl TextureMetadataEditor {
    fn from_brga(size: [usize; 2], brga: &[u8]) -> egui::ColorImage {
        assert_eq!(size[0] * size[1] * 4, brga.len());
        let pixels = brga
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_premultiplied(p[1], p[2], p[0], p[3]))
            .collect();
        egui::ColorImage { size, pixels }
    }
}

impl EditorImpl for TextureMetadataEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let mut mtype = TextureMetaType::None;
        if let ObjectArchetype::TextureMetadata(tga) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            mtype = tga.meta;
        }
        let mut image_rot_angle = 0.0;
        match mtype {
            TextureMetaType::None => {
                ui.label("unloaded texture!");
            },
            TextureMetaType::Passthrough => {
                ui.label("this texture is a passthrough, you can export the texture by using one of the references on the left");
            },
            TextureMetaType::Metadata(meta) => {
                let txd_key = ectx.bf.object_table[&key].references[0];
                if let None = self.texture {
                    if let Some(data) = match meta.format {
                        TextureFormat::Bgra32 => {
                            let mut v = None;
                            if let Ok(_) = ectx.bf.load_file(txd_key) {
                                if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table[&txd_key].archetype {
                                    v = Some(TextureMetadataEditor::from_brga([meta.width.into(), meta.height.into()], &txd.texture_data))
                                }
                                let _ = ectx.bf.unload_file(txd_key);
                            }
                            v
                        },
                        TextureFormat::Rgba32 => {
                            let mut v = None;
                            if let Ok(_) = ectx.bf.load_file(txd_key) {
                                if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table[&txd_key].archetype {
                                    v = Some(egui::ColorImage::from_rgba_premultiplied([meta.width.into(), meta.height.into()], &txd.texture_data))
                                }
                                let _ = ectx.bf.unload_file(txd_key);
                            }
                            v
                        },
                        // TextureFormat::Gray => {
                        //     let mut v = None;
                        //     if let Ok(_) = ectx.bf.load_file(txd_key) {
                        //         if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table[&txd_key].archetype {
                        //             v = Some(egui::ColorImage::from_gray([meta.width.into(), meta.height.into()], &txd.texture_data))
                        //         }
                        //         let _ = ectx.bf.unload_file(txd_key);
                        //     }
                        //     v
                        // },
                        TextureFormat::Dxt1 => {
                            let mut v = None;
                            if let Ok(_) = ectx.bf.load_file(txd_key) {
                                if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table[&txd_key].archetype {
                                    let f = texpresso::Format::Bc1;
                                    let w: usize = meta.width.into();
                                    let h: usize = meta.height.into();
                                    let mut buf: Vec<u8> = vec![0; w * h * 4];
                                    f.decompress(&txd.texture_data, w, h, &mut buf);
                                    let buf: Vec<u8> = buf.chunks_exact(4).map(|x| [x[0], x[1], x[2], x[3]]).collect::<Vec<[u8; 4]>>().iter().flat_map(|x| *x).collect();
                                    v = Some(egui::ColorImage::from_rgba_premultiplied([w, h], &buf))
                                }
                                let _ = ectx.bf.unload_file(txd_key);
                            }
                            v
                        },
                        TextureFormat::Dxt5 => {
                            let mut v = None;
                            if let Ok(_) = ectx.bf.load_file(txd_key) {
                                if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table[&txd_key].archetype {
                                    let f = texpresso::Format::Bc3;
                                    let w: usize = meta.width.into();
                                    let h: usize = meta.height.into();
                                    let mut buf: Vec<u8> = vec![0; w * h * 4];
                                    f.decompress(&txd.texture_data, w, h, &mut buf);
                                    let buf: Vec<u8> = buf.chunks_exact(4).map(|x| [x[0], x[1], x[2], x[3]]).collect::<Vec<[u8; 4]>>().iter().flat_map(|x| *x).collect();
                                    v = Some(egui::ColorImage::from_rgba_premultiplied([w, h], &buf))
                                }
                                let _ = ectx.bf.unload_file(txd_key);
                            }
                            v
                        },
                        _ => { None }
                    } {
                        let tex = ectx.ctx.load_texture(format!("{:#010X}", key), data, Default::default());
                        self.texture = Some(tex);
                    }
                }

                ui.label(format!("width: {}", meta.width));
                ui.label(format!("height: {}", meta.height));
                ui.label(format!("format: {:?}", meta.format));
                ui.label(format!("fmt id: {:#04X}", meta.fmt_id));
    
                if ui.button("Export...").clicked() {
                    let obj = &ectx.bf.object_table.get(&key).unwrap();
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

                if let Some(tex) = &self.texture {
                    egui::Widget::ui(egui::Image::new(tex).max_size([500.0, 500.0].into()), ui);
                } else {
                    ui.label("texture not loaded or wrong texture format!");
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