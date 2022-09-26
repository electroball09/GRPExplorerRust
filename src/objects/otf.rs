use std::borrow::Cow;

use egui::FontData;

pub struct Otf {
    pub font: Option<FontData>,
}

impl Otf {
    pub fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), String> {
        let mut v: Vec<u8> = vec![0; buf.len()];
        v.copy_from_slice(buf);

        let cow = Cow::Owned(v);

        self.font = Some(FontData {
            font: cow,
            index: 0,
            tweak: egui::FontTweak::default()
        });

        Ok(())
    }

    pub fn unload(&mut self) {
        self.font = None
    }
}

impl Default for Otf {
    fn default() -> Self {
        Otf {
            font: None
        }
    }
}