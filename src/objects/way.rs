use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::objects::ArchetypeImpl;

#[derive(Default, Debug)]
pub struct Way {
    pub unk_dat01: [u8; 13],
    pub num_way_gaos: u16,
    pub way_datas: Vec<[u8; 10]>
}

impl ArchetypeImpl for Way {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), crate::bigfile::YetiIOError> {
        let mut cursor = Cursor::new(buf);

        cursor.read_exact(&mut self.unk_dat01)?;

        self.num_way_gaos = cursor.read_u16::<LittleEndian>()?;

        for _ in 0..self.num_way_gaos {
            let mut data = [0; 10];
            cursor.read_exact(&mut data)?;
            self.way_datas.push(data);
        }

        Ok(())
    }

    fn unload(&mut self) {
        self.way_datas.clear();
    }
}