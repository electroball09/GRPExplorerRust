use super::*;
// use byteorder::{ReadBytesExt, LittleEndian};
// use glam::*;
// use bitflags::bitflags;
// use crate::util::load_util::*;

#[derive(Default)]
pub struct World {

}

impl ArchetypeImpl for World {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}