use crate::Bigfile;
use crate::ggl::ShaderCache;

pub enum EditorResponse {
    None,
    OpenNewTab(u32),
    CloseTab(u32),
    ExtractFile(u32, String),
    GltfExport(u32),
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