use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::CreationContext;
use egui::Color32;
use font_loader::system_fonts;
use crate::bigfile::io::*;
use crate::FileDialog;
use crate::Bigfile;
use log::*;

use self::views::*;
use self::views::side_panel::SidePanelView;
use views::editor_tabs_view::FileEditorTabs;

pub mod views;
pub mod editors;

pub type BfRef = Option<Rc<RefCell<Bigfile>>>;

pub struct ExplorerApp {
    pub bigfile: BfRef,
    side_panel: views::side_panel::SidePanelView,
    fe_view: FileEditorTabs,
}

impl ExplorerApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
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

        cc.egui_ctx.set_fonts(egui::FontDefinitions {
            font_data: m,
            families: m2
        });

        let style = cc.egui_ctx.style().as_ref().clone();
        let style = egui::Style {
            visuals: egui::Visuals {
                widgets: egui::style::Widgets {
                    noninteractive: egui::style::WidgetVisuals {
                        fg_stroke: egui::Stroke {
                            color: Color32::from_rgb(170, 170, 170),
                            ..style.visuals.widgets.noninteractive.fg_stroke
                        },
                        ..style.visuals.widgets.noninteractive
                    },
                    inactive: egui::style::WidgetVisuals {
                        fg_stroke: egui::Stroke {
                            color: Color32::from_rgb(220, 220, 220),
                            ..style.visuals.widgets.inactive.fg_stroke
                        },
                        ..style.visuals.widgets.inactive
                    },
                    active: egui::style::WidgetVisuals {
                        fg_stroke: egui::Stroke {
                            color: Color32::from_rgb(100, 100, 100),
                            ..style.visuals.widgets.active.fg_stroke
                        },
                        bg_fill: Color32::from_rgb(220, 220, 220),
                        ..style.visuals.widgets.active
                    },
                    ..style.visuals.widgets
                },
                ..style.visuals
            },
            
            ..style
        };
        cc.egui_ctx.set_style(style);

        ExplorerApp {
            bigfile: None,
            side_panel: SidePanelView::new(None),
            fe_view: FileEditorTabs::new(None),
        }
    }
}

impl eframe::App for ExplorerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        frame.set_window_size(egui::Vec2::new(1500.0, 800.0));

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
                
                            self.bigfile.replace(Rc::new(RefCell::new(bigfile)));
                
                            self.side_panel.set_bigfile(self.bigfile.clone());
                            self.fe_view.set_bigfile(self.bigfile.clone());
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.separator();
                ui.menu_button("Settings", |ui| {
                    self.side_panel.settings_menu(ui, ctx);
                    self.fe_view.settings_menu(ui, ctx);
                });
                ui.separator();
            });
        });

        egui::SidePanel::left("folder_browser").min_width(350.0).default_width(350.0).max_width(800.0).show(ctx, |ui| {
            self.side_panel.draw(ui, ctx);
            if let Some(key) = self.side_panel.should_open_new_tab() {
                self.fe_view.open_new_tab(key);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.fe_view.draw(ui, ctx);
        });
    }
}