use std::env;
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
#[macro_use] extern crate num_derive;
extern crate strum_macros;

pub mod consts {
    pub const YETI_BIG: &str = "Yeti.big";
}

fn main() {
    let args: Vec<String> = env::args().collect();

    env::set_var("RUST_BACKTRACE", "1");

    let log_cfg = setup_log();
    log4rs::init_config(log_cfg).unwrap();
    
    info!("app initialized");
    debug!("args - {:?}", args);

    explorer_app_start();
}