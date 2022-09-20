use std::{env, fs::File};
use bigfile::*;
use bigfile::metadata::*;
use bigfile::io::*;

use crate::ui::ExplorerApp;

mod bigfile;
mod ui;
mod util;
#[macro_use] extern crate num_derive;

pub mod consts {
    pub const YETI_BIG: &str = "Yeti.big";
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    env::set_var("RUST_BACKTRACE", "1");

    let mut bigfile = Bigfile::<BigfileIOPacked>::new("H:/SteamLibrary/steamapps/common/_Tom Clancy's Ghost Recon Phantoms NA/Game/NCSA-Live/Yeti.big").expect("oh no why?");
    bigfile.load_metadata().expect("oh no!");
    // println!("{:?}", &bigfile);

    eframe::run_native("test", eframe::NativeOptions::default(), ExplorerApp::make_creator());
}
