use super::*;

#[derive(Default, Debug, Clone, Copy)]
pub enum TextureFormat {
    #[default]
    Unknown,
    Gray,
    Dxt5,
    Dxt1,
    Bgra32,
    Rgba32,
    UnusedX360_27,
    UnusedX360_28,
}

impl TextureFormat {
    pub fn from_id(id: u8) -> Self {
        match id {
            0x0F => TextureFormat::Gray,
            0x0A => TextureFormat::Dxt5,
            0x0C => TextureFormat::Dxt5,
            0x08 => TextureFormat::Dxt1,
            0x09 => TextureFormat::Dxt1,
            0x04 => TextureFormat::Bgra32,
            0x05 => TextureFormat::Rgba32,
            0x27 => TextureFormat::UnusedX360_27,
            0x28 => TextureFormat::UnusedX360_28,
            _ => TextureFormat::Unknown
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextureMetaType {
    None,
    Metadata(TextureMetadata),
    Passthrough,
}

pub struct TextureMetadataObject {
    pub meta: TextureMetaType,
}

impl Default for TextureMetadataObject {
    fn default() -> Self {
        Self { 
            meta: TextureMetaType::None
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct TextureMetadata {
    pub width: u16,
    pub height: u16,
    pub format: TextureFormat,
    pub fmt_id: u8,
}

impl ArchetypeImpl for TextureMetadataObject {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        if buf.len() < 6 {
            self.meta = TextureMetaType::Passthrough;
            return Ok(());
        }

        let mut meta = TextureMetadata::default();

        let mut bytes: [u8; 2] = [0; 2];
        bytes.copy_from_slice(&buf[4..6]);
        meta.width = u16::from_le_bytes(bytes);
        bytes.copy_from_slice(&buf[6..8]);
        meta.height = u16::from_le_bytes(bytes);

        meta.fmt_id = buf[9];
        meta.format = TextureFormat::from_id(meta.fmt_id);

        if let TextureFormat::Unknown = meta.format {
            return Err(format!("unknown texture format: {:#04X}", meta.fmt_id).into());
        }

        self.meta = TextureMetaType::Metadata(meta);

        Ok(())
    }

    fn unload(&mut self) {
        
    }
}

#[derive(Default)]
pub struct TextureData {
    pub unk_01: u32,
    pub fmt_id: u8,
    pub format: TextureFormat,
    pub unk_02: u16,
    pub unk_03: u8,
    pub texture_data: Vec<u8>,
}

impl ArchetypeImpl for TextureData {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut cursor = Cursor::new(buf);
        self.unk_01 = cursor.read_u32::<LittleEndian>()?;
        self.fmt_id = cursor.read_u8()?;
        self.format = TextureFormat::from_id(self.fmt_id);

        self.texture_data = buf[8..].to_vec();

        Ok(())
    }

    fn unload(&mut self) {
        self.texture_data.clear();
        self.texture_data.shrink_to(1);
    }
}