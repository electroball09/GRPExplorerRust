use std::cmp::Ordering;
use strum::{EnumString, AsRefStr};
use crate::{bigfile::Bigfile, ui::AppContext};
use crate::egui as egui;

pub struct SearchView {
    query: String,
    query_changed: bool,
    match_case: bool,
    results: Vec<u32>,
    clicked_file: Option<u32>,
    page: usize,
    files_per_page: usize,
    sort: SortBy,
}

#[derive(Debug, Clone, Copy, EnumString, PartialEq, Eq, AsRefStr)]
enum SortBy {
    Name,
    Type,
    Key,
    Offset
}

impl SearchView {
    pub fn new() -> SearchView {
        Self {
            query: String::new(),
            query_changed: false,
            match_case: false,
            results: Vec::new(),
            clicked_file: None,
            page: 0,
            files_per_page: 1000,
            sort: SortBy::Name,
        }
    }

    pub fn did_click_file(&self) -> Option<u32> {
        self.clicked_file
    }

    fn sort_results(&mut self, bf: &Bigfile) {
        self.results.sort_by(|a, b| {
            let a = &bf.file_table[a];
            let b = &bf.file_table[b];
            match &self.sort {
                SortBy::Name => a.get_name_ext().cmp(b.get_name_ext()),
                SortBy::Type => a.object_type.as_ref().cmp(b.object_type.as_ref()),
                SortBy::Key => a.key.cmp(&b.key),
                SortBy::Offset => a.offset.cmp(&b.offset)
            }
        });
    }
}

impl super::View for SearchView {
    fn draw(&mut self, ui: &mut egui::Ui, app: &mut AppContext) {
        if let Some(ref mut bf) = app.bigfile {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    let edit_rsp = ui.text_edit_singleline(&mut self.query);
                    if edit_rsp.changed() {
                        self.query_changed = true;
                    }
    
                    let search_rsp = ui.button("Search").on_hover_text(
                        "Search will match on the following:\n\
                        \tKey, Offset, Type, Name\n\
                        \n\
                        -File key/offset will match on values parseable as decimal or hexadecimal\n\
                        -File type extension will match on values 3 characters long\n\
                        \tExtension also matches if value starts with '.' (e.g. \".wor\")\n\
                        -File name will match on ascii name without extension\n\
                        \tTo match a value would qualify above, use double quotes (e.g. \"F2000\")"
                    );
    
                    if (search_rsp.clicked() || edit_rsp.lost_focus())
                             && self.query_changed {
                        self.page = 0;
                        self.query_changed = false;
    
                        if self.query.is_empty() {
                            self.results.clear();
                            return;
                        }
    
                        self.results = bf.file_table.iter()
                            .filter_map(|entry| {
                                let some = Some(*entry.0);
    
                                if self.query.starts_with('.') {
                                    if entry.1.object_type.as_ref().cmp(&self.query[1..]) == Ordering::Equal {
                                        return some;
                                    }
                                    None
                                } else if let Ok(val) = u32::from_str_radix(&self.query.trim_start_matches("0x"), 16) {
                                    if entry.1.offset == val || entry.1.key == val {
                                        return some;
                                    }
                                    None
                                } else if let Ok(val) = u32::from_str_radix(&self.query, 10) {
                                    if entry.1.offset == val || entry.1.key == val {
                                        return some;
                                    }
                                    None
                                } else {
                                    if self.query.len() <= 6 {
                                        if matches!(entry.1.object_type.as_ref().cmp(&self.query), Ordering::Equal) {
                                            return some;
                                        }
                                    }
    
                                    if self.match_case {
                                        if entry.1.get_name().contains(&self.query.trim_start_matches("\"").trim_end_matches("\"")) {
                                            return some;
                                        }
                                    } else {
                                        if entry.1.get_name().to_ascii_lowercase().contains(&self.query.trim_start_matches("\"").trim_end_matches("\"").to_ascii_lowercase()) {
                                            return some;
                                        }
                                    }
        
                                    None
                                }
                            })
                            .collect();
    
                        self.results.dedup();
    
                        self.sort_results(bf);
                    };
                });
            });
    
            ui.horizontal(|ui| {
                ui.label(format!("results: {}", self.results.len()));
                ui.separator();
    
                ui.label("Sort:");
                let old_sort = self.sort.clone();
                egui::ComboBox::from_id_salt("bf_search_sort").selected_text(self.sort.as_ref()).show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sort, SortBy::Name, "Name");
                    ui.selectable_value(&mut self.sort, SortBy::Type, "Type");
                    ui.selectable_value(&mut self.sort, SortBy::Key, "Key");
                    ui.selectable_value(&mut self.sort, SortBy::Offset, "Offset");
                    false
                });
                if old_sort != self.sort {
                    self.sort_results(bf);
                }
                ui.separator();
    
    
    
                if ui.checkbox(&mut self.match_case, "Match Case").changed() {
                    self.query_changed = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Test load results").clicked() {
                    for key in self.results.iter() {
                        if let Err(error) = bf.load_file(*key) {
                            log::error!("{:#010X} - {}", key, error);
                        }
                        let _ = bf.unload_file(*key);
                    }
                }
            });
            ui.separator();
    
            ui.horizontal(|ui| {
                let total_pages = (self.results.len() / self.files_per_page) + 1;
                if ui.add_enabled(self.page > 0, egui::Button::new(" |< ")).clicked() {
                    self.page = 0;
                }
                if ui.add_enabled(self.page > 0, egui::Button::new(" < ")).clicked() {
                    self.page = self.page - 1;
                }
                ui.add_enabled_ui(total_pages > 1, |ui| {
                    let old_page = self.page;
                    let mut pg_str = format!("{}", old_page + 1);
                    if egui::TextEdit::singleline(&mut pg_str).desired_width(20.0).show(ui).response.changed() {
                        if let Ok(val) = usize::from_str_radix(&pg_str, 10) {
                            if val <= total_pages {
                                self.page = val - 1;
                            } else {
                                self.page = old_page;
                            }
                        } else {
                            self.page = old_page;
                        }
                    }
                });
                if ui.add_enabled(total_pages > 1 && self.page < total_pages - 1, egui::Button::new(" > ")).clicked() {
                    self.page = self.page + 1;
                }
                if ui.add_enabled(total_pages > 1 && self.page < total_pages - 1 , egui::Button::new(" >| ")).clicked() {
                    self.page = total_pages - 1;
                }
            });
    
            if !self.results.is_empty() {
                ui.add_space(2.0);
    
                ui.push_id(self.page, |ui| {
                    egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui,|ui| {
                        self.clicked_file = None;
        
                        let first_ind = self.page * self.files_per_page;
                        let last_ind = usize::min(self.results.len(), (self.page + 1) * self.files_per_page);
                        let slice = &self.results[first_ind..last_ind];
        
                        for key in slice {
                            let file = bf.file_table[key];
                            ui.horizontal(|ui| {
                                ui.label(format!(".{:?} - {:#010X}", file.object_type, file.key));
                                if ui.button(format!("{}", file.get_name())).clicked() {
                                    self.clicked_file = Some(*key);
                                }
                            });
                        }
                    });
                });
            }
        }
    }
}