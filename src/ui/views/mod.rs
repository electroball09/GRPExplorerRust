pub mod file_tree_view;
pub mod editor_tabs_view;
pub mod side_panel;

pub trait View {
    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn settings_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn set_bigfile(&mut self, bf: super::BfRef);
}