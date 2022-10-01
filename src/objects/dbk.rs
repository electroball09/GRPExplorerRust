use std::io::Cursor;
use byteorder::ReadBytesExt;

#[derive(Default)]
pub struct DynamicBank {
    pub bank_id: u8,
    pub num_bank_entries: u8,
    pub the_rest_of_the_data: Vec<u8>
}

impl DynamicBank {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let mut cursor = Cursor::new(buf);

        self.bank_id = cursor.read_u8().unwrap();
        self.num_bank_entries = cursor.read_u8().unwrap();
        self.the_rest_of_the_data = buf[3..].to_vec();
        Ok(())
    }

    pub fn unload(&mut self) {
        self.bank_id = 0;
        self.num_bank_entries = 0;
        self.the_rest_of_the_data.clear();
        self.the_rest_of_the_data.shrink_to(1);
    }
}

