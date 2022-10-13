use std::env;
use bigfile::*;
use rfd::FileDialog;

use crate::ui::ExplorerApp;

mod bigfile;
mod ui;
mod objects;
mod util;
mod export;
#[macro_use] extern crate num_derive;
extern crate strum_macros;

pub mod consts {
    pub const YETI_BIG: &str = "Yeti.big";
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    env::set_var("RUST_BACKTRACE", "1");

    eframe::run_native("GRP Explorer", eframe::NativeOptions::default(), Box::new(|cc| Box::new(ExplorerApp::new(cc))));
}