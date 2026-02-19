use egui::Widget;

use super::*;
use crate::objects::{ObjectArchetype, TextureFormat, TextureMetaType};
use crate::export::*;
use crate::util::texture_util;

pub struct TextureMetadataEditor {
    texture: Option<egui::TextureHandle>,
    tex_data: Option<Vec<u8>>,
    tex_size: egui::Vec2,
    texture_combinations: Option<TextureCombinations>,
    tex_view_mode: [bool; 4],
    zoom: f32,
}

impl Default for TextureMetadataEditor {
    fn default() -> Self {
        Self { 
            texture: None,
            tex_data: None,
            tex_size: [100.0, 100.0].into(),
            texture_combinations: None,
            tex_view_mode: [true, true, true, false],
            zoom: 1.0,
        }
    }
}

struct ImageCombinations {
    pub rgb: egui::ColorImage,
    pub rba: egui::ColorImage,
    pub rga: egui::ColorImage,
    pub gba: egui::ColorImage,
    pub rg : egui::ColorImage,
    pub rb : egui::ColorImage,
    pub ra : egui::ColorImage,
    pub gb : egui::ColorImage,
    pub ga : egui::ColorImage,
    pub ba : egui::ColorImage,
    pub r  : egui::ColorImage,
    pub g  : egui::ColorImage,
    pub b  : egui::ColorImage,
    pub a  : egui::ColorImage,
}

struct TextureCombinations {
    pub rgb: egui::TextureHandle,
    pub rba: egui::TextureHandle,
    pub rga: egui::TextureHandle,
    pub gba: egui::TextureHandle,
    pub rg : egui::TextureHandle,
    pub rb : egui::TextureHandle,
    pub ra : egui::TextureHandle,
    pub gb : egui::TextureHandle,
    pub ga : egui::TextureHandle,
    pub ba : egui::TextureHandle,
    pub r  : egui::TextureHandle,
    pub g  : egui::TextureHandle,
    pub b  : egui::TextureHandle,
    pub a  : egui::TextureHandle,
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
        Ok(egui::ColorImage::new(size, pixels))
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
        Ok(egui::ColorImage::new(size, pixels))
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
        Ok(egui::ColorImage::new(size, pixels))
    }

    fn image_to_buf(img: &egui::ColorImage) -> Vec<u8> {
        img.pixels.iter().flat_map(|img| [img.r(), img.g(), img.b(), img.a()]).collect()
    }

    fn get_displayed_texture_handle(&self) -> &egui::TextureHandle {
        match self.tex_view_mode {
            [true, true, true, true] =>     self.texture.as_ref().unwrap(),
            [true, true, true, false] =>    &self.texture_combinations.as_ref().unwrap().rgb,
            [true, true, false, true] =>    &self.texture_combinations.as_ref().unwrap().rba,
            [true, false, true, true] =>    &self.texture_combinations.as_ref().unwrap().rga,
            [false, true, true, true] =>    &self.texture_combinations.as_ref().unwrap().gba,
            [true, true, false, false] =>   &self.texture_combinations.as_ref().unwrap().rg ,
            [true, false, true, false] =>   &self.texture_combinations.as_ref().unwrap().rb ,
            [true, false, false, true] =>   &self.texture_combinations.as_ref().unwrap().ra ,
            [false, true, true, false] =>   &self.texture_combinations.as_ref().unwrap().gb ,
            [false, true, false, true] =>   &self.texture_combinations.as_ref().unwrap().ga ,
            [false, false, true, true] =>   &self.texture_combinations.as_ref().unwrap().ba ,
            [true, false, false, false] =>  &self.texture_combinations.as_ref().unwrap().r  ,
            [false, true, false, false] =>  &self.texture_combinations.as_ref().unwrap().g  ,
            [false, false, true, false] =>  &self.texture_combinations.as_ref().unwrap().b  ,
            [false, false, false, true] =>  &self.texture_combinations.as_ref().unwrap().a  ,
            [false, false, false, false] => self.texture.as_ref().unwrap(),
        }
    }

    fn make_image_combinations(src: &egui::ColorImage) -> ImageCombinations {
        ImageCombinations {
            rgb: egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), c32.g(), c32.b(), 0)).collect()),
            rba: egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), 0, c32.b(), c32.a())).collect()),
            rga: egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), c32.g(), 0, c32.a())).collect()),
            gba: egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(0, c32.g(), c32.b(), c32.a())).collect()),
            rg : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), c32.g(), 0, 0)).collect()),
            rb : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), 0, c32.b(), 0)).collect()),
            ra : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), 0, 0, c32.a())).collect()),
            gb : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(0, c32.g(), c32.b(), 0)).collect()),
            ga : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(0, c32.g(), 0, c32.a())).collect()),
            ba : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(0, 0, c32.b(), c32.a())).collect()),
            r  : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.r(), 0, 0, 0)).collect()),
            g  : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(0, c32.g(), 0, 0)).collect()),
            b  : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(0, 0, c32.b(), 0)).collect()),
            a  : egui::ColorImage::new(src.size, src.pixels.iter().map(|c32| egui::Color32::from_rgba_premultiplied(c32.a(), c32.a(), c32.a(), 0)).collect()),
        }
    }

    fn make_texture_combinations(name: &str, ctx: &egui::Context, combs: ImageCombinations) -> TextureCombinations {
        TextureCombinations {
            rgb: ctx.load_texture(format!("{}-rgb", name), combs.rgb, Default::default()),
            rba: ctx.load_texture(format!("{}-rba", name), combs.rba, Default::default()),
            rga: ctx.load_texture(format!("{}-rga", name), combs.rga, Default::default()),
            gba: ctx.load_texture(format!("{}-gba", name), combs.gba, Default::default()),
            rg : ctx.load_texture(format!("{}-rg ", name), combs.rg , Default::default()),
            rb : ctx.load_texture(format!("{}-rb ", name), combs.rb , Default::default()),
            ra : ctx.load_texture(format!("{}-ra ", name), combs.ra , Default::default()),
            gb : ctx.load_texture(format!("{}-gb ", name), combs.gb , Default::default()),
            ga : ctx.load_texture(format!("{}-ga ", name), combs.ga , Default::default()),
            ba : ctx.load_texture(format!("{}-ba ", name), combs.ba , Default::default()),
            r  : ctx.load_texture(format!("{}-r  ", name), combs.r  , Default::default()),
            g  : ctx.load_texture(format!("{}-g  ", name), combs.g  , Default::default()),
            b  : ctx.load_texture(format!("{}-b  ", name), combs.b  , Default::default()),
            a  : ctx.load_texture(format!("{}-a  ", name), combs.a  , Default::default()),
        }
    }
}

impl EditorImpl for TextureMetadataEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
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
                            let name = format!("{:#010X}", key);
                            self.tex_data = Some(TextureMetadataEditor::image_to_buf(&image));
                            let icombs = TextureMetadataEditor::make_image_combinations(&image);
                            self.texture_combinations = Some(TextureMetadataEditor::make_texture_combinations(&name, ectx.ctx, icombs)); //this is really inefficient but idk another way
                            let tex = ectx.ctx.load_texture(name, image, Default::default());
                            self.texture = Some(tex);
                            self.tex_size = [meta.width as f32, meta.height as f32].into();
                        }
                    }
                }
                
                egui::SidePanel::left("tex_panel").resizable(false).exact_width(200.0).show_inside(ui, |ui| {
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

                    ui.separator();
                    ui.checkbox(&mut self.tex_view_mode[0], "r");
                    ui.checkbox(&mut self.tex_view_mode[1], "g");
                    ui.checkbox(&mut self.tex_view_mode[2], "b");
                    ui.checkbox(&mut self.tex_view_mode[3], "a");
                });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if let Some(_) = &self.texture {
                        ui.centered_and_justified(|ui| {
                            let rsp = egui::Widget::ui(egui::Image::new(self.get_displayed_texture_handle()).fit_to_exact_size(self.tex_size * self.zoom), ui);
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
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
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