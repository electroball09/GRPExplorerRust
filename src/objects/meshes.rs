use super::ArchetypeImpl;
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use glam::*;

#[derive(Default)]
pub struct MeshMetadata {

}

impl ArchetypeImpl for MeshMetadata {
    fn load_from_buf(&mut self, _buf: &[u8]) -> Result<(), super::LoadError> {
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

    pub vertex_data: VertexData,
    pub faces: Vec<FaceData>
}

#[derive(Default)]
pub struct FaceData {
    pub f0: u16,
    pub f1: u16,
    pub f2: u16
}

#[derive(Default)]
pub struct VertexData {
    pub bufs: Vec<[u8; 32]>,
    pub pos: Vec<Vec3>,
    pub uv0: Vec<Vec2>
}

impl VertexData {
    pub fn clear_data(&mut self) {
        *self = Default::default();
    }
}

fn snorm16_to_float(v: i16) -> f32 {
    f32::max((v as f32) / 32767.0, -1.0)
}

fn uvi16_to_float(v: i16) -> f32 {
    f32::from(v) / 1024.0
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

        let mut bufs: Vec<[u8; 32]> = Vec::with_capacity(self.num_vertices as usize);
        let mut pos: Vec<Vec3> = Vec::with_capacity(self.num_vertices as usize);
        let mut uv0: Vec<Vec2> = Vec::with_capacity(self.num_vertices as usize);
        for _i in 0..self.num_vertices {
            let mut vbuf: [u8; 32] = [0; 32];
            cursor.read(&mut vbuf)?;
            bufs.push(vbuf.clone());
            let mut vbufr: &[u8] = &vbuf;
            pos.push((Vec3::new(
                    snorm16_to_float(vbufr.read_i16::<LittleEndian>()?),
                    snorm16_to_float(vbufr.read_i16::<LittleEndian>()?),
                    snorm16_to_float(vbufr.read_i16::<LittleEndian>()?)
                ) * snorm16_to_float(vbufr.read_i16::<LittleEndian>()?)
                    * self.uniform_scale)
                    + self.pivot_offset
                
            );
            uv0.push(
                 Vec2::new(
                uvi16_to_float(vbufr.read_i16::<LittleEndian>()?),
                uvi16_to_float(vbufr.read_i16::<LittleEndian>()?),
            ));
            //cursor.seek(SeekFrom::Current(20))?;
        }
        self.vertex_data = VertexData {
            bufs,
            pos,
            uv0,
        };

        self.faces = Vec::with_capacity(self.num_indices as usize);
        let mut i2 = 0;
        while i2 < self.num_indices {
            self.faces.push(FaceData {
                f0: cursor.read_u16::<LittleEndian>()?,
                f1: cursor.read_u16::<LittleEndian>()?,
                f2: cursor.read_u16::<LittleEndian>()?
            });
            i2 += 3;
        }

        Ok(())
    }

    fn unload(&mut self) {
        self.vertex_data.clear_data();
        self.faces.clear();
        self.faces.shrink_to(1);
    }
}