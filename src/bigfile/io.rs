use std::fs::{self, read_dir, File};
use std::io::{Error, SeekFrom, Seek};

use super::metadata::{SegmentHeader, BigfileHeader, seek_to_bigfile_header};

pub trait BigfileIO {
    fn create_from_path(path: &str) -> Result<Self, Error> where Self: Sized;

    fn load_segment_header(&mut self) -> Result<SegmentHeader, String>;
    fn load_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, String>;
}

#[derive(Debug)]
pub struct BigfileIOPacked {
    path: String,
    file: File,
}

impl BigfileIO for BigfileIOPacked {
    fn create_from_path(path: &str) -> Result<Self, Error> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(error) => return Err(error)
        };

        let packed = BigfileIOPacked {
            path: String::from(path),
            file: file
        };
        Ok(packed)
    }

    fn load_segment_header(& mut self) -> Result<SegmentHeader,String> {
        if let Err(_) = self.file.seek(SeekFrom::Start(0)) {
            return Err(String::from("failed to seek to file start!"));
        }

        let seg_header = match SegmentHeader::read_from(&mut self.file) {
            Ok(header) => header,
            Err(error) => return Err(error.to_string())
        };

        Ok(seg_header)
    }

    fn load_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, String> {
        if let Err(_) = seek_to_bigfile_header(&mut self.file, seg_header) {
            return Err(String::from("could not seek to segment header!"));
        }

        Ok(BigfileHeader::default())
    }
}