use super::View;
use super::file_tree_view::FileTreeView;
use super::super::BfRef;

pub struct SidePanelView {
    bigfile: BfRef,
    ft_view: FileTreeView,
}

impl SidePanelView {
    pub fn new(bf: BfRef) -> Self {
        Self {
            bigfile: bf.clone(),
            ft_view: FileTreeView::new(bf.clone())
        }
    }

    pub fn should_open_new_tab(&self) -> Option<u32> {
        self.ft_view.did_click_file()
    }
}

impl View for SidePanelView {
    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf;
        self.ft_view.set_bigfile(self.bigfile.clone());
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.ft_view.draw(ui, ctx);
    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.ft_view.settings_menu(ui, ctx);
    }
}