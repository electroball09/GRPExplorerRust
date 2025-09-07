use std::fmt::Display;

pub trait AppUiUtil {
    fn enum_selector<T: strum::IntoEnumIterator + Clone + Display + PartialEq>(&mut self, value: &mut T);
    fn number_field(&mut self, num: &mut f32, storage_string: &mut String);
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
}