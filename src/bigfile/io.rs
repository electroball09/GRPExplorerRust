use std::fs::File;
use std::io::{Error, SeekFrom, Seek, Read};
use std::collections::HashMap;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::read::ZlibDecoder;

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

pub fn seek_to_file_data(reader: &mut impl Seek, seg_header: &SegmentHeader, bf_header: &BigfileHeader, offset: u32) -> Result<u64, Error> {
    //println!("{}", &offset);
    seek_to_folder_table(reader, seg_header, bf_header)?;
    let folder_data_size = bf_header.num_folders as i64 * 64;
    reader.seek(SeekFrom::Current(folder_data_size))?;
    let byte_pack_offset = 8 - ((reader.stream_position().unwrap()) % 8);
    reader.seek(SeekFrom::Current(byte_pack_offset as i64))?;
    reader.seek(SeekFrom::Current((offset as i64) * 8))
}

pub trait BigfileIO {
    fn create_from_path(path: &str) -> Result<Self, Error> where Self: Sized;

    fn get_path(&self) -> &str;

    fn read_segment_header(&mut self) -> Result<SegmentHeader, String>;
    fn read_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, String>;
    fn read_file_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u32, FileEntry>, String>;
    fn read_folder_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u16, FolderEntry>, String>;

    fn read_file(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader, entry: &FileEntry) -> Result<Vec<u8>, String>;
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

    fn read_segment_header(& mut self) -> Result<SegmentHeader,String> {
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

    fn read_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, String> {
        if let Err(_) = seek_to_bigfile_header(&mut self.file, seg_header) {
            return Err(String::from("could not seek to bigfile header!"));
        }

        println!("loading bigfile header");

        BigfileHeader::read_from(&mut self.file)
    }

    fn read_file_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u32, FileEntry>, String> {
        if let Err(error) = seek_to_file_table(&mut self.file, seg_header, bf_header) {
            return Err(error.to_string());
        }

        println!("loading file table, num_files={}", bf_header.num_files);

        let mut v = HashMap::with_capacity(bf_header.num_files as usize);

        const FILE_BUF_SIZE: usize = 100;

        let mut buf: [u8; 100 * FILE_BUF_SIZE] = [0; 100 * FILE_BUF_SIZE];
        let mut slice = &buf[..];
        
        let mut i: u32 = 0;
        while i < bf_header.num_files {
            if i % FILE_BUF_SIZE as u32 == 0 {
                self.file.read(&mut buf).unwrap();
                slice = &buf[..];
            }

            let entry = FileEntry::read_from(&mut slice)?;
            // println!("{}", entry.get_name());
            v.insert(entry.key, entry);
            i = i + 1;
        };

        Ok(v)
    }

    fn read_folder_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u16, FolderEntry>, String> {
        if let Err(error) = seek_to_folder_table(&mut self.file, seg_header, bf_header) {
            return Err(error.to_string());
        }

        println!("loading folder table, num_folders={}", bf_header.num_folders);

        let mut v = HashMap::with_capacity(bf_header.num_folders as usize);

        const FOLDER_BUF_SIZE: usize = 100;

        let mut buf: [u8; 64 * FOLDER_BUF_SIZE] = [0; 64 * FOLDER_BUF_SIZE];
        let mut slice = &buf[..];
        
        let mut i: u16 = 0;
        while i < bf_header.num_folders {
            if i as usize % FOLDER_BUF_SIZE == 0 {
                self.file.read(&mut buf).unwrap();
                slice = &buf[..];
            }

            let mut entry = FolderEntry::read_from(&mut slice)?;
            entry.idx = i;
            // println!("{:?}", &entry);
            v.insert(i, entry);
            i = i + 1;
        }

        Ok(v)
    }

    fn read_file(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader, entry: &FileEntry) -> Result<Vec<u8>, String> {
        if entry.offset == 0xFFFFFFFF {
            println!("could not seek, offset is invalid");
            return Err(String::from("invalid offset"));
        }

        if let Err(error) = seek_to_file_data(&mut self.file, seg_header, bf_header, entry.offset) {
            return Err(error.to_string());
        }

        if entry.zip {
            let compressed_size = self.file.read_u32::<LittleEndian>().unwrap();
            let decompressed_size = self.file.read_u32::<LittleEndian>().unwrap();
            //dbg!(&compressed_size);
            //dbg!(&decompressed_size);

            let mut v = vec![0; decompressed_size as usize];
            let mut decompress = ZlibDecoder::new(&mut self.file);
            decompress.read_exact(&mut v[..]).unwrap();
            Ok(v)
        } else {
            let size = self.file.read_i32::<LittleEndian>().unwrap();
            let mut v = vec![0; size as usize];
            self.file.read(&mut v[..]).unwrap();
            Ok(v)
        }
    }
}