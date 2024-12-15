use log::*;
use std::fs::File;
use std::io::{Error, SeekFrom, Seek, Read, Cursor};
use std::collections::HashMap;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::read::ZlibDecoder;

use crate::YetiIOError;

use super::metadata::{SegmentHeader, BigfileHeader, FileEntry, FolderEntry};
use super::YKey;

pub fn seek_to_bigfile_header(reader: &mut impl Seek, seg_header: &SegmentHeader) -> Result<u64, Error> {
    reader.seek(SeekFrom::Start(seg_header.header_offset as u64))
}

pub fn seek_to_file_table(reader: &mut impl Seek, seg_header: &SegmentHeader, _bf_header: &BigfileHeader) -> Result<u64, Error> {
    reader.seek(SeekFrom::Start((seg_header.header_offset + 128) as u64))
}

pub fn seek_to_folder_table(reader: &mut impl Seek, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<u64, Error> {
    reader.seek(SeekFrom::Start(seg_header.header_offset + 128u64 + bf_header.num_files as u64 * FileEntry::struct_size(bf_header.version) as u64))
}

pub fn seek_to_file_data(reader: &mut impl Seek, seg_header: &SegmentHeader, bf_header: &BigfileHeader, offset: u32) -> Result<u64, Error> {
    seek_to_folder_table(reader, seg_header, bf_header)?;
    let folder_data_size = bf_header.num_folders as u64 * FolderEntry::struct_size(bf_header.version) as u64;
    reader.seek(SeekFrom::Current(folder_data_size as i64))?;
    let stream_pos = reader.stream_position().unwrap();
    let rem = stream_pos % 8;
    if rem != 0 {
        reader.seek(SeekFrom::Current((8u64 - rem) as i64))?;
    }
    reader.seek(SeekFrom::Current((offset as i64) * 8))
}

pub fn parse_and_remove_refs(buf: &[u8]) -> (Vec<YKey>, &[u8]) {
    let mut cursor = Cursor::new(&buf);
    let num_refs = cursor.read_u32::<LittleEndian>().unwrap();
    let mut refs: Vec<YKey> = Vec::with_capacity(num_refs as usize);
    let mut i = 0;
    while i < num_refs {
        refs.push(cursor.read_u32::<LittleEndian>().unwrap().into());
        i += 1;
    }
    
    (refs, &buf[4 + (4 * num_refs as usize)..])
}

#[allow(unused)]
pub trait BigfileIO {
    fn create_from_path(path: &str) -> Result<Self, Error> where Self: Sized;

    fn get_path(&self) -> &str;

    fn read_segment_header(&mut self) -> Result<SegmentHeader, YetiIOError>;
    fn read_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, YetiIOError>;
    fn read_file_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<YKey, FileEntry>, YetiIOError>;
    fn read_folder_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u16, FolderEntry>, YetiIOError>;

    fn read_file(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader, entry: &FileEntry) -> Result<Vec<u8>, YetiIOError>;
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

    fn read_segment_header(& mut self) -> Result<SegmentHeader, YetiIOError> {
        self.file.seek(SeekFrom::Start(0))?;

        info!("loading segment header");


        let seg_header = SegmentHeader::read_from(&mut self.file)?;

        trace!("{}", seg_header);

        Ok(seg_header)
    }

    fn read_bigfile_header(&mut self, seg_header: &SegmentHeader) -> Result<BigfileHeader, YetiIOError> {
        seek_to_bigfile_header(&mut self.file, seg_header)?;

        info!("loading bigfile header");

        let header = BigfileHeader::read_from(&mut self.file)?;

        Ok(header)
    }

    fn read_file_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<YKey, FileEntry>, YetiIOError> {
        seek_to_file_table(&mut self.file, seg_header, bf_header)?;

        info!("loading file table, num_files={}", bf_header.num_files);

        let mut v = HashMap::with_capacity(bf_header.num_files as usize);

        const BUF_SIZE: usize = 100;
        let struct_size = FileEntry::struct_size(bf_header.version);
        let mut buf: Vec<u8> = vec![0; BUF_SIZE * struct_size];
        let mut slice = &buf[..];
        
        for i in 0..bf_header.num_files {
            if i % BUF_SIZE as u32 == 0 {
                self.file.read(&mut buf).unwrap();
                slice = &buf[..];
            }

            let entry = FileEntry::read_from(&mut slice, bf_header.version)?;
            debug!("FILE ENTRY:   {}", entry);
            v.insert(entry.key, entry);
        }

        Ok(v)
    }

    fn read_folder_table(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader) -> Result<HashMap<u16, FolderEntry>, YetiIOError> {
        seek_to_folder_table(&mut self.file, seg_header, bf_header)?;

        info!("loading folder table, num_folders={}", bf_header.num_folders);

        let mut v = HashMap::with_capacity(bf_header.num_folders as usize);

        const BUF_SIZE: usize = 100;
        let struct_size = FolderEntry::struct_size(bf_header.version);
        let mut buf: Vec<u8> = vec![0; BUF_SIZE * struct_size];
        let mut slice = &buf[..];
        
        for i in 0..bf_header.num_folders {
            if i as usize % BUF_SIZE == 0 {
                self.file.read(&mut buf).unwrap();
                slice = &buf[..];
            }

            let mut entry = FolderEntry::read_from(&mut slice, bf_header.version)?;
            entry.idx = i;
            debug!("FOLDER ENTRY:   {}", &entry);
            v.insert(i, entry);
        }

        Ok(v)
    }

    fn read_file(&mut self, seg_header: &SegmentHeader, bf_header: &BigfileHeader, entry: &FileEntry) -> Result<Vec<u8>, YetiIOError> {
        if entry.offset == 0xFFFFFFFF {
            //warn!("could not seek, offset is invalid");
            return Err("invalid offset".into());
        }

        seek_to_file_data(&mut self.file, seg_header, bf_header, entry.offset)?;

        if entry.zip {
            let _compressed_size = self.file.read_u32::<LittleEndian>().unwrap();
            let decompressed_size = self.file.read_u32::<LittleEndian>().unwrap();

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