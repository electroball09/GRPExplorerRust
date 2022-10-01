use std::{collections::HashMap, io::{Read, Seek, Cursor}};

use byteorder::{ReadBytesExt, LittleEndian};

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
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let mut cursor = Cursor::new(buf);
        self.entries = self.load_from_reader(&mut cursor)?;
        Ok(())
    }

    fn load_from_reader(&self, reader: &mut impl Read) -> Result<Vec<IniEntry>, String> {
        let mut entries: Vec<IniEntry> = Vec::new();
        let num_entries = reader.read_u32::<LittleEndian>().unwrap();
        let mut i = 0;
        while i < num_entries {
            let mut v: Vec<u8> = Vec::new();
            let mut byte = reader.read_u8().unwrap();
            while byte != 0 {
                v.push(byte);
                byte = reader.read_u8().unwrap();
            }
            let entry_type = reader.read_u8().unwrap();
            let value = reader.read_u32::<LittleEndian>().unwrap();

            let key = String::from_utf8(v).unwrap();
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

    pub fn unload(&mut self) {
        self.entries.clear();
        self.entries.shrink_to_fit();
    }
}