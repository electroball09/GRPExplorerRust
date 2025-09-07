use std::env;
use clap::Parser;
use rfd::FileDialog;
use log::*;
use ui::explorer_init::*;

use crate::bigfile::*;
use crate::util::log_config::*;

mod bigfile;
mod ui;
mod objects;
mod util;
mod export;
mod ggl;

#[cfg(feature = "eframe")]
pub use eframe::egui;
#[cfg(feature = "eframe")]
pub use eframe::glow as glow;
#[cfg(feature = "miniquad")]
pub use egui as egui;

pub mod consts {
    pub const YETI_BIG: &str = "Yeti.big";
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pub log_level: Option<LevelFilter>
}

fn main() {
    let args = Args::parse();

    info!("{:?}", args.log_level);

    env::set_var("RUST_BACKTRACE", "1");

    let log_cfg = setup_log(match args.log_level {
        Some(level) => level,
        None => LevelFilter::Info
    });
    log4rs::init_config(log_cfg).unwrap();
    
    info!("app initialized");

    unsafe {
        explorer_app_start();
    }
}