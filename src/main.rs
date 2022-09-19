use std::{env, fs::File};
use bigfile::*;
use bigfile::metadata::*;
use bigfile::io::*;

mod bigfile;
#[macro_use] extern crate num_derive;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let path = &args[1];

    // let mut reader = File::open(path).expect("wtf");
    // let header = SegmentHeader::read_from(&mut reader).expect("wtf2");
    // seek_to_bigfile_header(&mut reader, &header);

    let mut bigfile = Bigfile::<BigfileIOPacked>::new(path).expect("oh no why?");
    bigfile.load_metadata().expect("oh no!");
    // println!("{:?}", &bigfile);
}

pub mod consts {
    pub const YETI_BIG: &str = "Yeti.big";
}