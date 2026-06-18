use std::fmt::Display;

use eframe::egui;

pub fn format_bytes_to_hex(bytes: &[u8]) -> String {bytes.chunks(8)
        .map(|chunk| {
            chunk.iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\r\n")
}

pub trait AppUiUtil {
    fn enum_selector<T: strum::IntoEnumIterator + Clone + Display + PartialEq>(&mut self, value: &mut T);
    fn number_field(&mut self, num: &mut f32, storage_string: &mut String);
    fn page_selector(&mut self, page: &mut usize, total_pages: usize);
}

impl AppUiUtil for eframe::egui::Ui {
    fn enum_selector<T: strum::IntoEnumIterator + Clone + Display + PartialEq>(&mut self, value: &mut T) {
        for e in T::iter() {
            self.radio_value(value, e.clone(), e.to_string());
        }
    }
    
    fn number_field(&mut self, num: &mut f32, storage_string: &mut String) {
        let rsp = self.text_edit_singleline(storage_string);
        if rsp.lost_focus() {
            if let Ok(n) = storage_string.parse::<f32>() {
                *num = n;
            }
            *storage_string = format!("{}", num);
        }
    }
    
    fn page_selector(&mut self, page: &mut usize, total_pages: usize) {
        self.horizontal(|ui| {
            if ui.add_enabled(*page > 0, egui::Button::new(" |< ")).clicked() {
                *page = 0;
            }
            if ui.add_enabled(*page > 0, egui::Button::new(" < ")).clicked() {
                *page = *page - 1;
            }
            ui.add_enabled_ui(total_pages > 1, |ui| {
                let old_page = *page;
                let mut pg_str = format!("{}", old_page + 1);
                if egui::TextEdit::singleline(&mut pg_str).desired_width(40.0).show(ui).response.changed() {
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
                ui.label(format!("/{}", total_pages));
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