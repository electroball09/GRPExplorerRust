use {super::ExplorerApp, egui_miniquad as egui_mq, miniquad as mq};

pub fn explorer_app_start() {
    let conf = mq::conf::Conf {
        high_dpi: true,
        window_width: 1500,
        window_height: 800,
        window_resizable: false,
        window_title: "GRP Explorer".to_string(),
        ..Default::default()
    };
    mq::start(conf, || Box::new(Stage::new()));
}

struct Stage {
    egui_mq: egui_mq::EguiMq,
    mq_ctx: Box<dyn mq::RenderingBackend>,
    app: ExplorerApp
}

impl Stage {
    fn new() -> Self {
        let mut mq_ctx = mq::window::new_rendering_backend();
        let egui_mq = egui_mq::EguiMq::new(&mut *mq_ctx);
        let app = ExplorerApp::new(egui_mq.egui_ctx());

        Self {
            egui_mq,
            mq_ctx,
            app
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        self.mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        self.mq_ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.mq_ctx.end_render_pass();

        let dpi_scale = mq::window::dpi_scale();

        self.egui_mq.run(&mut *self.mq_ctx, |_mq_ctx, egui_ctx| {
            egui_ctx.set_zoom_factor(1.25);

            self.app.update(egui_ctx);
        });

        self.egui_mq.draw(&mut *self.mq_ctx);

        self.mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx / 20.0, dy / 20.0);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}