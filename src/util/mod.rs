use std::io::Read;
use byteorder::ReadBytesExt;

pub mod dds_header;

pub fn read_nul_term_string(rdr: &mut impl Read) -> String {
    let mut s = String::new();
    let mut b: u8 = rdr.read_u8().unwrap();
    while b != 0 {
        s.push(b.into());
        b = rdr.read_u8().unwrap();
    }
    s
}