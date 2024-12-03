use egui::Widget;

use super::*;
use crate::objects::{ObjectArchetype, TextureFormat, TextureMetaType};
use crate::export::*;
use crate::util::texture_util;

pub struct TextureMetadataEditor {
    texture: Option<egui::TextureHandle>,
    tex_data: Option<Vec<u8>>,
    tex_size: egui::Vec2,
    zoom: f32,
}

impl Default for TextureMetadataEditor {
    fn default() -> Self {
        Self { 
            texture: None,
            tex_data: None,
            tex_size: [100.0, 100.0].into(),
            zoom: 1.0,
        }
    }
}

impl TextureMetadataEditor {
    fn from_rgba(size: [usize; 2], brga: &[u8]) -> Result<egui::ColorImage, String> {
        let expected_size = size[0] * size[1] * 4;
        if expected_size != brga.len() {
            return Err(format!("unexpected size: expected {} != len {}", expected_size, brga.len()))
        }
        let pixels = brga
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        Ok(egui::ColorImage { size, pixels })
    }

    fn from_brga(size: [usize; 2], brga: &[u8]) -> Result<egui::ColorImage, String> {
        let expected_size = size[0] * size[1] * 4;
        if expected_size != brga.len() {
            return Err(format!("unexpected size: expected {} != len {}", expected_size, brga.len()))
        }
        let pixels = brga
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_premultiplied(p[1], p[2], p[0], p[3]))
            .collect();
        Ok(egui::ColorImage { size, pixels })
    }

    fn from_normal_map(size: [usize; 2], brga: &[u8]) -> Result<egui::ColorImage, String> {
        let expected_size = size[0] * size[1] * 4;
        if expected_size != brga.len() {
            return Err(format!("unexpected size: expected {} != len {}", expected_size, brga.len()))
        }
        let pixels = brga
            .chunks_exact(4)
            .map(|p| {
                let r = p[1];
                let g = p[3];
                let b = p[2];
                let a = 255; // clear alpha channel
                egui::Color32::from_rgba_premultiplied(r,g,b,a)
            }).collect();
        Ok(egui::ColorImage { size, pixels })
    }

    fn image_to_buf(img: &egui::ColorImage) -> Vec<u8> {
        img.pixels.iter().flat_map(|img| [img.r(), img.g(), img.b(), img.a()]).collect()
    }
}

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
                        let size = [meta.width as usize, meta.height as usize];
                        let image = {
                            match meta.format {
                                TextureFormat::Bgra8 => TextureMetadataEditor::from_brga(size, &data),
                                _ => {
                                    if meta.is_normal_map() {
                                        TextureMetadataEditor::from_normal_map(size, &data)
                                    } else {
                                        TextureMetadataEditor::from_rgba(size, &data)
                                    }
                                }
                            }
                        };
                        if let Ok(image) = image {
                            self.tex_data = Some(TextureMetadataEditor::image_to_buf(&image));
                            let tex = ectx.ctx.load_texture(format!("{:#010X}", key), image, Default::default());
                            self.texture = Some(tex);
                            self.tex_size = [meta.width as f32, meta.height as f32].into();
                        }
                    }
                }
                
                egui::SidePanel::left("tex_panel").resizable(false).exact_width(100.0).show_inside(ui, |ui| {
                    ui.label(format!("unk_01: {}", meta.unk_01));
                    ui.label(format!("width: {}", meta.width));
                    ui.label(format!("height: {}", meta.height));
                    ui.label(format!("unk_02: {}", meta.unk_02));
                    ui.label(format!("format: {:?}", meta.format));
                    ui.label(format!("tex type?: {:#06X}", meta.mb_type_indicator));

                    if let Some(ref data) = self.tex_data {
                        if ui.button("Export...").clicked() {
                            let obj = &ectx.bf.object_table.get(&key).unwrap();
                            let key = obj.get_key();
                            if let Some(path) = pick_exp_path_no_ext(&ectx.bf.object_table[&key]) {
                                let path = format!("{}.png", path);
                                image::save_buffer_with_format(path, &data, meta.width.into(), meta.height.into(), image::ExtendedColorType::Rgba8, image::ImageFormat::Png).unwrap();
                            }
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