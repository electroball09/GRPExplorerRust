use std::io::Write;
use std::{io::Read, path::PathBuf};
use std::fs::File;
use crate::{egui as egui, ui::tools::{ExplorerToolId, Tool}};
use crate::util::twofish::*;
use rfd::FileDialog;

pub const INI_ENCRYPTION_KEY: &str = "506d6bdd571aa0f90256c91fbabc32ac";
pub const OASIS_ENCRYPTION_KEY: &str = "570462DC49E9E51F0B55F30287A5C7CD";

pub struct IniEditor {
    id: ExplorerToolId,
    close_requested: bool,
    status: Option<String>,

    open_file_path: Option<PathBuf>,
    backup_file_path: Option<PathBuf>,
    is_file_encrypted: bool,
    is_file_oasis: bool,

    base_string: String,
    output_string: String,
    changes_dirty: bool,

}

impl Tool for IniEditor {
    fn create(id: u32) -> Self{
        Self::new(id)
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        self.draw(ui, ctx)
    }
}

fn is_encrypted_file_path(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        return ext == "enc";
    }
    false
}

fn is_oasis_file_path(path: &PathBuf) -> bool {
    if let Some(name) = path.file_name() {
        return name.to_string_lossy().contains("Oasis");
    }
    false
}

fn load_file(path: &PathBuf, encryption_key: Option<&[u8]>) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(&path)?;
    
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    
    if let Some(key_bytes) = encryption_key {
        let iv = &key_bytes[0..16];
        twofish_decrypt_cfb1(&mut buf, key_bytes, iv);
    }

    Ok(buf)
}

fn get_encryption_key(path: &PathBuf) -> Option<&[u8]> {
    if is_encrypted_file_path(path) {
        if is_oasis_file_path(path) {
            Some(OASIS_ENCRYPTION_KEY.as_bytes())
        } else {
            Some(INI_ENCRYPTION_KEY.as_bytes())
        }
    } else { None }
}

impl IniEditor {
    fn new(id: u32) -> Self {
        Self {
            id: ExplorerToolId::new("yeti_ini_editor", id),
            close_requested: false,
            open_file_path: None,
            backup_file_path: None,
            base_string: "".into(),
            output_string: "".into(),
            changes_dirty: false,
            status: None,
            is_file_encrypted: false,
            is_file_oasis: false,
        }
    }

    fn load_new_file(&mut self, path: PathBuf) {
        let encryption_key = get_encryption_key(&path);

        match load_file(&path, encryption_key).map_err(|error| error.to_string())
            .and_then(|buf| String::from_utf8(buf).map_err(|error| error.to_string())) {
                Ok(data) => {
                    self.backup_file_path = {
                        let mut backup_path = path.clone();
                        backup_path.add_extension("bak");
                        Some(backup_path)
                    };

                    self.is_file_encrypted = is_encrypted_file_path(&path);
                    self.is_file_oasis = is_oasis_file_path(&path);
                    self.status = None;
                    self.base_string = data;
                    self.output_string = self.base_string.clone();
                    self.open_file_path = Some(path);
                },
                Err(error) => {
                    self.status = Some(error);
                    self.is_file_encrypted = false;
                    self.is_file_oasis = false;
                    self.base_string = "".into();
                    self.output_string = "".into();
                    self.open_file_path = None;
                }
            }

        self.changes_dirty = false;
    }

    fn save_file(&mut self) -> Result<(), String> {
        let Some(path) = &self.open_file_path else { return Ok(()); };

        let mut buf = self.output_string.as_bytes().to_vec();

        if self.is_file_encrypted {
            let key_bytes = if self.is_file_oasis {
                OASIS_ENCRYPTION_KEY.as_bytes()
            } else {
                INI_ENCRYPTION_KEY.as_bytes()
            };
            let iv = &key_bytes[0..16];
            twofish_encrypt_cfb1(&mut buf, key_bytes, iv);
        }

        File::create(&path).and_then(|mut file| {
            file.write_all(&buf)
        }).and_then(|()| {
            self.base_string = self.output_string.clone();
            Ok(())
        }).map_err(|error| error.to_string())
    }

    fn load_from_backup(&mut self) {
        let Some(path) = &self.open_file_path else { return; };
        let Some(backup) = &self.backup_file_path else { return; };

        let encryption_key = get_encryption_key(&path);

        match load_file(&backup, encryption_key).map_err(|error| error.to_string())
            .and_then(|buf| String::from_utf8(buf).map_err(|error| error.to_string())) {
                Ok(data) => {
                    self.status = None;
                    self.output_string = data;

                    self.changes_dirty = self.output_string != self.base_string;
                },
                Err(error) => {
                    self.status = Some(error);
                }
            }
    }

    fn save_backup_file(&self) -> Result<(), String> {
        let Some(path) = &self.open_file_path else { return Ok(()); };
        let Some(backup) = &self.backup_file_path else { return Ok(()); };

        std::fs::copy(path, backup).map(|_| ()).map_err(|error| error.to_string())
    }

    fn save_decrypted_file(&self) -> Result<(), String> {
        let Some(path) = &self.open_file_path else { return Ok(()); };

        let decrypted_path = path.with_extension("");
        match File::create(decrypted_path) {
            Ok(mut file) => file.write_all(self.base_string.as_bytes()).map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string())
        }
    }

    fn draw(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of(&self.id),
            egui::ViewportBuilder::default()
            .with_title("INI Editor")
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
                        if ui.button("Open File...").clicked() {
                            if let Some(file_path) = FileDialog::new()
                                .add_filter("ini", &["enc", "ini"])
                                .pick_file() {
                                    self.load_new_file(file_path);
                                }
                        }
                        if let Some(file) = &self.open_file_path {
                            ui.label(file.to_str().unwrap());
                        }
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(_) = &mut self.open_file_path {
                        if let Some(status) = &self.status {
                            ui.label(status);
                        }

                        ui.horizontal(|ui| {
                            let mut do_load_from_backup = false;

                            if let Some(backup_path) = &self.backup_file_path {
                                if backup_path.exists() {
                                    if ui.button("LOAD BACKUP").clicked() {
                                        do_load_from_backup = true;
                                    }
                                    ui.label(format!("{}", backup_path.display()));
                                } else {
                                    if ui.button("CREATE BACKUP").clicked() {
                                        if let Err(error) = self.save_backup_file() {
                                            self.status = Some(error);
                                        }
                                    }
                                    ui.label(format!("{}", backup_path.display()));
                                }
                            }

                            if do_load_from_backup {
                                self.load_from_backup();
                            }
                        });

                        if self.is_file_encrypted {
                            ui.horizontal(|ui| {
                                if let Some(path) = &self.open_file_path {
                                    let decrypted_path = path.with_extension("");
                                    if ui.button("SAVE DECRYPTED FILE").clicked() {
                                        if let Err(error) = self.save_decrypted_file() {
                                            self.status = Some(error);
                                        }
                                    }
                                    ui.label(format!("{}", decrypted_path.display()));
                                }
                            });
                        }

                        if self.changes_dirty {
                            ui.horizontal(|ui| {
                                ui.label("***UNSAVED CHANGES***");
                                ui.add_space(20.0);
                                if ui.button("SAVE").clicked() {
                                    if let Err(error) = self.save_file() {
                                        self.status = Some(error);
                                    } else {
                                        self.changes_dirty = false;
                                    }
                                }
                                ui.add_space(20.0);
                                if ui.button("DISCARD").clicked() {
                                    self.output_string = self.base_string.clone();
                                    self.changes_dirty = false;
                                }
                            });
                        } else {
                            ui.label("UP TO DATE");
                        }
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if ui.add(egui::TextEdit::multiline(&mut self.output_string).desired_width(f32::INFINITY)).changed() {
                                self.changes_dirty = self.output_string != self.base_string;
                            }
                        });
                    };
                });
            }
        );

        self.close_requested
    }
}