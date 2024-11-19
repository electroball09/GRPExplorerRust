use crate::ui::*;

pub unsafe fn explorer_app_start() {
    info!("initing eframe...");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((1500.0, 800.0))
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "GRP Explorer", 
        native_options, 
        Box::new(|cc| {
            let mut app = ExplorerApp::init(&cc.egui_ctx);
            if let Some(gl) = &cc.gl {
                app.shader_cache.init(gl.clone());
            }
            Ok(Box::<ExplorerApp>::new(app))
        })
    ).unwrap();
}

impl eframe::App for ExplorerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx);
    }
}