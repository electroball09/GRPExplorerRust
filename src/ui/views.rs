use std::{rc::Rc, cell::RefCell, ops::Deref};
use crate::bigfile::Bigfile;
use super::components;

pub trait View {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {

    }
}

pub struct FileTreeView {
    pub bigfile: Option<Rc<RefCell<Bigfile>>>,
    debug_folders: bool,
    debug_files: bool,
}

impl FileTreeView {
    pub fn new(bigfile: Option<Rc<RefCell<Bigfile>>>) -> Self {
        FileTreeView {
            bigfile: None,
            debug_folders: true,
            debug_files: true
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

        if let Some(bf) = self.bigfile.clone() {
            components::draw_file_tree(bf.as_ref().borrow().deref(), ui, ctx, self.debug_folders, self.debug_files);
        };
    }
}