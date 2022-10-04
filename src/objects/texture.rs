use super::*;
use image::*;
use squish::*;

#[derive(Default, Debug)]
pub enum TextureFormat {
    #[default]
    Unknown,
    Gray,
    Dxt5,
    Dxt1,
    Bgra32,
    Rgba32,
}

#[derive(Default)]
pub struct TextureMetadata {
    pub width: u16,
    pub height: u16,
    pub format: TextureFormat,
    pub fmt_id: u8,
}

impl ArchetypeImpl for TextureMetadata {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        if buf.len() < 6 {
            return Ok(());
            //return Err(String::from("small metadata").into());
        }

        let mut bytes: [u8; 2] = [0; 2];
        bytes.copy_from_slice(&buf[4..6]);
        self.width = u16::from_le_bytes(bytes);
        bytes.copy_from_slice(&buf[6..8]);
        self.height = u16::from_le_bytes(bytes);

        self.fmt_id = buf[9];
        self.format = match self.fmt_id {
            0x0F => TextureFormat::Gray,
            0x0A => TextureFormat::Dxt5,
            0x0C => TextureFormat::Dxt5,
            0x08 => TextureFormat::Dxt1,
            0x09 => TextureFormat::Dxt1,
            0x04 => TextureFormat::Bgra32,
            0x05 => TextureFormat::Rgba32,
            _ => TextureFormat::Unknown
        };

        Ok(())
    }

    fn unload(&mut self) {
        
    }
}

#[derive(Default)]
pub struct TextureData {

}

impl ArchetypeImpl for TextureData {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        
        Ok(())
    }

    fn unload(&mut self) {
        
    }
}