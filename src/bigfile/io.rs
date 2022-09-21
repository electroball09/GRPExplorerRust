use std::fs::{File};
use std::io::{Error, SeekFrom, Seek};
use std::collections::HashMap;

use crate::util::IndAcc;

use super::metadata::{SegmentHeader, BigfileHeader, FileEntry, FolderEntry};

pub fn seek_to_bigfile_header(reader: &mut impl Seek, seg_header: &SegmentHeader) -> Result<u64, Error> {
    reader.seek(SeekFrom::Start(seg_header.header_offset as u64))
}

pub fn seek_to_file_table(reader: &mut impl Seek, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<u64, Error> {
    reader.seek(SeekFrom::Start((seg_header.header_offset + 128) as u64))
}

pub fn seek_to_folder_table(reader: &mut impl Seek, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<u64, Error> {
    reader.seek(SeekFrom::Start((seg_header.header_offset + 128 + bf_header.num_files * 100) as u64))
}

pub trait BigfileIO {
    fn create_from_path(path: &str) -> Result<Self, Error> where Self: Sized;

    fn get_path(&self) -> &str;

    fn load_segment_header(&mut self) -> Result<SegmentHeader, String>;
    fn load_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, String>;
    fn load_file_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u32, FileEntry>, String>;
    fn load_folder_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u16, FolderEntry>, String>;
}

#[derive(Debug)]
pub struct BigfileIOPacked {
    file: File,
    path: String,
}

impl BigfileIO for BigfileIOPacked {
    fn create_from_path(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;

        let packed = BigfileIOPacked {
            file,
            path: String::from(path),
        };
        Ok(packed)
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn load_segment_header(& mut self) -> Result<SegmentHeader,String> {
        if let Err(_) = self.file.seek(SeekFrom::Start(0)) {
            return Err(String::from("failed to seek to file start!"));
        }

        println!("loading segment header");

        let seg_header = match SegmentHeader::read_from(&mut self.file) {
            Ok(header) => header,
            Err(error) => return Err(error.to_string())
        };

        Ok(seg_header)
    }

    fn load_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, String> {
        if let Err(_) = seek_to_bigfile_header(&mut self.file, seg_header) {
            return Err(String::from("could not seek to bigfile header!"));
        }

        println!("loading bigfile header");

        BigfileHeader::read_from(&mut self.file)
    }

    fn load_file_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u32, FileEntry>, String> {
        if let Err(error) = seek_to_file_table(&mut self.file, seg_header, bf_header) {
            return Err(error.to_string());
        }

        println!("loading file table, num_files={}", bf_header.num_files);

        let mut v = HashMap::with_capacity(bf_header.num_files as usize);
        
        let mut i: u32 = 0;
        while i < bf_header.num_files {
            let entry = FileEntry::read_from(&mut self.file)?;
            // println!("{}", entry.get_name());
            v.insert(entry.key, entry);
            i = i + 1;
        };

        Ok(v)
    }

    fn load_folder_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u16, FolderEntry>, String> {
        if let Err(error) = seek_to_folder_table(&mut self.file, seg_header, bf_header) {
            return Err(error.to_string());
        }

        println!("loading folder table, num_folders={}", bf_header.num_folders);

        let mut v = HashMap::with_capacity(bf_header.num_folders as usize);
        
        let mut i: u16 = 0;
        while i < bf_header.num_folders {
            let mut entry = FolderEntry::read_from(&mut self.file)?;
            entry.idx = i;
            // println!("{:?}", &entry);
            v.insert(i, entry);
            i = i + 1;
        }

        Ok(v)
    }
}