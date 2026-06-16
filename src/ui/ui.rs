use eframe::egui::{self, Ui};



pub struct ExplorerUi;

impl ExplorerUi {
    pub fn page_selector(ui: &mut Ui, page: &mut usize, total_pages: usize) {
        ui.horizontal(|ui| {
            if ui.add_enabled(*page > 0, egui::Button::new(" |< ")).clicked() {
                *page = 0;
            }
            if ui.add_enabled(*page > 0, egui::Button::new(" < ")).clicked() {
                *page = *page - 1;
            }
            ui.add_enabled_ui(total_pages > 1, |ui| {
                let old_page = *page;
                let mut pg_str = format!("{}", old_page + 1);
                if egui::TextEdit::singleline(&mut pg_str).desired_width(20.0).show(ui).response.changed() {
                    if let Ok(val) = usize::from_str_radix(&pg_str, 10) {
                        if val <= total_pages {
                            *page = val - 1;
                        } else {
                            *page = old_page;
                        }
                    } else {
                        *page = old_page;
                    }
                }
            });
            if ui.add_enabled(total_pages > 1 && *page < total_pages - 1, egui::Button::new(" > ")).clicked() {
                *page = *page + 1;
            }
            if ui.add_enabled(total_pages > 1 && *page < total_pages - 1 , egui::Button::new(" >| ")).clicked() {
                *page = total_pages - 1;
            }
        });
    }
}