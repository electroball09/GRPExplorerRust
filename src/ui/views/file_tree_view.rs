use super::*;
use std::ops::Deref;
use egui::Ui;

use crate::bigfile::Bigfile;
use crate::ui::*;

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
            if !bf.folder_table.contains_key(&idx) { return None; }
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
    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf.clone();
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui|{
            self.draw_file_tree(ui, ctx, self.debug_folders, self.debug_files);
        });
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.menu_button("File Tree", |ui| {
            ui.checkbox(&mut self.debug_folders, "Folder Info");
            ui.checkbox(&mut self.debug_files, "File Info");
        });
    }
}