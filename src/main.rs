use std::{env, fs::File};
use bigfile::*;
use bigfile::metadata::*;
use bigfile::io::*;
use rfd::FileDialog;

use crate::ui::ExplorerApp;

mod bigfile;
mod ui;
mod util;
#[macro_use] extern crate num_derive;
#[macro_use] extern crate strum_macros;

pub mod consts {
    pub const YETI_BIG: &str = "Yeti.big";
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    env::set_var("RUST_BACKTRACE", "1");

    eframe::run_native("test", eframe::NativeOptions::default(), Box::new(|cc| Box::new(ExplorerApp::new(cc))));

    start_flow();
    // let mut bigfile = Bigfile::new::<BigfileIOPacked>("H:/SteamLibrary/steamapps/common/_Tom Clancy's Ghost Recon Phantoms NA/Game/NCSA-Live/Yeti.big").expect("oh no why?");
    // bigfile.load_metadata().expect("oh no!");

    // eframe::run_native("test", eframe::NativeOptions::default(), ExplorerApp::make_creator());
}

fn start_flow() {

    
}