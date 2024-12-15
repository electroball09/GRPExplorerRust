//use std::io::Cursor;
//use byteorder::ReadBytesExt;
use super::{ArchetypeImpl, YetiIOError};

#[derive(Default)]
pub struct Dbr {

}

impl ArchetypeImpl for Dbr {
    fn load_from_buf(&mut self, _buf: &[u8]) -> Result<(), YetiIOError> {
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}