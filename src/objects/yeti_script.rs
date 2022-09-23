use super::ObjectArchetype;



#[derive(Default)]
pub struct YetiScript {


    pub buffer: Vec<u8>
}

impl YetiScript {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        self.buffer = buf.to_vec();
        Ok(())
    }
}