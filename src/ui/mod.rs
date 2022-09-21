use eframe::App;
use eframe::AppCreator;
use eframe::CreationContext;
use egui::Response;
use crate::bigfile::io::*;
use crate::ui;
use crate::FileDialog;
use crate::Bigfile;

pub mod components;
pub mod views;

pub struct ExplorerApp<'a> {
    pub bigfile: Option<Bigfile<'a>>
}

impl <'a> ExplorerApp<'a> {
    pub fn new(cc: &CreationContext<'_>) -> Self {

        ExplorerApp {
            bigfile: None,
        }
    }
}

impl eframe::App for ExplorerApp<'_> {

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        frame.set_window_size(egui::Vec2::new(1500.0, 800.0));

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Bigfile...").clicked() {
                        let file = FileDialog::new()
                        .add_filter("bigfile", &["big"])
                        .set_directory("H:/SteamLibrary/steamapps/common/_Tom Clancy's Ghost Recon Phantoms NA/Game/NCSA-Live")
                        .pick_file()
                        .unwrap();

                        let path = String::from(file.to_str().unwrap());
                        let mut bigfile = Bigfile::new::<BigfileIOPacked>(path).expect("oh no why?");
                        bigfile.load_metadata().expect("oh no!");

                        self.bigfile = Some(bigfile);
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("folder_browser").min_width(400.0).max_width(400.0).resizable(false).show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).auto_shrink([false, false]).show(ui, |ui|{
                if let Some(bf) = &self.bigfile {
                    components::draw_bigfile(&bf, ui, ctx, frame);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("hello");
        });
    }
}