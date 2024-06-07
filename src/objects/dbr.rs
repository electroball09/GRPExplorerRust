use std::io::Cursor;
use byteorder::ReadBytesExt;
use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct Dbr {

}

impl ArchetypeImpl for Dbr {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}