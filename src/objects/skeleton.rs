use crate::util::load_util::read_mat4;

use super::ArchetypeImpl;
use std::{fmt::Display, io::{Cursor, Read}};
use byteorder::ReadBytesExt;
use glam::Mat4;

#[derive(Default)]
pub struct Skeleton {
    pub version: u8,
    pub num_bones: u8,
    pub unk_01: u8,
    pub bones: Vec<Bone>,
}

#[derive(Clone, Copy, Default)]
pub struct BoneParent(pub Option<u8>);

impl From<Option<u8>> for BoneParent {
    fn from(value: Option<u8>) -> Self {
        BoneParent(value)
    }
}

impl From<BoneParent> for Option<u8> {
    fn from(value: BoneParent) -> Self {
        value.0
    }
}

impl Display for BoneParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "None")
        }
    }
}

pub struct Bone {
    name: String,
    pub unk_01: [u8; 4],
    pub parent: BoneParent,
    pub children: Vec<u8>,
    pub data: [u8; 63],
    pub bind_matrix: Mat4,
    pub inv_bind_matrix: Mat4,
}

impl Bone {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl ArchetypeImpl for Skeleton {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::YetiIOError> {
        let mut cursor = Cursor::new(buf);
        self.version = cursor.read_u8()?;
        self.num_bones = cursor.read_u8()?;
        self.unk_01 = cursor.read_u8()?;

        let mut bones: Vec<Bone> = Vec::with_capacity(self.num_bones as usize);
        for _ in 0..self.num_bones as usize {

            let mut bone = Bone {
                name: String::new(),
                unk_01: [0; 4],
                parent: Option::<u8>::None.into(),
                data: [0; 63],
                children: Vec::new(),
                bind_matrix: Mat4::IDENTITY,
                inv_bind_matrix: Mat4::IDENTITY,
            };
            
            cursor.read(&mut bone.unk_01)?;
            bone.parent.0 = match cursor.read_u8()? {
                255 => None,
                v => Some(v)
            };
            cursor.read(&mut bone.data)?;

            bone.bind_matrix = read_mat4(&mut cursor)?;
            bone.inv_bind_matrix = read_mat4(&mut cursor)?;

            bones.push(bone);
        }
        
        let mut strbuf: Vec<u8> = Vec::new();
        for i in 0..self.num_bones as usize {
            let len = cursor.read_u8()?;
            strbuf.clear();

            for _ in 0..len - 1 {
                strbuf.push(cursor.read_u8()?);
            }
            cursor.read_u8()?; // null terminator

            bones[i].name = String::from_utf8(strbuf.clone())?;
        }

        for i in 0..self.num_bones {
            let b = &bones[i as usize];
            if let Some(idx) = b.parent.0 {
                bones[idx as usize].children.push(i);
            }
        }

        self.bones = bones;

        Ok(())
    }

    fn unload(&mut self) {
        *self = Self::default()
    }
}