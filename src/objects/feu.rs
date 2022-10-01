use std::io::Cursor;
use std::io::Seek;
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Default)]
pub struct Feu {
    pub unk_01: u32,
    pub unk_02: u32,
    pub feu_refs: Vec<String>,
    pub feu_data: Vec<u8>
}

impl Feu {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let mut cursor = Cursor::new(buf);

        self.unk_01 = cursor.read_u32::<LittleEndian>().unwrap();
        self.unk_02 = cursor.read_u32::<LittleEndian>().unwrap();

        fn check_uef(buf: &[u8]) -> bool {
            buf[0] == b'U' &&
            buf[1] == b'E' &&
            buf[2] == b'F' &&
            buf[3] == 0x08
        }

        let mut cursor_pos = cursor.position() as usize;
        let mut found_uef = check_uef(&buf[cursor_pos..cursor_pos + 4]);
        let mut refs: Vec<String> = Vec::new();

        while !found_uef {
            let mut r = String::new();
            let mut b = cursor.read_u8().unwrap();
            while b != 0x00 {
                r.push(b as char);
                b = cursor.read_u8().unwrap();
            }
            refs.push(r);
            cursor_pos = cursor.position() as usize;
            found_uef = check_uef(&buf[cursor_pos..cursor_pos + 4]);
        }

        self.feu_refs = refs;

        let data_pos = cursor.position() as usize;
        self.feu_data = (&buf[data_pos..]).iter().map(|b| *b).collect();

        Ok(())
    }

    pub fn unload(&mut self) {
        self.feu_data = Vec::new();
        self.feu_refs = Vec::new();
    }
}