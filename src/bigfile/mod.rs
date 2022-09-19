pub mod metadata;
pub mod io;

use metadata::*;
use io::*;

#[derive(Debug)]
pub struct Bigfile<T: BigfileIO> {
    pub segment_header: SegmentHeader,
    pub bigfile_header: BigfileHeader,
    pub file_table: Vec<FileEntry>,
    pub folder_table: Vec<FolderEntry>,
    pub io: T,
}

impl<T> Bigfile<T> where T: BigfileIO {
    pub fn new(path: &str) -> Result<Bigfile<T>, String> {
        let io = match T::create_from_path(path) {
            Ok(io) => io,
            Err(error) => return Err(error.to_string())
        };
        
        let bigfile = Bigfile {
            segment_header: SegmentHeader::default(),
            bigfile_header: BigfileHeader::default(),
            io: io,
            file_table: Vec::new(),
            folder_table: Vec::new(),
        };

        Ok(bigfile)
    }

    pub fn load_metadata(&mut self) -> Result<(), String> {
        self.segment_header = match self.io.load_segment_header() {
            Ok(header) => header,
            Err(error) => return Err(String::from(error))
        };
        self.bigfile_header = self.io.load_bigfile_header(&self.segment_header)?;
        self.file_table = self.io.load_file_table(&self.segment_header, &self.bigfile_header)?;
        self.folder_table = self.io.load_folder_table(&self.segment_header, &self.bigfile_header)?;
        Ok(())
    }
}
