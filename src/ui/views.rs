use std::{rc::Rc, cell::RefCell, ops::Deref, borrow::Borrow};
use egui::Ui;
use id_tree::NodeId;

use crate::bigfile::Bigfile;
use crate::bigfile::metadata::FileEntry;
use crate::ui::*;
use crate::ui::editors::get_editor_for_type;

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
            debug_files: true,
            clicked_file: None
        }
    }

    pub fn did_click_file(&self) -> Option<u32> {
        self.clicked_file
    }

    fn draw_file_tree(&mut self, ui: &mut Ui, ctx: &eframe::egui::Context, debug_folders: bool, debug_files: bool) {
        fn draw_folder(bf: &Bigfile, node_id: &NodeId, ctx: &eframe::egui::Context, ui: &mut Ui, debug_folders: bool, debug_files: bool) -> Option<u32> {
            let folder = bf.node_id_to_folder(node_id);
            let rsp = ui.collapsing(folder.get_name(), |ui| {
                for node_id in bf.tree.get(node_id).unwrap().children().iter() {
                    if let Some(key) = draw_folder(bf, node_id, ctx, ui, debug_folders, debug_files) {
                        return Some(key);
                    }
                }
                let mut opt: Option<u32> = None;
                if let Some(v) = bf.file_list_map.get(&bf.node_id_to_folder(node_id).idx) {
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
                    ui.label(format!("idx: {:#06X}\nparent_folder: {:#06X}\nfirst_child: {:#06X}\nunk03: {:#06X}\nunk05: {:#06X}", 
                                            folder.idx, folder.parent_folder, folder.first_child, folder.unk03, folder.unk05));
                });
            };
            if let Some(inner) = rsp.body_returned {
                return inner;
            };
            None
        }

        if let Some(bf) = self.bigfile.clone() {
            let r = bf.as_ref().borrow();
            let bf = r.deref();
            self.clicked_file = draw_folder(&bf, &bf.node_id_map[&0].0, ctx, ui, debug_folders, debug_files);
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
        }
       self.set_open_tab(key);
    }

    pub fn set_open_tab(&mut self, key: u32) {
        if let Some(idx) = self.editor_tabs.iter().position(|k| *k == key) {
            let new_key = self.editor_tabs.remove(idx as usize);
            self.editor_tabs.insert(0, new_key);
            self.open_tab = Some(new_key);
        } else {
            println!("wtf couldn't find key for tab!");
        }
    }

    pub fn close_tab(&mut self, key: u32) {
        if let Some(idx) = self.editor_tabs.iter().position(|k| *k == key) {
            self.editor_tabs.remove(idx);
            if self.editor_tabs.len() > 0 {
                self.set_open_tab(self.editor_tabs[0]);
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

        egui::SidePanel::left("file_entry_panel").default_width(200.0)
            .resizable(false).show(ctx, |ui| {
                file_metadata_line(ui, "      key:", &format!("{:#010X}", file.key));
                file_metadata_line(ui, "   offset:", &format!("{:#010X}", file.offset));
                file_metadata_line(ui, "    unk01:", &format!("{:#010X}", file.unk01));
                file_metadata_line(ui, "     type:", &format!("{:?}", file.object_type));
                file_metadata_line(ui, "   folder:", &format!("{:#06X}", file.parent_folder));
                file_metadata_line(ui, "timestamp:", &format!("{}", file.timestamp));
                file_metadata_line(ui, "    flags:", &format!("{:#010X}", file.flags));
                file_metadata_line(ui, "    unk02:", &format!("{:#010X}", file.unk02));
                file_metadata_line(ui, "    unk03:", &format!("{:#010X}", file.unk03));
                file_metadata_line(ui, "      zip:", &format!("{}", file.zip));
        });
    }
}

impl View for FileEditorTabs {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(bf) = self.bigfile.clone() {
            let r = bf.as_ref().borrow();
            let bf = r.deref();
            egui::TopBottomPanel::top("file_editor_tabs").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut new_open_tab: Option<u32> = None;
                    let mut close_tab: Option<u32> = None;
                    for key in self.editor_tabs.iter() {
                        match self.open_tab {
                            Some(key2) => {
                                if key.clone() == key2 { 
                                    ui.label(bf.file_table[key].get_name_ext());
                                } else {
                                    let rsp = ui.selectable_label(false, bf.file_table[key].get_name_ext());
                                    if rsp.clicked() {
                                        new_open_tab = Some(*key);
                                    }
                                    if rsp.middle_clicked() {
                                        close_tab = Some(*key);
                                    }
                                }
                                if ui.selectable_label(false, "x").clicked() {
                                    if let Some(idx) = self.editor_tabs.iter().position(|k| *k == *key) {
                                        self.editor_tabs.remove(idx);
                                        if self.editor_tabs.len() > 0 {
                                            self.set_open_tab(self.editor_tabs[0]);
                                        } else {
                                            self.open_tab = None;
                                        }
                                    }
                                    return;
                                }
                            },
                            None => { }
                        }
                        ui.separator();
                    }
                    if let Some(key) = new_open_tab {
                        self.set_open_tab(key);
                    }
                    if let Some(key) = close_tab {
                        self.close_tab(key);
                    }
                });
            });
            if let Some(key) = self.open_tab {
                FileEditorTabs::draw_file_metadata_view(&bf.file_table[&key], ui, ctx)
            }
        }
    }
}