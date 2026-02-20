use super::ArchetypeImpl;
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use glam::*;
use std::ops::Range;
use crate::util::mesh_util::*;

#[derive(Default)]
pub struct MeshMetadata {
    pub num_submeshes: u8,
    pub version: i32,
    pub unk_dat01: [u8; 10],
    pub submeshes: Vec<SubmeshDescriptor>,
    pub unk_dat02: [u8; 32],
}

#[derive(Default)]
pub struct SubmeshDescriptor {
    pub vtx_start: u16,
    pub vtx_num: u16,
    pub face_start: u16,
    pub face_num: u16,
    pub unk_03: u16,
    pub unk_04: u16,
    pub unk_05: u16,
    pub unk_vec: Vec<u8>,
}

impl ArchetypeImpl for MeshMetadata {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::YetiIOError> {
        let mut cursor = Cursor::new(buf);

        self.num_submeshes = cursor.read_u8()?;
        self.version = cursor.read_i32::<LittleEndian>()?;
        if self.version != 2 {
            return Err(format!("unknown version: {}", self.version).into());
        }

        cursor.read(&mut self.unk_dat01)?;
        for _ in 0..self.num_submeshes {
            let mut desc = SubmeshDescriptor {
                vtx_start: cursor.read_u16::<LittleEndian>()?,
                vtx_num: cursor.read_u16::<LittleEndian>()?,
                face_start: cursor.read_u16::<LittleEndian>()?,
                face_num: cursor.read_u16::<LittleEndian>()?,
                unk_03: cursor.read_u16::<LittleEndian>()?,
                unk_04: cursor.read_u16::<LittleEndian>()?,
                unk_05: cursor.read_u16::<LittleEndian>()?,
                unk_vec: vec![0; cursor.read_u8()? as usize],
            };
            for idx in 0..desc.unk_vec.len() {
                desc.unk_vec[idx] = cursor.read_u8()?;
            }
            self.submeshes.push(desc);
        }
        cursor.read(&mut self.unk_dat02)?;

        if self.unk_dat02[0] != 0x20 && self.unk_dat02[31] != 0xFF {
            //there's some older unused files with a different format, cba to figure it out tho
            return Err("unknown data in mesh metadata".into());
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default()
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
    pub f0: u32,
    pub f1: u32,
    pub f2: u32
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Weight {
    pub bone: u32,
    pub weight: f32,
}

#[derive(Default)]
pub struct VertexData {
    pub bufs: Vec<[u8; 32]>,
    pub pos: Vec<Vec3>,
    pub uv0: Vec<Vec2>,
    pub uv1: Vec<Vec2>,
    pub weights: Vec<[Weight; 4]>,
    pub tangents: Vec<Vec3>,
    pub normals: Vec<Vec3>,
}

impl ArchetypeImpl for MeshData {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::YetiIOError> {
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

        let mut vbuf: [u8; 32] = [0; 32];
        let mut tnbuf: [u8; 8] = [0; 8];
        let mut bufs: Vec<[u8; 32]> = Vec::with_capacity(self.num_vertices as usize);
        let mut pos: Vec<Vec3> = Vec::with_capacity(self.num_vertices as usize);
        let mut uv0: Vec<Vec2> = Vec::with_capacity(self.num_vertices as usize);
        let mut uv1: Vec<Vec2> = Vec::with_capacity(self.num_vertices as usize);
        let mut weights: Vec<[Weight; 4]> = Vec::with_capacity(self.num_vertices as usize);
        let mut tangents: Vec<Vec3> = Vec::with_capacity(self.num_vertices as usize);
        let mut normals: Vec<Vec3> = Vec::with_capacity(self.num_vertices as usize);
        for _ in 0..self.num_vertices {
            cursor.read(&mut vbuf)?;
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
            uv1.push(
                 Vec2::new(
                uvi16_to_float(vbufr.read_i16::<LittleEndian>()?),
                uvi16_to_float(vbufr.read_i16::<LittleEndian>()?),
            ));
            weights.push({
                let mut weights = [Weight::default(); 4];
                for i in 0..4 {
                    weights[i].weight = vbufr.read_u8()? as f32 / 255.0;
                }
                for i in 0..4 {
                    weights[i].bone = vbufr.read_u8()? as u32;
                }
                weights
            });

            {
                vbufr.read_exact(&mut tnbuf)?;
                let (normal, tangent, _) = bytes_to_rgb10_a2_tnb_with_a(&tnbuf);

                tangents.push(tangent.truncate());
                normals.push(normal.truncate());
            }
            
            bufs.push(vbuf);
        }
        self.vertex_data = VertexData {
            bufs,
            pos,
            uv0,
            uv1,
            weights,
            tangents,
            normals,
        };

        self.faces = Vec::with_capacity(self.num_indices as usize);

        for _ in 0..self.num_indices / 3 {
            self.faces.push(FaceData {
                f0: cursor.read_u16::<LittleEndian>()? as u32,
                f1: cursor.read_u16::<LittleEndian>()? as u32,
                f2: cursor.read_u16::<LittleEndian>()? as u32
            });
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}

impl MeshData {
    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        let mut min = Vec3::new(0.0, 0.0, 0.0);
        let mut max = Vec3::new(0.0, 0.0, 0.0);

        for pos in self.vertex_data.pos.iter() {
            for i in 0..3 {
                min[i] = f32::min(min[i], pos[i]);
                max[i] = f32::max(max[i], pos[i]);
            }
        }

        (min, max)
    }

    pub fn bounding_box_range(&self, range: Range<usize>) -> (Vec3, Vec3) {
        let mut min = Vec3::new(0.0, 0.0, 0.0);
        let mut max = Vec3::new(0.0, 0.0, 0.0);

        for idx in range {
            let pos = &self.vertex_data.pos[idx];
            for i in 0..3 {
                min[i] = f32::min(min[i], pos[i]);
                max[i] = f32::max(max[i], pos[i]);
            }
        }

        (min, max)
    }
}