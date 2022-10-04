use super::ArchetypeImpl;
use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use glam::*;

#[derive(Default)]
pub struct MeshMetadata {

}

impl ArchetypeImpl for MeshMetadata {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::LoadError> {
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}

#[derive(Default)]
pub struct MeshData {
    pub unk_01: u32,
    pub unk_02: u32,
    pub unk_03: u8,
    pub num_vertices: u16,
    pub num_indices: u32,
    pub data_offset: u32,

    pub old_num_submeshes: u16,
    pub old_submesh_size: u32,

    pub num_submeshes: u16,
    pub pivot_offset: Vec3,
    pub uniform_scale: f32,

    pub vertices: Vec<VertexData>,
    pub faces: Vec<[u16; 3]>
}

#[derive(Default)]
pub struct VertexData {
    pub pos: Vec3,
}

fn snorm16_to_float(v: i16) -> f32 {
    f32::max((v as f32) / 32767.0, -1.0)
}

impl ArchetypeImpl for MeshData {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::LoadError> {
        let mut cursor = Cursor::new(buf);

        self.unk_01 = cursor.read_u32::<LittleEndian>()?;
        self.unk_02 = cursor.read_u32::<LittleEndian>()?;
        self.unk_03 = cursor.read_u8()?;
        self.num_vertices = cursor.read_u16::<LittleEndian>()?;
        self.num_indices = cursor.read_u32::<LittleEndian>()?;
        self.data_offset = cursor.read_u32::<LittleEndian>()?;

        cursor.seek(SeekFrom::Start(0x19))?;

        self.old_num_submeshes = cursor.read_u16::<LittleEndian>()?;
        self.old_submesh_size = cursor.read_u32::<LittleEndian>()?;

        cursor.seek(SeekFrom::Start(0x21))?;

        self.num_submeshes = cursor.read_u16::<LittleEndian>()?;

        cursor.seek(SeekFrom::Start(0x37))?;

        self.pivot_offset = Vec3::new(
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?
        );
        self.uniform_scale = cursor.read_f32::<LittleEndian>()?;

        cursor.seek(SeekFrom::Start((0x47 + self.data_offset) as u64))?;

        self.vertices = Vec::with_capacity(self.num_vertices as usize);
        let mut i = 0;
        while i < self.num_vertices {
            self.vertices.push(VertexData {
                pos: (Vec3::new(
                    snorm16_to_float(cursor.read_i16::<LittleEndian>()?),
                    snorm16_to_float(cursor.read_i16::<LittleEndian>()?),
                    snorm16_to_float(cursor.read_i16::<LittleEndian>()?)
                ) * snorm16_to_float(cursor.read_i16::<LittleEndian>()?)
                    * self.uniform_scale)
                    + self.pivot_offset
            });
            cursor.seek(SeekFrom::Current(24))?;
            i += 1;
        }

        self.faces = Vec::with_capacity(self.num_indices as usize);
        let mut i2 = 0;
        while i2 < self.num_indices {
            let a = cursor.read_u16::<LittleEndian>()?;
            let b = cursor.read_u16::<LittleEndian>()?;
            let c = cursor.read_u16::<LittleEndian>()?;
            self.faces.push([a, b, c]);
            i2 += 3;
        }

        Ok(())
    }

    fn unload(&mut self) {
        self.vertices.clear();
        self.vertices.shrink_to(1);
        self.faces.clear();
        self.faces.shrink_to(1);
    }
}