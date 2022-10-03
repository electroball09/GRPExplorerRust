use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct YetiLayer {
    pub name: String,
}

impl ArchetypeImpl for YetiLayer {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let vec: Vec<u8> = buf.iter().skip(4).map(|b| *b).take_while(|b| *b != 0).collect(); 
        self.name = match String::from_utf8(vec) {
            Ok(name) => name,
            Err(error) => return Err(error.to_string().into())
        };
        Ok(())
    }

    fn unload(&mut self) {
        self.name = String::new()
    }
}