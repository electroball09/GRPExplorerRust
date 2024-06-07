use super::ArchetypeImpl;
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{Cursor, Read};
use crate::util::read_nul_term_string;

#[derive(Default)]
pub struct SoundBank {
    pub numbers: Vec<u32>,
    pub name: String,
    pub entries: Vec<SnkEntry>,
}

pub struct SnkEntry {
    pub id: u8,
    pub name: String,
    pub m_offset: u32,
    pub m_len: u16,
    pub m_idk: u32
}

impl ArchetypeImpl for SoundBank {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), super::LoadError> {
        let mut cursor = Cursor::new(buf);

        let mut ns: Vec<u32> = vec![0; cursor.read_u8()?.into()];
        for v in ns.iter_mut() {
            *v = cursor.read_u32::<LittleEndian>()?;
        }
        self.numbers = ns;
        
        let mut buf: [u8; 32] = [0; 32];
        cursor.read(&mut buf)?;
        let v: Vec<u8> = buf.iter().map(|b| *b).take_while(|b| *b != 0).collect();
        self.name = String::from_utf8(v)?;

        let mut num_snk = cursor.read_u8()?;
        self.entries = Vec::with_capacity(num_snk.into());
        while num_snk > 0 {
            self.entries.push(SnkEntry {
                id: cursor.read_u8()?,
                name: read_nul_term_string(&mut cursor)?,
                m_offset: cursor.read_u32::<LittleEndian>()?,
                m_len: cursor.read_u16::<LittleEndian>()?,
                m_idk: cursor.read_u32::<LittleEndian>()?
            });

            num_snk -= 1;
        }

        Ok(())
    }

    fn unload(&mut self) {
        self.entries.shrink_to(0);
        self.numbers.shrink_to(0);
    }
}