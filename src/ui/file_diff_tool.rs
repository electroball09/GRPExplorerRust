use std::collections::HashMap;

use crate::egui as egui;

pub struct FileDiffTool {
    files: Vec<(String, String)>,
    datas: HashMap<usize, Vec<u8>>,
    diff: Vec<Option<u8>>,
    close_requested: bool,
}

impl FileDiffTool {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            diff: Vec::new(),
            datas: HashMap::new(),
            close_requested: false,
        }
    }

    fn do_diff(&mut self) {
        let mut datas = HashMap::new();
        let mut diff = Vec::new();

        for (idx, (_, path)) in self.files.iter().enumerate() {
            if let Ok(mut data) = std::fs::read(&path) {
                datas.insert(idx, data.clone());
                
                if diff.len() == 0 {
                    diff = data.iter().map(|b| Some(*b)).collect();
                } else {
                    let len = data.len().min(diff.len());
                    for (i, b) in data.drain(0..len).enumerate() {
                        if let Some(a) = diff[i] {
                            if a == b {
                                diff[i] = Some(a);
                            } else {
                                diff[i] = None;
                            }
                        }
                    }
                    for b in data.iter() {
                        diff.push(Some(*b));
                    }
                }
            }
        }
        
        self.diff = diff;
        self.datas = datas;
    }

    pub fn draw(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("test!"), 
            egui::ViewportBuilder::default()
            .with_title("test!")
            .with_always_on_top()
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_position([500.0, 200.0])
            .with_drag_and_drop(true)
            .with_inner_size([800.0, 900.0])
            .with_min_inner_size([400.0, 100.0]), 
            |ctx, _class| {
                ctx.input(|state| {
                    if state.viewport().close_requested() {
                        self.close_requested = true;
                    }
                });                

                egui::SidePanel::left("filediffside").exact_width(400.0).show(ctx, |ui| {
                    ctx.input(|state| {
                        if !state.raw.dropped_files.is_empty() {
                            for file in &state.raw.dropped_files {
                                if let Some(path) = &file.path {
                                    let name = path.as_path().file_name().unwrap().to_string_lossy().into_owned();
                                    self.files.push((name, path.to_string_lossy().into_owned()));
                                }
                            }
                        }
                    });

                    if ui.button("Calculate!").clicked() {
                        self.do_diff();
                    }

                    ui.separator();

                    {
                        let mut rm = None;
                        for (idx, (name, _path)) in self.files.iter().enumerate() {
                            let rsp = ui.label(name);
                            if rsp.clicked() {
    
                            }
                            if rsp.middle_clicked() {
                                rm = Some(idx);
                            }
                        }
                        if let Some(idx) = rm {
                            self.files.remove(idx);
                        }
                    }
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.diff.len() > 0 {
                        let str = self.diff.chunks(8).map(|chunk| {
                            chunk.iter().map(|v| {
                                if let Some(b) = v {
                                    format!("{:02X}", *b)
                                } else {
                                    "XX".into()
                                }
                            }).collect::<Vec<String>>().join(" ")
                        }).collect::<Vec<String>>().join("\r\n");
                        ui.label(str);
                    };
                });
            }
        );

        self.close_requested
    }
}