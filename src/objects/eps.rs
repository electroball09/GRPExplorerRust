use std::{io::{Cursor, Seek, SeekFrom}};
use byteorder::{LittleEndian, ReadBytesExt};
use super::{ArchetypeImpl, LoadError};
use crate::util::*;

#[derive(Default)]
pub struct EditableParamStruct {
    pub unk_01: u32,
    pub struct_data_len: u32,
    pub num_entries: u32,
    pub entries: Vec<StructEntry>
}

#[derive(Default)]
pub struct StructEntry {
    pub name: String,
    pub unk_01: u8,
    pub data_offset: u32,
}

impl ArchetypeImpl for EditableParamStruct {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);
        self.unk_01 = cursor.read_u32::<LittleEndian>()?;
        self.struct_data_len = cursor.read_u32::<LittleEndian>()?;
        self.num_entries = cursor.read_u32::<LittleEndian>()?;
        self.entries = Vec::with_capacity(self.num_entries as usize);

        let mut i = 0;
        while i < self.num_entries {
            let s = StructEntry {
                name: read_nul_term_string(&mut cursor)?,
                unk_01: cursor.read_u8()?,
                data_offset: cursor.read_u32::<LittleEndian>()?,
            };
            self.entries.push(s);
            i += 1;
        }
        Ok(())
    }
    
    fn unload(&mut self) {
        *self = EditableParamStruct
        {
            ..Default::default()
        };
    }
}