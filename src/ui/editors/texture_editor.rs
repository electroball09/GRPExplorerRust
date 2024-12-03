use egui::Widget;

use super::*;
use crate::objects::{ObjectArchetype, TextureMetaType};
use crate::export::*;
use crate::util::texture_util;

pub struct TextureMetadataEditor {
    texture: Option<egui::TextureHandle>,
    tex_size: egui::Vec2,
    zoom: f32,
}

impl Default for TextureMetadataEditor {
    fn default() -> Self {
        Self { 
            texture: None,
            tex_size: [100.0, 100.0].into(),
            zoom: 1.0,
        }
    }
}

// impl TextureMetadataEditor {
//     fn from_brga(size: [usize; 2], brga: &[u8]) -> egui::ColorImage {
//         assert_eq!(size[0] * size[1] * 4, brga.len());
//         let pixels = brga
//             .chunks_exact(4)
//             .map(|p| egui::Color32::from_rgba_premultiplied(p[1], p[2], p[0], p[3]))
//             .collect();
//         egui::ColorImage { size, pixels }
//     }
// }

impl EditorImpl for TextureMetadataEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        let mut mtype = TextureMetaType::None;
        if let ObjectArchetype::TextureMetadata(tga) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            mtype = tga.meta;
        }
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
                    ectx.bf.load_file(txd_key).unwrap();
                    if let ObjectArchetype::TextureData(txd) = &ectx.bf.object_table.get(&txd_key).unwrap().archetype {
                        let data = texture_util::decompress_texture(&meta, txd);
                        let image = egui::ColorImage::from_rgba_premultiplied([meta.width.into(), meta.height.into()], &data);
                        let tex = ectx.ctx.load_texture(format!("{:#010X}", key), image, Default::default());
                        self.texture = Some(tex);
                        self.tex_size = [meta.width as f32, meta.height as f32].into();
                    }
                }
                
                egui::SidePanel::left("tex_panel").resizable(false).exact_width(100.0).show_inside(ui, |ui| {
                    ui.label(format!("unk_01: {}", meta.unk_01));
                    ui.label(format!("width: {}", meta.width));
                    ui.label(format!("height: {}", meta.height));
                    ui.label(format!("unk_02: {}", meta.unk_02));
                    ui.label(format!("format: {:?}", meta.format));
                    ui.label(format!("tex type?: {:#06X}", meta.mb_type_indicator));
        
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
                });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if let Some(tex) = &self.texture {
                        ui.centered_and_justified(|ui| {
                            let rsp = egui::Widget::ui(egui::Image::new(tex).fit_to_exact_size(self.tex_size * self.zoom), ui);
                            if rsp.hovered() {
                                let delta = ui.input(|i| {
                                    i.events.iter().find_map(|e| match e {
                                        egui::Event::MouseWheel {
                                            unit: _,
                                            delta,
                                            modifiers: _,
                                        } => Some(*delta),
                                        _ => None,
                                    })
                                });
                                if let Some(delta) = delta {
                                    let mut zoom = self.zoom;
                                    zoom += delta.y * 0.1;
                                    zoom *= 10.0;
                                    zoom = zoom.floor() / 10.0;
                                    self.zoom = zoom;
                                }
                            }
                        });
                    } else {
                        ui.label("texture not loaded or wrong texture format!");
                    }
                });
                egui::TopBottomPanel::bottom("tex_controls").exact_height(50.0).show_inside(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        egui::Slider::new(&mut self.zoom, 0.1..=4.0).ui(ui);
                    });
                });
            }
        }
    }
}

pub struct TextureDataEditor;

impl EditorImpl for TextureDataEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
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