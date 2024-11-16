use super::*;
use log::*;
use std::ops::DerefMut;
use crate::egui::Ui;
use std::time::Instant;
use clipboard::*;
use crate::bigfile::{Bigfile, obj_type_to_name};
use crate::bigfile::metadata::FileEntry;
use crate::ui::*;
use crate::ui::editors::{draw_editor_for_type, EditorResponse};

pub struct FileEditorTabs {
    bigfile: BfRef,
    editor_tabs: Vec<u32>,
    open_tab: Option<u32>,
    last_update: Instant,
    num_frames: u32,
}

impl FileEditorTabs {
    pub fn new(bigfile: BfRef) -> Self {
        FileEditorTabs {
            bigfile,
            editor_tabs: Vec::new(),
            open_tab: None,
            last_update: Instant::now(),
            num_frames: 0
        }
    }
}

impl FileEditorTabs {
    pub fn open_new_tab(&mut self, key: u32) {
        if !self.editor_tabs.contains(&key) {
            self.editor_tabs.push(key);
            if let Err(error) = self.bigfile.as_ref().unwrap().as_ref().borrow_mut().load_file(key) {
                error!("{}", error);
            }
        }
       self.set_open_tab(key);
    }

    pub fn set_open_tab(&mut self, key: u32) {
        if self.editor_tabs.contains(&key) {
            self.open_tab = Some(key);
        } else {
            error!("wtf couldn't find key for tab!");
        }
    }

    pub fn close_tab(&mut self, key: u32, bf: &mut Bigfile) {
        if let Some(idx) = self.editor_tabs.iter().position(|k| *k == key) {
            self.editor_tabs.remove(idx);

            bf.unload_file(key).expect("failed to unload file");

            let mut idx = idx as i32;
            if key == self.open_tab.unwrap() {
                idx = (idx).min(self.editor_tabs.len() as i32 - 1);
            } else {
                idx = self.editor_tabs.iter().position(|k| self.open_tab.unwrap() == *k).unwrap() as i32;
            }
            if idx >= 0 {
                self.set_open_tab(self.editor_tabs[idx as usize]);
            } else {
                self.open_tab = None;
            }
        }
    }

    fn draw_file_metadata_view(file: &FileEntry, ui: &mut Ui, _ctx: &egui::Context) {
        fn file_metadata_line(ui: &mut Ui, label: &str, value: &str) -> bool {
            ui.horizontal(|ui| {
                ui.label(label);
                let rsp = ui.selectable_label(false, value).on_hover_text("Click to copy");
                if rsp.clicked() {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(String::from(value)).unwrap();
                    return true;
                }
                false
            }).inner
        }

        let tmp_type_name = &format!("{:?}", file.object_type);
        let obj_type_name: &str = match obj_type_to_name(&file.object_type) {
            Some(string) => string,
            None => &tmp_type_name
        };

        file_metadata_line(ui, "   key:", &format!("{:#010X}", file.key));
        file_metadata_line(ui, "offset:", &format!("{:#010X}", file.offset));
        file_metadata_line(ui, " unk01:", &format!("{:#010X}", file.unk01));
        file_metadata_line(ui, "  type:", &format!("{}", obj_type_name));
        file_metadata_line(ui, "folder:", &format!("{:#06X}", file.parent_folder));
        file_metadata_line(ui, "  time:", &format!("{}", file.timestamp));
        file_metadata_line(ui, " flags:", &format!("{:#010X}", file.flags));
        file_metadata_line(ui, " unk02:", &format!("{:#010X}", file.unk02));
        file_metadata_line(ui, " unk03:", &format!("{:#010X}", file.unk03));
        file_metadata_line(ui, "   zip:", &format!("{}", file.zip));
    }
}

impl View for FileEditorTabs {
    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf;
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut open_new_tab: Option<Vec<u32>> = None;
        if let Some(bf) = self.bigfile.clone() {
            let mut r = bf.as_ref().borrow_mut();
            let mut bf = r.deref_mut();
            egui::TopBottomPanel::top("file_editor_tabs").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut new_open_tab: Option<u32> = None;
                    let mut close_tab: Option<u32> = None;
                    for key in self.editor_tabs.iter() {
                        let open_tab_key = self.open_tab.expect("we have tabs, but none are open???");
                        let rsp = match *key == open_tab_key {
                            true => {
                                ui.selectable_label(true, bf.file_table[key].get_name_ext())
                            },
                            false => {
                                ui.selectable_label(false, bf.file_table[key].get_name_ext())
                            }
                        };
                        if rsp.clicked() {
                            new_open_tab = Some(*key);
                        }
                        if rsp.middle_clicked() {
                            close_tab = Some(*key);
                        }
                        if ui.selectable_label(false, "x").clicked() {
                            close_tab = Some(*key);
                        }
                        ui.separator();
                    }
                    if let Some(key) = new_open_tab {
                        self.set_open_tab(key);
                    }
                    if let Some(key) = close_tab {
                        self.close_tab(key, &mut bf);
                    }
                });
                if let Some(key) = self.open_tab {
                    ui.separator();
                    let mut dir = bf.get_full_directory(bf.file_table[&key].parent_folder);
                    dir += bf.file_table[&key].get_name_ext();
                    ui.label(dir);
                }
            });
            if let Some(key) = self.open_tab {
                ui.push_id(key, |_ui| {
                    egui::SidePanel::left("file_entry_panel").default_width(200.0).max_width(600.0).min_width(200.0)
                        .resizable(true).show(ctx, |ui| {
                            FileEditorTabs::draw_file_metadata_view(&bf.file_table[&key], ui, ctx);
    
                            ui.horizontal(|ui| {
                                if ui.button("Extract...").clicked()  {
                                    if let Some(path) = crate::export::pick_extract_folder() {
                                        let mut path = String::from(path.to_str().unwrap());
                                        path += &String::from(format!("/{:#010X} {}", key, String::from(bf.file_table[&key].get_name_ext())));
        
                                        bf.extract_file_to_path(&path, key).unwrap();
                                    }
                                }
                            });
    
                            ui.add_space(15.0);
    
                            ui.label(format!("references: {}", bf.object_table[&key].references.len()));
                            egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui| {
                                for key in bf.object_table[&key].references.iter() {
                                    if bf.file_table.contains_key(&key) {
                                        let rsp = ui.selectable_label(false, format!("{:#010X} {}", key, bf.file_table[&key].get_name_ext()));
                                        if rsp.clicked() {
                                            open_new_tab = Some(vec![*key]);
                                        }
                                    } else {
                                        ui.label(format!("{:#010X}", key));
                                    }
                                }
                            })
                        });
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let mut y = bf.object_table.get_mut(&key).unwrap();
                        let rsp = draw_editor_for_type(&bf.file_table[&key].object_type, &mut y, ui, ctx);
                        if let EditorResponse::OpenNewTabs(v) = &rsp {
                            open_new_tab = Some(v.clone());
                        } 
                        if let EditorResponse::PerformAction(key, act) = rsp {
                            act(key, bf);
                        }
                    }); 
                });
            }
        }
        if let Some(v) = open_new_tab {
            for key in &v {
                self.open_new_tab(*key);
            }
        }
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        let now = Instant::now();
        ui.menu_button("Stats", |ui| {
            ui.label(format!("ft: {} ms", (now - self.last_update).as_secs_f32() * 1000.0));
            ui.label(format!("fr: {}", self.num_frames));
        });
        self.last_update = now;
        self.num_frames += 1;
    }
}