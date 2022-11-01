use std::{io::{Read, Cursor}, string::FromUtf8Error};

use byteorder::{ReadBytesExt, LittleEndian};
use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct YetiIni {
    pub entries: Vec<IniEntry>
}

pub enum IniEntry {
    Invalid,
    Int(String, u32),
    AssetKey(String, u32)
}

impl YetiIni {
    fn load_from_reader(&self, reader: &mut impl Read) -> Result<Vec<IniEntry>, LoadError> {
        let mut entries: Vec<IniEntry> = Vec::new();
        let num_entries = reader.read_u32::<LittleEndian>()?;
        let mut i = 0;
        while i < num_entries {
            let mut v: Vec<u8> = Vec::new();
            let mut byte = reader.read_u8()?;
            while byte != 0 {
                v.push(byte);
                byte = reader.read_u8()?;
            }
            let entry_type = reader.read_u8()?;
            let value = reader.read_u32::<LittleEndian>()?;

            let key = String::from_utf8(v)?;
            let value = match entry_type {
                0 => IniEntry::Int(key, value),
                1 => IniEntry::AssetKey(key, value),
                _ => IniEntry::Invalid
            };

            entries.push(value);

            i += 1;
        }

        Ok(entries)
    }

}

impl ArchetypeImpl for YetiIni {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);
        self.entries = self.load_from_reader(&mut cursor)?;
        Ok(())
    }

    fn unload(&mut self) {
        self.entries.clear();
        self.entries.shrink_to_fit();
    }
}