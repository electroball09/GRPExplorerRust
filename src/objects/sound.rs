use super::ArchetypeImpl;
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{Cursor, Read};
use crate::util::read_nul_term_string;

#[derive(Default)]
pub struct SoundBank {
    pub snk_type: SnkType,
    pub bin_name: String,
    pub entries: Vec<SnkEntry>,
}

pub enum SnkType {
    Unknown(u8),
    Type0,
    Type1(u32),
    Type2,
    Type3(u32),
    Type8,
}

impl Default for SnkType {
    fn default() -> Self {
        SnkType::Unknown(0)
    }
}

pub struct SnkEntry {
    pub id: u8,
    pub name: String,
    pub unk00: u8,
    pub unk01: u8,
    pub unk02: u8,
    pub unk03: u8,
    pub unk04: u8,
    pub unk05: u8,
    pub unk06: u8,
    pub unk07: u8,
    pub unk08: u8,
    pub unk09: u8,

}

impl ArchetypeImpl for SoundBank {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::YetiIOError> {
        let mut cursor = Cursor::new(buf);

        self.snk_type = match cursor.read_u8()? {
            0 => SnkType::Type0,
            1 => SnkType::Type1(cursor.read_u32::<LittleEndian>()?),
            2 => SnkType::Type2,
            3 => SnkType::Type3(cursor.read_u32::<LittleEndian>()?),
            8 => SnkType::Type8,
            v => {
                return Err(format!("unknown snk type: {:#04X}", v).into());
            }
        };
        
        let mut buf: [u8; 32] = [0; 32];
        cursor.read(&mut buf)?;
        let v: Vec<u8> = buf.iter().map(|b| *b).take_while(|b| *b != 0).collect();
        self.bin_name = String::from_utf8(v)?;

        let mut num_snk = cursor.read_u8()?;
        self.entries = Vec::with_capacity(num_snk.into());
        while num_snk > 0 {
            self.entries.push(SnkEntry {
                id: cursor.read_u8()?,
                name: read_nul_term_string(&mut cursor)?,
                unk00: cursor.read_u8()?,
                unk01: cursor.read_u8()?,
                unk02: cursor.read_u8()?,
                unk03: cursor.read_u8()?,
                unk04: cursor.read_u8()?,
                unk05: cursor.read_u8()?,
                unk06: cursor.read_u8()?,
                unk07: cursor.read_u8()?,
                unk08: cursor.read_u8()?,
                unk09: cursor.read_u8()?,
            });

            num_snk -= 1;
        }

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}