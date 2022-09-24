use std::{collections::HashMap, io::{Read, Seek, Cursor}};

use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Default)]
pub struct YetiIni {
    pub entries: HashMap<String, IniEntry>
}

pub enum IniEntry {
    Invalid,
    Int(u32),
    AssetKey(u32)
}

impl YetiIni {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let mut cursor = Cursor::new(buf);
        self.load_from_reader(&mut cursor)
    }

    fn load_from_reader(&mut self, reader: &mut impl Read) -> Result<(), String> {
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
                0 => IniEntry::Int(value),
                1 => IniEntry::AssetKey(value),
                _ => IniEntry::Invalid
            };

            self.entries.insert(key, value);

            i += 1;
        }

        Ok(())
    }
}