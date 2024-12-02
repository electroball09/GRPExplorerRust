use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::egui as egui;
use crate::ggl::ShaderCache;
use font_loader::system_fonts;
use crate::bigfile::io::*;
use crate::FileDialog;
use crate::Bigfile;
use log::*;

#[cfg(feature = "eframe")]
pub mod init_eframe;
#[cfg(feature = "eframe")]
pub use init_eframe as explorer_init;

#[cfg(feature = "miniquad")]
pub mod init_mq;
#[cfg(feature = "miniquad")]
pub use init_mq as explorer_init;

use self::views::*;
use self::views::side_panel::SidePanelView;
use views::editor_tabs_view::FileEditorTabs;

pub mod views;
pub mod editors;
mod editor_context; pub use editor_context::*;

pub struct AppContext<'a> {
    pub bigfile: Option<&'a mut Bigfile>,
    pub shader_cache: &'a mut ShaderCache,
    pub ctx: &'a egui::Context,
}

pub struct ExplorerApp {
    pub bigfile: Option<Bigfile>,
    side_panel: views::side_panel::SidePanelView,
    fe_view: FileEditorTabs, 
    pub shader_cache: ShaderCache
}

impl Default for ExplorerApp {
    fn default() -> Self {
        Self {
            bigfile: None,
            side_panel: SidePanelView::new(),
            fe_view: FileEditorTabs::new(),
            shader_cache: ShaderCache::new()
        }
    }
}

impl ExplorerApp {
    pub fn init(ctx: &egui::Context) -> Self {
        let prop = system_fonts::FontPropertyBuilder::new().family("Cascadia Mono").build();
        let (font, _) = system_fonts::get(&prop).unwrap();

        let f = Cow::Owned(font);

        let f_data = egui::FontData {
            font: f,
            index: 0,
            tweak: egui::FontTweak::default()
        };

        let mut m = BTreeMap::new();
        m.insert(String::from("Cascadia Mono"), f_data);

        let mut m2 = BTreeMap::new();
        m2.insert(egui::FontFamily::Monospace, vec![String::from("Cascadia Mono")]);
        m2.insert(egui::FontFamily::Proportional, vec![String::from("Cascadia Mono")]);

        ctx.set_fonts(egui::FontDefinitions {
            font_data: m,
            families: m2
        });

        let style = ctx.style().as_ref().clone();
        let style = egui::Style {
            visuals: egui::Visuals {
                widgets: egui::style::Widgets {
                    noninteractive: egui::style::WidgetVisuals {
                        fg_stroke: egui::Stroke {
                            color: egui::Color32::from_rgb(170, 170, 170),
                            ..style.visuals.widgets.noninteractive.fg_stroke
                        },
                        ..style.visuals.widgets.noninteractive
                    },
                    inactive: egui::style::WidgetVisuals {
                        fg_stroke: egui::Stroke {
                            color: egui::Color32::from_rgb(220, 220, 220),
                            ..style.visuals.widgets.inactive.fg_stroke
                        },
                        ..style.visuals.widgets.inactive
                    },
                    active: egui::style::WidgetVisuals {
                        fg_stroke: egui::Stroke {
                            color: egui::Color32::from_rgb(100, 100, 100),
                            ..style.visuals.widgets.active.fg_stroke
                        },
                        bg_fill: egui::Color32::from_rgb(220, 220, 220),
                        ..style.visuals.widgets.active
                    },
                    ..style.visuals.widgets
                },
                ..style.visuals
            },
            
            ..style
        };
        ctx.set_style(style);

        ExplorerApp::default()
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        macro_rules! app_context {
            ($app:ident, $ctx:ident) => {
                let mut $app = AppContext {
                    ctx: $ctx,
                    bigfile: match self.bigfile {
                        Some(ref mut bf) => Some(bf),
                        None => None
                    },
                    shader_cache: &mut self.shader_cache
                };
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if let None = &self.bigfile {
                        if ui.button("Open Bigfile...").clicked() {
                            info!("picking bigfile...");
                            let file = match FileDialog::new()
                            .add_filter("bigfile", &["big"])
                            .pick_file() {
                                Some(f) => f,
                                None => {
                                    info!("bigfile picker cancelled");
                                    return;
                                }
                            };

                            let path = file.to_str().unwrap_or("invalid file path");

                            info!("picked file {}", path);
                            debug!("   {:?}", file);
                
                            let path = String::from(path);
                            let mut bigfile = match Bigfile::new::<BigfileIOPacked>(path) {
                                Ok(bf) => bf,
                                Err(err) => {
                                    error!("{}", &err);
                                    return;
                                }
                            };

                            if let Err(err) = bigfile.load_metadata() {
                                error!("{}", &err);
                            }
                
                            self.bigfile = Some(bigfile);
                        }
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.separator();
                ui.menu_button("Settings", |ui| {
                    app_context!(app, ctx);
                    self.side_panel.settings_menu(ui, &mut app);
                    self.fe_view.settings_menu(ui, &mut app);
                });
                ui.separator();
            });
        });

        egui::SidePanel::left("folder_browser").min_width(350.0).default_width(350.0).max_width(800.0).show(ctx, |ui| {
            app_context!(app, ctx);
            self.side_panel.draw(ui, app);
        });
        
        if let Some(key) = self.side_panel.should_open_new_tab() {
            if let Some(ref mut bf) = self.bigfile {
                self.fe_view.open_new_tab(bf, key);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            app_context!(app, ctx);
            self.fe_view.draw(ui, app);
        });
    }
}