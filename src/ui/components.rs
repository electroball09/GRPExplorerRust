use egui::Ui;

use crate::bigfile::{*, io::BigfileIO};

pub fn draw_file_tree<T: BigfileIO>(bf: &Bigfile<T>, ui: &mut Ui, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    let draw_folder = |ui: &mut Ui| {
        
    };
}