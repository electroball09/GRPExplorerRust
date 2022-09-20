use eframe::AppCreator;
use crate::ui;

pub mod components;

#[derive(Debug, Default)]
pub struct ExplorerApp {

}

impl ExplorerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn make_creator() -> AppCreator {
        Box::new(|cc| Box::new(ui::ExplorerApp::new(cc)))
    }
}

impl eframe::App for ExplorerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("omg what?");
            // components::draw_file_tree(ui, ctx, frame);
        });
    }
}