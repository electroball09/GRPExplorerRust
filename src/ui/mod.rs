use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use eframe::CreationContext;
use font_loader::system_fonts;
use crate::bigfile::io::*;
use crate::FileDialog;
use crate::Bigfile;

use self::views::*;

pub mod components;
pub mod views;
pub mod editors;

pub type BfRef = Option<Rc<RefCell<Bigfile>>>;

pub struct ExplorerApp {
    pub bigfile: BfRef,
    ft_view: FileTreeView,
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

        ExplorerApp {
            bigfile: None,
            ft_view: FileTreeView::new(None),
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
                            let file = FileDialog::new()
                            .add_filter("bigfile", &["big"])
                            //.set_directory("")
                            .pick_file()
                            .unwrap();
                
                            let path = String::from(file.to_str().unwrap());
                            let mut bigfile = Bigfile::new::<BigfileIOPacked>(path).expect("oh no why?");
                            bigfile.load_metadata().expect("oh no!");
                
                            self.bigfile.replace(Rc::new(RefCell::new(bigfile)));
                
                            self.ft_view = FileTreeView::new(self.bigfile.clone());
                            self.fe_view = FileEditorTabs::new(self.bigfile.clone());
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("folder_browser").min_width(400.0).max_width(400.0).resizable(false).show(ctx, |ui| {
            self.ft_view.draw(ui, ctx);
            if let Some(key) = self.ft_view.did_click_file() {
                self.fe_view.open_new_tab(key);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.fe_view.draw(ui, ctx);
        });
    }
}