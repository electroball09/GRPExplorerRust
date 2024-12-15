use super::*;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use glam::Vec3;

#[derive(Default)]
pub struct Vxt {
    pub header: [u8; 4],
    pub vxt: Vec<Vec3>,
}

impl ArchetypeImpl for Vxt {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {
        let mut cursor = Cursor::new(buf);
        cursor.read(&mut self.header)?;

        let num_vtx = cursor.read_u16::<LittleEndian>()?;
        self.vxt = Vec::with_capacity(num_vtx as usize);
        for _ in 0..num_vtx {
            self.vxt.push(Vec3::new(
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?
            ));
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}