use log::*;
use log4rs::{*, append::{file::FileAppender, console::ConsoleAppender}, encode::pattern::PatternEncoder, config::{Appender, Root}};
use platform_dirs::*;

pub fn setup_log() -> Config {
    let log_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({({l}):5.5})} | {f}:{L} — {m}{n}";

    let log_folder = AppDirs::new(Some("GXR"), false).unwrap().state_dir.join("GXR.log");

    let log_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .build(log_folder)
        .unwrap();

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .build();

    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("log_ap", Box::new(log_appender)))
        .build(Root::builder().appender("stdout").appender("log_ap").build(LevelFilter::Info))
        .unwrap()
}