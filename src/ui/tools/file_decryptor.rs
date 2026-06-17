use std::io::Write;
use std::{io::Read, path::PathBuf};
use std::fs::File;
use crate::ui::{AppUiUtil};
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
    lines_per_page: u32,

    base_strings: Vec<String>,
    output_strings: Vec<String>,
    changes_dirtys: Vec<bool>,
    current_page: usize,

    //base_string: String,
    //output_string: String,
    //changes_dirty: bool,
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

fn paginate_string_into(string: &String, pages: &mut Vec<String>, lines_per_page: u32) {
    pages.clear();

    let mut counter = 0;

    let mut tmp_string = String::new();
    for line in string.lines() {
        tmp_string.push_str(line);
        tmp_string.push_str("\n");

        counter = counter + 1;

        if counter >= lines_per_page {
            counter = 0;
            pages.push(tmp_string);
            tmp_string = String::new();
        }
    }

    if tmp_string.len() > 0 {
        pages.push(tmp_string);
    }
}

fn depaginate_string_from(pages: &Vec<String>) -> String {
    let mut len = 0;
    for page in pages {
        len = len + page.len();
        if !page.ends_with("\n") {
            len = len + 1;
        }
    }

    let mut string = String::with_capacity(len);
    for page in pages {
        string.push_str(page);
        if !page.ends_with("\n") {
            string.push_str("\n");
        }
    }

    string
}

enum EditorCommand {
    None,
    LoadBackup,
    SaveBackup,
    LoadDecrypted,
    SaveDecrypted,
}

impl IniEditor {
    fn new(id: u32) -> Self {
        Self {
            id: ExplorerToolId::new("yeti_ini_editor", id),
            close_requested: false,
            open_file_path: None,
            backup_file_path: None,
            base_strings: Vec::new(),
            output_strings: Vec::new(),
            changes_dirtys: Vec::new(),
            //base_string: "".into(),
            //output_string: "".into(),
            //changes_dirty: false,
            status: None,
            is_file_encrypted: false,
            is_file_oasis: false,
            lines_per_page: 50,
            current_page: 0,
        }
    }

    fn check_all_changes_dirty(&mut self) {
        for i in 0..self.changes_dirtys.len() {
            let base = &self.base_strings[i];
            let output = &self.output_strings[i];
            
            let value = base != output;
            self.changes_dirtys[i] = value;
        }
    }

    fn are_changes_dirty(&self) -> bool {
        if self.base_strings.len() != self.output_strings.len() {
            return true;
        }

        self.changes_dirtys.iter().any(|b| *b)
    }

    fn get_min_pages(&self) -> usize {
        if self.base_strings.len() < self.output_strings.len() {
            return self.base_strings.len();
        }

        self.output_strings.len()
    }

    fn reset_changes_dirty(&mut self) {
        if self.changes_dirtys.len() != self.get_min_pages() {
            self.changes_dirtys.resize(self.get_min_pages(), false);
        }
        for b in &mut self.changes_dirtys {
            *b = false;
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
                    paginate_string_into(&data, &mut self.base_strings, self.lines_per_page);
                    self.output_strings = self.base_strings.clone();
                    self.reset_changes_dirty();
                    self.open_file_path = Some(path);
                },
                Err(error) => {
                    self.status = Some(error);
                    self.is_file_encrypted = false;
                    self.is_file_oasis = false;
                    self.base_strings.clear();
                    self.output_strings.clear();
                    self.reset_changes_dirty();
                    self.open_file_path = None;
                }
            }
    }

    fn save_file(&mut self) -> Result<(), String> {
        let Some(path) = &self.open_file_path else { return Ok(()); };

        let mut buf = depaginate_string_from(&self.output_strings).as_bytes().to_vec();

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
            self.base_strings = self.output_strings.clone();
            self.reset_changes_dirty();
            Ok(())
        }).map_err(|error| error.to_string())
    }

    fn save_backup_file(&self) -> Result<(), String> {
        let Some(path) = &self.open_file_path else { return Ok(()); };
        let Some(backup) = &self.backup_file_path else { return Ok(()); };

        std::fs::copy(path, backup).map(|_| ()).map_err(|error| error.to_string())
    }

    fn load_backup_file(&mut self) -> Result<(), String> {
        let Some(path) = &self.open_file_path else { return Ok(()); };
        let Some(backup) = &self.backup_file_path else { return Ok(()); };

        let encryption_key = get_encryption_key(&path);

        load_file(&backup, encryption_key).map_err(|error| error.to_string())
            .and_then(|buf| String::from_utf8(buf).map_err(|error| error.to_string()))
            .and_then(|data| {
                paginate_string_into(&data, &mut self.output_strings, self.lines_per_page);
                self.reset_changes_dirty();
                Ok(())
            })?;

        self.check_all_changes_dirty();
        Ok(())
    }

    fn save_decrypted_file(&self) -> Result<(), String> {
        if !self.is_file_encrypted { return Ok(()); }
        let Some(path) = &self.open_file_path else { return Ok(()); };

        let decrypted_path = path.with_extension("");
        match File::create(decrypted_path) {
            Ok(mut file) => {
                let string = depaginate_string_from(&self.base_strings);
                file.write_all(string.as_bytes()).map_err(|error| error.to_string())
            },
            Err(error) => Err(error.to_string())
        }
    }

    fn load_decrypted_file(&mut self) -> Result<(), String> {
        if !self.is_file_encrypted { return Ok(()); };
        let Some(path) = &self.open_file_path else { return Ok(()); };
        
        let decrypted_path = path.with_extension("");

        let data = load_file(&decrypted_path, None).map_err(|error| error.to_string())
            .and_then(|buf| String::from_utf8(buf).map_err(|error| error.to_string()))?;

        paginate_string_into(&data, &mut self.output_strings, self.lines_per_page);
        self.reset_changes_dirty();

        self.check_all_changes_dirty();

        Ok(())
    }

    fn draw(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of(&self.id),
            egui::ViewportBuilder::default()
            .with_title("File Decryptor")
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
                                .add_filter("ini", &["enc", "ini", "inf"])
                                .pick_file() {
                                    self.load_new_file(file_path);
                                }
                        }
                        if let Some(file) = &self.open_file_path {
                            ui.label(file.to_str().unwrap());
                        }
                    });
                });

                let mut command = EditorCommand::None;

                egui::CentralPanel::default().show(ctx, |ui| {
                    let Some(path) = &self.open_file_path else { return; };

                    if let Some(status) = &self.status {
                        ui.label(status);
                    }

                    if let Some(backup_path) = &self.backup_file_path {
                        ui.horizontal(|ui| {
                            
                                if backup_path.exists() {
                                    if ui.button("LOAD BACKUP").clicked() {
                                        command = EditorCommand::LoadBackup;
                                    }
                                    ui.label(format!("{}", backup_path.display()));
                                } else {
                                    if ui.button("CREATE BACKUP").clicked() {
                                        command = EditorCommand::SaveBackup;
                                    }
                                    ui.label(format!("{}", backup_path.display()));
                                }
                            });
                    };

                    if self.is_file_encrypted {
                        ui.add_space(10.0);

                        let decrypted_path = path.with_extension("");
                        ui.horizontal(|ui| {
                            if ui.button("SAVE DECRYPTED FILE").clicked() {
                                command = EditorCommand::SaveDecrypted;
                            }
                            if decrypted_path.exists() {
                                if ui.button("LOAD DECRYPTED FILE").clicked() {
                                    command = EditorCommand::LoadDecrypted;
                                }
                            }
                        });
                        ui.label(format!("{}", decrypted_path.display()));
                    }

                    ui.add_space(10.0);
                    if self.are_changes_dirty() {
                        ui.horizontal(|ui| {
                            ui.label("***UNSAVED CHANGES***");
                            ui.add_space(10.0);
                            if ui.button("SAVE").clicked() {
                                if let Err(error) = self.save_file() {
                                    self.status = Some(error);
                                } else {
                                    self.reset_changes_dirty();
                                }
                            }
                            ui.add_space(20.0);
                            if ui.button("DISCARD").clicked() {
                                self.output_strings = self.base_strings.clone();
                                self.reset_changes_dirty();
                            }
                        });
                    } else {
                        ui.label("UP TO DATE");
                    }

                    ui.page_selector(&mut self.current_page, self.base_strings.len());
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let output = &mut self.output_strings[self.current_page];
                        let changed = &mut self.changes_dirtys[self.current_page];
                        if ui.add(egui::TextEdit::multiline(output).desired_width(f32::INFINITY)).changed() {
                            *changed = true;
                        }
                    });
                });

                match command {
                    EditorCommand::LoadBackup => {
                        if let Err(error) = self.load_backup_file() {
                            self.status = Some(error);
                        }
                    },
                    EditorCommand::SaveBackup => {
                        if let Err(error) = self.save_backup_file() {
                            self.status = Some(error);
                        }
                    },
                    EditorCommand::LoadDecrypted => {
                        if let Err(error) = self.load_decrypted_file() {
                            self.status = Some(error);
                        }
                    },
                    EditorCommand::SaveDecrypted => {
                        if let Err(error) = self.save_decrypted_file() {
                            self.status = Some(error);
                        }
                    },
                    _ => { }
                };
            }
        );

        self.close_requested
    }
}