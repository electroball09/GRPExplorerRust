use std::io::Write;
use std::{io::Read, path::PathBuf};
use std::fs::File;
use crate::{egui as egui, ui::tools::{ExplorerToolId, Tool}};
use crate::util::twofish::*;
use rfd::FileDialog;

pub struct IniEditor {
    id: ExplorerToolId,
    close_requested: bool,
    open_file_path: Option<PathBuf>,
    base_ini_string: String,
    output_ini_string: String,
    changes_dirty: bool,
    error: Option<String>,
}

impl Tool for IniEditor {
    fn create(id: u32) -> Self{
        Self::new(id)
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        self.draw(ui, ctx)
    }
}

impl IniEditor {
    fn new(id: u32) -> Self {
        Self {
            id: ExplorerToolId::new("yeti_ini_editor", id),
            close_requested: false,
            open_file_path: None,
            base_ini_string: "".into(),
            output_ini_string: "".into(),
            changes_dirty: false,
            error: None,
        }
    }

    fn load_new_file(&mut self, path: PathBuf) {
        let mut file = File::open(&path).unwrap();

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        if path.extension().is_some_and(|ext| ext == "enc") {
            let key_bytes = if path.file_name().is_some_and(|name| name.to_string_lossy().contains("Oasis")) {
                "570462DC49E9E51F0B55F30287A5C7CD".as_bytes()
            } else {
                "506d6bdd571aa0f90256c91fbabc32ac".as_bytes()
            };
            let iv = &key_bytes[0..16];
            buf = twofish_decrypt_cfb1(&buf, key_bytes, iv);
        }
        
        match String::from_utf8(buf) {
            Ok(data) => {
                self.error = None;
                self.base_ini_string = data;
                self.output_ini_string = self.base_ini_string.clone();
                self.open_file_path = Some(path);
            },
            Err(error) => {
                self.error = Some(error.to_string());
                self.base_ini_string = "".into();
                self.output_ini_string = "".into();
                self.open_file_path = None;
            }
        }
        self.changes_dirty = false;
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.open_file_path {
            let mut buf = self.output_ini_string.as_bytes().to_vec();

            if path.extension().is_some_and(|ext| ext == "enc") {
                let key_bytes = if path.file_name().is_some_and(|name| name.to_string_lossy().contains("Oasis")) {
                    "570462DC49E9E51F0B55F30287A5C7CD".as_bytes()
                } else {
                    "506d6bdd571aa0f90256c91fbabc32ac".as_bytes()
                };
                let iv = &key_bytes[0..16];
                buf = twofish_encrypt_cfb1(&buf, key_bytes, iv);
            }

            if let Ok(mut file) = File::create(&path) {
                file.write_all(&buf).unwrap();
            }

            self.base_ini_string = self.output_ini_string.clone();
        }
    }

    fn draw(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of(&self.id),
            egui::ViewportBuilder::default()
            .with_title("INI Editor")
            .with_always_on_top()
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_position([500.0, 200.0])
            .with_inner_size([1000.0, 600.0])
            .with_min_inner_size([400.0, 100.0]), 
            |ctx, _class| {
                ctx.input(|state| {
                    if state.viewport().close_requested() {
                        self.close_requested = true;
                    }
                });

                egui::TopBottomPanel::top(&self.id).show(ctx, |ui| {
                    egui::MenuBar::new().ui(ui, |ui| {
                        ui.label("File:");
                        if let Some(file) = &self.open_file_path {
                            ui.label(file.to_str().unwrap());
                        }
                        if ui.button("...").clicked() {
                            if let Some(file_path) = FileDialog::new()
                                .add_filter("ini", &["enc", "ini"])
                                .pick_file() {
                                    self.load_new_file(file_path);
                                }
                        }
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(_) = &mut self.open_file_path {
                        if let Some(error) = &self.error {
                            ui.label(error);
                        }

                        if self.changes_dirty {
                            ui.horizontal(|ui| {
                                ui.label("***UNSAVED CHANGES***");
                                ui.add_space(20.0);
                                if ui.button("SAVE").clicked() {
                                    self.save_file();
                                }
                                ui.add_space(20.0);
                                if ui.button("DISCARD").clicked() {
                                    self.output_ini_string = self.base_ini_string.clone();
                                }
                            });
                        } else {
                            ui.label("UP TO DATE");
                        }
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if ui.add(egui::TextEdit::multiline(&mut self.output_ini_string).desired_width(f32::INFINITY)).changed() {

                            }

                            self.changes_dirty = self.output_ini_string != self.base_ini_string;
                        });
                    };
                });
            }
        );

        self.close_requested
    }
}