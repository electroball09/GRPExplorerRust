use log::*;
use log4rs::{*, append::{file::FileAppender, console::ConsoleAppender}, encode::pattern::PatternEncoder, config::{Appender, Root}};

pub fn setup_log() -> Config {
    let log_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {h({({l}):5.5})} | {f}:{L} — {m}{n}";

    let log_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .build("$ENV{LOCALAPPDATA}/GXR/log.log")
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