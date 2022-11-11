use std::io::Cursor;
use byteorder::ReadBytesExt;
use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct DynamicBank {
    pub bank_id: u8,
    pub num_bank_entries: u8,
    pub the_rest_of_the_data: Vec<u8>
}

impl ArchetypeImpl for DynamicBank {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);

        self.bank_id = cursor.read_u8()?;
        self.num_bank_entries = cursor.read_u8()?;
        if self.num_bank_entries > 0 {
            self.the_rest_of_the_data = buf[3..].to_vec();
        }
        
        Ok(())
    }

    fn unload(&mut self) {
        let _ = std::mem::take(self);
    }
}

