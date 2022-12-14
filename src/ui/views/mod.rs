pub mod file_tree_view;
pub mod editor_tabs_view;
pub mod side_panel;
pub mod bf_metadata_view;
pub mod search_view;
pub mod tools;

pub trait View {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn settings_menu(&mut self, _ui: &mut egui::Ui, _ctx: &egui::Context) { }
    fn set_bigfile(&mut self, bf: super::BfRef);
}