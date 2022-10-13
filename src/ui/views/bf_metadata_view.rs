use super::super::BfRef;

pub struct BigfileMetadataView {
    bigfile: BfRef,
}

impl BigfileMetadataView {
    pub fn new(bf: BfRef) -> Self {
        Self {
            bigfile: bf,
        }
    }
}

impl super::View for BigfileMetadataView {
    fn draw(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        let bf = self.bigfile.clone().unwrap();
        let bf = bf.as_ref().borrow();

        ui.label("segment header");
        ui.indent("seg_header", |ui| {
            ui.label(format!("sig: {}", bf.segment_header.sig_to_str()));
            ui.label(format!("segment: {}", bf.segment_header.segment));
            ui.label(format!("num segments: {}", bf.segment_header.num_segments));
            ui.label(format!("data offset: {}", bf.segment_header.header_offset));
        });

        ui.add_space(5.0);

        ui.label("bigfile header");
        ui.indent("bf_header", |ui| {
            ui.label(format!("version: {:#04X}", bf.bigfile_header.version));
            ui.label(format!("num folders: {}", bf.bigfile_header.num_folders));
            ui.label(format!("num files: {}", bf.bigfile_header.num_files));
            ui.label(format!("load priority: {}", bf.bigfile_header.load_priority));
            ui.label(format!("auto activate: {}", bf.bigfile_header.auto_activate));
            ui.label(format!("data root: {}", bf.bigfile_header.data_root_str()));
        });
    }

    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf;
    }

    fn settings_menu(&mut self, _ui: &mut egui::Ui, _ctx: &egui::Context) {
        
    }
}