use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use super::{ArchetypeImpl, LoadError};
use crate::util::*;

#[derive(Default)]
pub struct EditableParamsList {
    pub names_list: Vec<String>
}

impl ArchetypeImpl for EditableParamsList {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);

        let num_entries = cursor.read_u32::<LittleEndian>()?;
        self.names_list = Vec::with_capacity(num_entries as usize);
        let mut i = 0;
        while i < num_entries {
            self.names_list.push(read_nul_term_string(&mut cursor)?);
            i += 1;
        };

        Ok(())
    }

    fn unload(&mut self) {
        self.names_list.clear();
        self.names_list.shrink_to(0);
    }
}