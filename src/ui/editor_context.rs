use crate::metadata::YKey;
use crate::Bigfile;
use crate::ggl::ShaderCache;
use eframe::egui;

pub enum EditorResponse {
    None,
    OpenNewTab(YKey),
    CloseTab(YKey),
    ExtractFile(YKey, String),
    GltfExport(YKey),
}

pub struct EditorContext<'a> {
    pub bf: &'a mut Bigfile,
    pub shader_cache: &'a mut ShaderCache,
    pub ctx: &'a egui::Context,
    pub responses: Vec<EditorResponse>,
}

impl <'a> EditorContext<'a> {
    pub fn new(bf: &'a mut Bigfile, shader_cache: &'a mut ShaderCache, ctx: &'a egui::Context) -> Self {
        Self {
            bf, shader_cache, ctx,
            responses: Vec::new(),
        }
    }

    pub fn respond(&mut self, response: EditorResponse) {
        self.responses.push(response);
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, EditorResponse> {
        self.responses.drain(..)
    }

    pub fn num_responses(&self) -> usize {
        self.responses.len()
    }
}