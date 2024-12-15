use super::*;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use glam::Vec4;

#[derive(Default)]
pub struct VertexColors {
    pub colors: Vec<Vec4>,
}

impl ArchetypeImpl for VertexColors {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {
        let mut cursor = Cursor::new(buf);

        let num_vtx = cursor.read_u16::<LittleEndian>()?;
        self.colors = Vec::with_capacity(num_vtx as usize);
        for _ in 0..num_vtx {
            self.colors.push(Vec4::new(
                cursor.read_u8()? as f32 / 255.0,
                cursor.read_u8()? as f32 / 255.0,
                cursor.read_u8()? as f32 / 255.0,
                cursor.read_u8()? as f32 / 255.0
            ));
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}