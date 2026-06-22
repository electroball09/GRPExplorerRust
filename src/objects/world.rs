use std::io::Read;

use super::*;

#[derive(Default)]
pub struct World {

}

impl ArchetypeImpl for World {
    fn load_from_buf(&mut self, _buf: &[u8]) -> Result<(), YetiIOError> {
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}

#[derive(Default)]
pub struct Rsf {
    pub entries: Vec<RsfEntry>,
}

pub struct RsfEntry {
    pub data: String,
}

impl ArchetypeImpl for Rsf {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), YetiIOError> {
        let mut cursor = Cursor::new(buf);

        let num = cursor.read_u32::<LittleEndian>()?;

        let mut entries = Vec::new();
        for _ in 0..num {
            let data_size = cursor.read_u32::<LittleEndian>()?;
            let mut data = vec![0; data_size as usize];
            cursor.read_exact(&mut data)?;

            cursor.read_u8()?; // null terminator

            let string = String::from_utf8(data)?;
            //log::info!("{}", &string);

            let entry = RsfEntry {
                data: string
            };

            if entry.data != "Creation default" {
                return Err(format!("weird rsf entry: {}", &entry.data).into());
            }

            entries.push(entry);
        }

        self.entries = entries;

        Ok(())
    }

    fn unload(&mut self) {
        *self = Default::default();
    }
}