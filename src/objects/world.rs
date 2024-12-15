use super::*;
// use byteorder::{ReadBytesExt, LittleEndian};
// use glam::*;
// use bitflags::bitflags;
// use crate::util::load_util::*;

#[derive(Default)]
pub struct World {

}

impl ArchetypeImpl for World {
    fn load_from_buf(&mut self, _buf: &[u8]) -> Result<(), YetiIOError> {
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}