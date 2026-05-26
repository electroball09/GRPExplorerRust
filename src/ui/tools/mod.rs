use eframe::egui;

mod file_diff_tool; pub use file_diff_tool::*;
mod ini_editor; pub use ini_editor::*;

pub trait Tool {
    fn create(id: u32) -> Self where Self: Sized;
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> bool;
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct ExplorerToolId {
    pub id: u32,
    pub name: String
}

impl ExplorerToolId {
    pub fn new(name: &str, id: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

impl From<ExplorerToolId> for eframe::egui::Id {
    fn from(value: ExplorerToolId) -> Self {
        Self::new(value)
    }
}

impl From<&ExplorerToolId> for eframe::egui::Id {
    fn from(value: &ExplorerToolId) -> Self {
        Self::new(value)
    }
}