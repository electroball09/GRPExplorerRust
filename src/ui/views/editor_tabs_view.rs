use std::collections::HashMap;

use super::*;
use editors::{create_editor_for_type, EditorContext, EditorImpl};
use log::*;
use crate::egui::Ui;
use crate::loader::BigfileLoad;
use crate::objects::YetiObject;
use clipboard::*;
use crate::bigfile::{Bigfile, obj_type_to_name};
use crate::bigfile::metadata::FileEntry;
use crate::ui::*;
use crate::ui::editors::EditorResponse;

struct EditorTab {
    key: u32,
    name: String,
    editor: Box<dyn EditorImpl>,
}

pub struct FileEditorTabs {
    editor_tabs: Vec<EditorTab>,
    open_tab: Option<u32>,
    loads: Vec<BigfileLoad>,
    load_statuses: HashMap<u32, String>,
}

impl FileEditorTabs {
    pub fn new() -> Self {
        FileEditorTabs {
            editor_tabs: Vec::new(),
            open_tab: None,
            loads: Vec::new(),
            load_statuses: HashMap::new(),
        }
    }
}

impl FileEditorTabs {
    fn find_tab(&self, key: u32) -> Option<usize> {
        self.editor_tabs.iter().position(|k| k.key == key)
    }

    pub fn open_new_tab(&mut self, bf: &mut Bigfile, key: u32) {
        if let None = self.find_tab(key) {
            let editor = create_editor_for_type(&bf.file_table[&key].object_type);
            self.editor_tabs.push(EditorTab {
                key,
                name: bf.file_table[&key].get_name_ext().to_string(),
                editor
            });
            if let Err(error) = bf.load_file(key) {
                error!("{}", error);
            }
        }
       self.set_open_tab(key);
    }

    pub fn set_open_tab(&mut self, key: u32) {
        if let Some(_) = self.find_tab(key) {
            self.open_tab = Some(key);
        } else {
            error!("wtf couldn't find key for tab!");
        }
    }

    pub fn close_tab(&mut self, key: u32, bf: &mut Bigfile) {
        if let Some(idx) = self.find_tab(key) {
            self.editor_tabs.remove(idx);

            bf.unload_file(key).expect("failed to unload file");

            let mut idx = idx as i32;
            if key == self.open_tab.unwrap() {
                idx = (idx).min(self.editor_tabs.len() as i32 - 1);
            } else {
                idx = self.editor_tabs.iter().position(|k| self.open_tab.unwrap() == k.key).unwrap() as i32;
            }
            if idx >= 0 {
                self.set_open_tab(self.editor_tabs[idx as usize].key);
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

    fn draw_top_panel(&mut self, bf: &Bigfile, _ui: &mut egui::Ui, ectx: &mut EditorContext<'_>) {
        egui::TopBottomPanel::top("file_editor_tabs").show(ectx.ctx, |ui| {
            let mut new_open_tab: Option<u32> = None;
            ui.horizontal_wrapped(|ui| {
                for tab in self.editor_tabs.iter() {
                    let open_tab_key = self.open_tab.expect("we have tabs, but none are open???");
                    let rsp = match tab.key == open_tab_key {
                        true => {
                            ui.selectable_label(true, &tab.name)
                        },
                        false => {
                            ui.selectable_label(false, &tab.name)
                        }
                    };
                    if rsp.clicked() {
                        new_open_tab = Some(tab.key);
                    }
                    if rsp.middle_clicked() || ui.selectable_label(false, "x").clicked() {
                        ectx.respond(EditorResponse::CloseTab(tab.key));
                    }
                    ui.separator();
                }
            });

            if let Some(key) = new_open_tab {
                self.set_open_tab(key);
            }

            ui.horizontal_wrapped(|ui| {
                if let Some(key) = self.open_tab {
                    ui.separator();
                    let mut dir = bf.get_full_directory(bf.file_table[&key].parent_folder);
                    dir += bf.file_table[&key].get_name_ext();
                    ui.label(dir);
                }
            });
        });
    }

    fn draw_central_panel(&mut self, yobj: &mut YetiObject, _ui: &mut egui::Ui, ectx: &mut EditorContext<'_>) {
        egui::CentralPanel::default().show(ectx.ctx, |ui| {
            for tab in self.editor_tabs.iter_mut() {
                if tab.key == yobj.get_key() {
                    tab.editor.draw(yobj, ui, ectx);
                }
            }
        }); 
    }

    fn draw_side_panel(&mut self, bf: &Bigfile, key: u32, _ui: &mut egui::Ui, ectx: &mut EditorContext<'_>) {
        egui::SidePanel::left("file_entry_panel").default_width(200.0).max_width(600.0).min_width(200.0)
            .resizable(true).show(ectx.ctx, |ui| {
                FileEditorTabs::draw_file_metadata_view(&bf.file_table[&key], ui, ectx.ctx);

                ui.horizontal_wrapped(|ui| {
                    if ui.button("Extract...").clicked()  {
                        if let Some(path) = crate::export::pick_extract_folder() {
                            let name = &self.editor_tabs[self.find_tab(key).unwrap()].name;
                            let path = format!("{}\\{:#010X} {}", path.to_str().unwrap(), key, name);
                            ectx.respond(EditorResponse::ExtractFile(key, path.to_string()))
                        }
                    }

                    if self.load_statuses.contains_key(&key) {
                        
                        ui.label(format!("load: {}", self.load_statuses.get(&key).unwrap()));
                    } else {
                        if ui.button("Load tree...").clicked() {
                            self.loads.push(BigfileLoad::new(key));
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
                                ectx.respond(EditorResponse::OpenNewTab(*key));
                            }
                        } else {
                            ui.label(format!("{:#010X}", key));
                        }
                    }
                });
            }
        );
    }

    fn process_loads(&mut self, bf: &mut Bigfile) -> bool {
        let mut to_remove = Vec::new();
        let mut idx = 0;
        for load in self.loads.iter_mut() {
            if load.load_num(bf, 100) {
                to_remove.push(idx);
            }
            self.load_statuses.insert(load.get_initial_key(), load.get_load_status());
            idx += 1;
        }

        to_remove.reverse();
        for idx in to_remove {
            self.loads.remove(idx);
        }

        self.loads.len() > 0
    }
}

impl View for FileEditorTabs {
    fn draw<'a>(&mut self, ui: &mut egui::Ui, app: &'a mut AppContext<'a>) {
        if let Some(ref mut bf) = app.bigfile {
            if self.process_loads(bf) {
                app.ctx.request_repaint();
            }
        }

        let mut ectx = EditorContext {
            shader_cache: app.shader_cache,
            ctx: app.ctx,
            responses: Vec::new()
        };

        if let Some(ref mut bf) = app.bigfile {
            self.draw_top_panel(bf, ui, &mut ectx);
    
            if let Some(key) = self.open_tab {
                ui.push_id(key, |ui| {
                    self.draw_side_panel(bf, key, ui, &mut ectx);
                    
                    self.draw_central_panel(bf.object_table.get_mut(&key).unwrap(), ui, &mut ectx);
                });
            }
            
            for rsp in ectx.drain() {
                match rsp {
                    EditorResponse::OpenNewTab(key) => {
                        self.open_new_tab(bf, key);
                    },
                    EditorResponse::CloseTab(key) => {
                        self.close_tab(key, bf);
                    },
                    EditorResponse::PerformAction(key, act) => {
                        act(key, bf);
                    },
                    EditorResponse::ExtractFile(key, path) => {
                        bf.extract_file_to_path(&path, key).expect("could not extract file!");
                    }
                }
            }
        }
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, _app: &mut AppContext) {
        
    }
}