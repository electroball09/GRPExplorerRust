use super::*;
use editors::{create_editor_for_type, EditorContext, EditorImpl};
use egui::Widget;
use log::*;
use crate::egui::Ui;
use crate::export::GltfExportWindow;
use crate::loader::{AmortizedLoad, LoadSet};
use crate::metadata::YKey;
use clipboard::*;
use crate::bigfile::{Bigfile, obj_type_to_name};
use crate::bigfile::metadata::FileEntry;
use crate::ui::*;
use crate::ui::editors::EditorResponse;

struct EditorTab {
    key: YKey,
    name: String,
    editor: Box<dyn EditorImpl>,
    load: AmortizedLoad,
    loaded: bool,
}

pub struct EditorTabContext<'a> {
    pub load_set: &'a dyn LoadSet,
}

pub struct FileEditorTabs {
    editor_tabs: Vec<EditorTab>,
    open_tab: Option<YKey>,
    loads_per_update: u32,
    open_exports: Vec<GltfExportWindow>,
}

impl FileEditorTabs {
    pub fn new() -> Self {
        FileEditorTabs {
            editor_tabs: Vec::new(),
            open_tab: None,
            loads_per_update: 700,
            open_exports: Vec::new(),
        }
    }
}

impl FileEditorTabs {
    fn find_tab(&self, key: YKey) -> Option<usize> {
        self.editor_tabs.iter().position(|k| k.key == key)
    }

    pub fn open_new_tab(&mut self, bf: &mut Bigfile, key: YKey) {
        if let None = self.find_tab(key) {
            let editor = create_editor_for_type(&bf.file_table[&key].object_type);
            self.editor_tabs.push(EditorTab {
                key,
                name: bf.file_table[&key].get_name_ext().to_string(),
                editor,
                load: AmortizedLoad::new(key),
                loaded: false,
            });
            if let Err(error) = bf.load_file(key) {
                error!("{}", error);
            }
        }
       self.set_open_tab(key);
    }

    pub fn set_open_tab(&mut self, key: YKey) {
        if let Some(_) = self.find_tab(key) {
            self.open_tab = Some(key);
        } else {
            error!("wtf couldn't find key for tab!");
        }
    }

    pub fn close_tab(&mut self, key: YKey, bf: &mut Bigfile) {
        if let Some(idx) = self.find_tab(key) {
            self.editor_tabs.get_mut(idx).unwrap().load.unload_all(bf);
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

    fn draw_top_panel(&mut self, _ui: &mut egui::Ui, ectx: &mut EditorContext<'_>) {
        egui::TopBottomPanel::top("file_editor_tabs").show(ectx.ctx, |ui| {
            let mut new_open_tab: Option<YKey> = None;
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
                    let mut dir = ectx.bf.get_full_directory(ectx.bf.file_table[&key].parent_folder);
                    dir += ectx.bf.file_table[&key].get_name_ext();
                    ui.label(dir);
                }
            });
        });
    }

    fn draw_central_panel(&mut self, key: YKey, _ui: &mut egui::Ui, ectx: &mut EditorContext<'_>) {
        egui::CentralPanel::default().show(ectx.ctx, |ui| {
            for tab in self.editor_tabs.iter_mut() {
                if tab.key == key && tab.loaded {
                    let tctx = EditorTabContext {
                        load_set: &tab.load
                    };
                    tab.editor.draw(key, ui, ectx, &tctx);
                }
            }
        });
    }

    fn draw_side_panel(&mut self, key: YKey, _ui: &mut egui::Ui, ectx: &mut EditorContext<'_>) {
        egui::SidePanel::left("file_entry_panel").default_width(200.0).max_width(600.0).min_width(200.0)
            .resizable(true).show(ectx.ctx, |ui| {

                FileEditorTabs::draw_file_metadata_view(&ectx.bf.file_table[&key], ui, ectx.ctx);

                match ui.horizontal_wrapped(|ui| {
                    if ui.button("Extract...").clicked()  {
                        if let Some(path) = crate::export::pick_extract_folder() {
                            let name = &self.editor_tabs[self.find_tab(key).unwrap()].name;
                            let path = format!("{}\\{:#010X} {}", path.to_str().unwrap(), key, name);
                            return EditorResponse::ExtractFile(key, path.to_string());
                        }
                    }

                    let tab_idx = self.find_tab(key).unwrap();
                    let tab: &mut EditorTab = self.editor_tabs.get_mut(tab_idx).unwrap();
                    ui.label(format!("load: {}", tab.load.get_load_status()));

                    EditorResponse::None
                }).inner {
                    EditorResponse::None => { },
                    rsp => ectx.respond(rsp)
                };

                ui.add_space(15.0);

                let bf: &Bigfile = ectx.bf;

                ui.label(format!("references: {}", bf.object_table[&key].references.len()));
                let rsp = egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui| {
                    for key in bf.object_table[&key].references.iter() {
                        if bf.file_table.contains_key(&key) {
                            let rsp = ui.selectable_label(false, format!("{:#010X} {}", key, bf.file_table[&key].get_name_ext()));
                            if rsp.clicked() {
                                return EditorResponse::OpenNewTab(*key);
                            }
                        } else {
                            ui.label(format!("{:#010X}", key));
                        }
                    }
                    EditorResponse::None
                }).inner;

                match rsp {
                    EditorResponse::None => { },
                    rsp => ectx.respond(rsp)
                }
            }
        );
    }

    fn process_loads(&mut self, bf: &mut Bigfile) -> bool {
        for tab in self.editor_tabs.iter_mut() {
            if !tab.loaded {
                if tab.load.load_num(bf, self.loads_per_update) {
                    tab.loaded = true;
                }
                return true;
            }
        }
        return false;
    }
}

impl View for FileEditorTabs {
    fn draw<'a>(&mut self, ui: &mut egui::Ui, mut app: AppContext) {
        if let Some(ref mut bf) = app.bigfile {
            if self.process_loads(bf) {
                app.ctx.request_repaint();
            }

            let do_closes = self.open_exports.iter_mut().map(|w| w.draw(app.ctx, bf)).collect::<Vec<bool>>();
            for (i, close) in do_closes.iter().rev().enumerate() {
                if *close {
                    self.open_exports.remove(i);
                }
            }
        } else {
            return;
        }

        let mut ectx = EditorContext::new(
            app.bigfile.unwrap(),
            app.shader_cache,
            app.ctx
        );

        self.draw_top_panel(ui, &mut ectx);

        if let Some(key) = self.open_tab {
            ui.push_id(key, |ui| {
                self.draw_side_panel(key, ui, &mut ectx);
                
                self.draw_central_panel(key, ui, &mut ectx);
            });
        }
        
        let mut responses = Vec::with_capacity(ectx.num_responses());
        for rsp in ectx.drain() {
            responses.push(rsp);
        }

        for rsp in responses {
            match rsp {
                EditorResponse::None => {
                    warn!("received None response from an editor!");
                }
                EditorResponse::OpenNewTab(key) => {
                    self.open_new_tab(ectx.bf, key);
                },
                EditorResponse::CloseTab(key) => {
                    self.close_tab(key, ectx.bf);
                },
                EditorResponse::ExtractFile(key, path) => {
                    ectx.bf.extract_file_to_path(&path, key).expect("could not extract file!");
                },
                EditorResponse::GltfExport(key) => {
                    //crate::export::gltf_export(key, ectx.bf);

                    let window = GltfExportWindow::new(key, ectx.bf.file_table[&key].get_name_ext());
                    self.open_exports.push(window);
                }
            }
        }
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, app: &mut AppContext) {
        if ui.button("Log loaded objects...").clicked() {
            if let Some(ref bf) = app.bigfile {
                bf.log_loaded_objects();
            }
        }
        
        ui.menu_button("Loading", |ui| {
            egui::Slider::new(&mut self.loads_per_update, 1..=1000).ui(ui);
        });
    }
}