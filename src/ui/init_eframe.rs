use crate::ui::*;

pub fn explorer_app_start() {
    eframe::run_native(
        "GRP Explorer", 
        eframe::NativeOptions::default(), 
        Box::new(|cc| Box::<ExplorerApp>::new(ExplorerApp::new(&cc.egui_ctx)))
    )
}

impl eframe::App for ExplorerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        frame.set_window_size(egui::Vec2::new(1500.0, 800.0));

        self.update(ctx);
    }
}