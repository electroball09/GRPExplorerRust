use super::ArchetypeImpl;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Default)]
pub struct Skeleton {
    pub version: u8,
    pub num_bones: u8,
    pub unk_01: u8,
    pub bones: Vec<Bone>,
}

pub struct Bone {
    name: String,
    pub data: [u8; 196],
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
        let mut i = 0;
        while i < self.num_bones {
            let mut bone = Bone {
                name: String::new(),
                data: [0; 196],
            };

            cursor.read(&mut bone.data)?;

            v.push(bone);
            i += 1;
        }

        let mut i = 0;
        while i < self.num_bones {
            let len = cursor.read_u8()?;
            let mut strbuf: Vec<u8> = Vec::with_capacity(len as usize);
            let mut byte = cursor.read_u8()?;
            while byte != 0 {
                strbuf.push(byte);
                byte = cursor.read_u8()?;
            }
            v[i as usize].name = String::from_utf8(strbuf)?;
            i += 1;
        }

        self.bones = v;

        Ok(())
    }

    fn unload(&mut self) {
        *self = Self::default()
    }
}