use std::ops::DerefMut;
use std::{ops::Deref};
use egui::Ui;

use crate::bigfile::{Bigfile, obj_type_to_name};
use crate::bigfile::metadata::FileEntry;
use crate::ui::*;
use crate::ui::editors::draw_editor_for_type;

pub trait View {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub struct FileTreeView {
    bigfile: BfRef,
    debug_folders: bool,
    debug_files: bool,
    clicked_file: Option<u32>
}

impl FileTreeView {
    pub fn new(bigfile: BfRef) -> Self {
        FileTreeView {
            bigfile,
            debug_folders: false,
            debug_files: false,
            clicked_file: None
        }
    }

    pub fn did_click_file(&self) -> Option<u32> {
        self.clicked_file
    }

    fn draw_file_tree(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context, debug_folders: bool, debug_files: bool) {
        fn draw_folder2(idx: &u16, bf: &Bigfile, ctx: &eframe::egui::Context, ui: &mut Ui, debug_folders: bool, debug_files: bool) -> Option<u32> {
            let folder = bf.folder_table[&idx];
            let rsp = ui.collapsing(folder.get_name(), |ui| {
                let mut child = folder.first_child;
                while child != 0xFFFF {
                    if let Some(key) = draw_folder2(&child, bf, ctx, ui, debug_folders, debug_files) {
                        return Some(key);
                    }
                    child = bf.folder_table[&child].next_folder;
                };
                let mut opt: Option<u32> = None;
                if let Some(v) = bf.file_list_map.get(&idx) {
                    for key in v.iter() {
                        let file = &bf.file_table[key];
                        ui.horizontal(|ui| {
                            ui.label(format!("{:?} -", file.object_type));
                            let btn = ui.button(file.get_name());
                            if btn.clicked() {
                                println!("clicked file {}", file.get_name_ext());
                                opt = Some(key.clone());
                            }
                            if debug_files {
                                btn.on_hover_ui_at_pointer(|ui| {
                                    ui.label(format!("{}\nkey: {:#010X}\noffset: {}\nflags: {:#010X}\nzip: {}", 
                                                            file.get_name(), file.key, file.offset, file.flags, file.zip));
                                });
                            };
                        });
                    };
                }
                opt
            });
            if debug_folders {
                rsp.header_response.on_hover_ui_at_pointer(|ui| {
                    ui.label(format!("idx: {:#06X}\nparent_folder: {:#06X}\nfirst_child: {:#06X}\nnext_folder: {:#06X}\nunk01: {:#06X}\nunk02: {:#06X}\nunk03: {:#06X}\nunk04: {:#06X}", 
                                            folder.idx, folder.parent_folder, folder.first_child, folder.next_folder, folder.unk01, folder.unk02, folder.unk03, folder.unk04));
                });
            };
            rsp.body_returned?
        }

        if let Some(bf) = self.bigfile.clone() {
            let r = bf.as_ref().borrow();
            let bf = r.deref();
            self.clicked_file = draw_folder2(&0, &bf, ctx, ui, debug_folders, debug_files);
        }
    }
}

impl View for FileTreeView {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.toggle_value(&mut self.debug_folders, "Folder Info");
            ui.toggle_value(&mut self.debug_files, "File Info");
        });

        ui.separator();
        
        egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui|{
            self.draw_file_tree(ui, ctx, self.debug_folders, self.debug_files);
        });
    }
}

pub struct FileEditorTabs {
    bigfile: BfRef,
    editor_tabs: Vec<u32>,
    open_tab: Option<u32>
}

impl FileEditorTabs {
    pub fn new(bigfile: BfRef) -> Self {
        FileEditorTabs {
            bigfile,
            editor_tabs: Vec::new(),
            open_tab: None
        }
    }
}

impl FileEditorTabs {
    pub fn open_new_tab(&mut self, key: u32) {
        if !self.editor_tabs.contains(&key) {
            self.editor_tabs.push(key);
            if let Err(error) = self.bigfile.as_ref().unwrap().as_ref().borrow_mut().load_file(key) {
                println!("{}", error);
            }
        }
       self.set_open_tab(key);
    }

    pub fn set_open_tab(&mut self, key: u32) {
        if self.editor_tabs.contains(&key) {
            self.open_tab = Some(key);
        } else {
            println!("wtf couldn't find key for tab!");
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

    fn draw_file_metadata_view(file: &FileEntry, ui: &mut Ui, ctx: &eframe::egui::Context) {
        fn file_metadata_line(ui: &mut Ui, label: &str, value: &str) -> bool {
            let mut clicked = false;
            ui.horizontal(|ui| {
                ui.label(label);
                clicked = ui.selectable_label(false, value).clicked();
            });
            clicked
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
    fn draw(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context) {
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
                                egui::Frame::default().fill(egui::Color32::from_rgb(255, 255, 255)).show(ui, |ui| {
                                    ui.label(bf.file_table[key].get_name_ext())
                                }).response
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
                egui::SidePanel::left("file_entry_panel").default_width(200.0)
                    .resizable(false).show(ctx, |ui| {
                        FileEditorTabs::draw_file_metadata_view(&bf.file_table[&key], ui, ctx);

                        ui.horizontal(|ui| {
                            if ui.button("Extract...").clicked()  {
                                let path = rfd::FileDialog::new()
                                    .pick_folder()
                                    .unwrap();

                                let mut path = String::from(path.to_str().unwrap());
                                path += &String::from(format!("/{:#010X} {}", key, String::from(bf.file_table[&key].get_name_ext())));

                                bf.extract_file_to_path(&path, key).unwrap();
                            }
                        });

                        ui.add_space(15.0);

                        ui.label(format!("references: {}", bf.object_table[&key].references.len()));
                        egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui| {
                            for key in bf.object_table[&key].references.iter() {
                                if bf.file_table.contains_key(&key) {
                                    if ui.selectable_label(false, format!("{:#010X}", key)).clicked() {
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
                    if let Some(v) = rsp.open_new_tab {
                        open_new_tab = Some(v);
                    }
                    if let Some(act) = rsp.perform_action {
                        act(&bf);
                    }
                }); 
            }
        }
        if let Some(v) = open_new_tab {
            for key in &v {
                self.open_new_tab(*key);
            }
        }
    }
}