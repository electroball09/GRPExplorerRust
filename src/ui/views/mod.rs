pub mod file_tree_view;
pub mod editor_tabs_view;
pub mod side_panel;
pub mod bf_metadata_view;
pub mod search_view;
pub mod tools;

use super::AppContext;

pub trait View {
    fn draw<'a>(&mut self, ui: &mut egui::Ui, app: &'a mut AppContext<'a>);
    fn settings_menu(&mut self, _ui: &mut egui::Ui, _app: &mut AppContext) { }
}