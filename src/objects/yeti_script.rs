use super::{ArchetypeImpl, LoadError};

#[derive(Default)]
pub struct YetiScript {


    pub buffer: Vec<u8>
}

impl ArchetypeImpl for YetiScript {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        self.buffer = buf.to_vec();
        Ok(())
    }

    fn unload(&mut self) {
        self.buffer.clear();
    }
}