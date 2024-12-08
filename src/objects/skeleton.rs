use super::ArchetypeImpl;
use std::{fmt::Display, io::{Cursor, Read}};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Default)]
pub struct Skeleton {
    pub version: u8,
    pub num_bones: u8,
    pub unk_01: u8,
    pub bones: Vec<Bone>,
}

#[derive(Clone, Copy, Default)]
pub struct BoneParent(Option<u8>);

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
    pub data: [u8; 191],
}

impl Bone {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl ArchetypeImpl for Skeleton {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::LoadError> {
        let mut cursor = Cursor::new(buf);
        self.version = cursor.read_u8()?;
        self.num_bones = cursor.read_u8()?;
        self.unk_01 = cursor.read_u8()?;

        let mut v: Vec<Bone> = Vec::with_capacity(self.num_bones as usize);
        for _ in 0..self.num_bones as usize {
            let mut bone = Bone {
                name: String::new(),
                unk_01: [0; 4],
                parent: Option::<u8>::None.into(),
                data: [0; 191],
                children: Vec::new(),
            };
            
            cursor.read(&mut bone.unk_01)?;
            bone.parent.0 = match cursor.read_u8()? {
                255 => None,
                v => Some(v)
            };
            cursor.read(&mut bone.data)?;

            v.push(bone);
        }
        
        for i in 0..self.num_bones as usize {
            let len = cursor.read_u8()?;
            let mut strbuf: Vec<u8> = Vec::with_capacity(len as usize);
            for _ in 0..len {
                strbuf.push(cursor.read_u8()?);
            }
            v[i].name = String::from_utf8(strbuf)?;
        }

        for i in 0..self.num_bones {
            let b = &v[i as usize];
            if let Some(idx) = b.parent.0 {
                v[idx as usize].children.push(i);
            }
        }

        self.bones = v;

        Ok(())
    }

    fn unload(&mut self) {
        *self = Self::default()
    }
}