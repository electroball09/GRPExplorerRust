use super::*;
use crate::egui::Ui;
use crate::metadata::YKey;
use log::*;

use crate::bigfile::Bigfile;
use crate::ui::*;

pub struct FileTreeView {
    debug_folders: bool,
    debug_files: bool,
    clicked_file: Option<YKey>
}

impl FileTreeView {
    pub fn new() -> Self {
        FileTreeView {
            debug_folders: false,
            debug_files: false,
            clicked_file: None
        }
    }

    pub fn did_click_file(&self) -> Option<YKey> {
        self.clicked_file
    }

    fn draw_file_tree(&mut self, ui: &mut Ui, app: &mut AppContext, debug_folders: bool, debug_files: bool) {
        fn draw_folder2(idx: &u16, bf: &Bigfile, ctx: &egui::Context, ui: &mut Ui, debug_folders: bool, debug_files: bool) -> Option<YKey> {
            if !bf.folder_table.contains_key(&idx) { return None; }
            let folder = bf.folder_table[&idx];
            let rsp = ui.collapsing(folder.get_name(), |ui| {
                let mut child = folder.first_child;
                let mut opt: Option<YKey> = None;
                while child != 0xFFFF {
                    if let Some(key) = draw_folder2(&child, bf, ctx, ui, debug_folders, debug_files) {
                       opt = Some(key);
                    }
                    child = match bf.folder_table.get(&child) {
                        Some(fld) => fld.next_folder,
                        None => {
                            warn!("could not find folder with id {}", &child);
                            0xFFFF
                        }
                    };
                };
                if let Some(v) = bf.file_list_map.get(&idx) {
                    for key in v.iter() {
                        let file = &bf.file_table[key];
                        ui.horizontal(|ui| {
                            ui.label(format!("{:?} -", file.object_type));
                            let btn = ui.button(file.get_name());
                            if btn.clicked() {
                                debug!("clicked file {}", file.get_name_ext());
                                opt = Some(*key);
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

        if let Some(ref mut bf) = app.bigfile {
            //folder id 0x0002 is the Data/ folder
            self.clicked_file = draw_folder2(&2, &bf, app.ctx, ui, debug_folders, debug_files);
        }
    }
}

impl View for FileTreeView {
    fn draw(&mut self, ui: &mut egui::Ui, mut app: AppContext) {
        egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui|{
            self.draw_file_tree(ui, &mut app, self.debug_folders, self.debug_files);
        });
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, _app: &mut AppContext) {
        ui.menu_button("File Tree", |ui| {
            ui.checkbox(&mut self.debug_folders, "Folder Info");
            ui.checkbox(&mut self.debug_files, "File Info");
        });
    }
}