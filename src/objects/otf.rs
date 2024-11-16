use std::borrow::Cow;

use crate::egui as egui;
use super::{ArchetypeImpl, LoadError};

pub struct Otf {
    pub font: Option<egui::FontData>,
}

impl ArchetypeImpl for Otf {
    fn load_from_buf(&mut self, buf: &[u8]) -> Result<(), LoadError> {
        let mut v: Vec<u8> = vec![0; buf.len()];
        v.copy_from_slice(buf);

        let cow = Cow::Owned(v);

        self.font = Some(egui::FontData {
            font: cow,
            index: 0,
            tweak: egui::FontTweak::default()
        });

        Ok(())
    }

    fn unload(&mut self) {
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