use super::search_view::SearchView;
use super::tools::ToolsView;
use super::{View, bf_metadata_view::BigfileMetadataView};
use super::file_tree_view::FileTreeView;
use super::super::BfRef;

pub struct SidePanelView {
    bigfile: BfRef,
    ft_view: FileTreeView,
    bf_view: BigfileMetadataView,
    tl_view: ToolsView,
    sr_view: SearchView,
    state: SidePanelViewState
}

enum SidePanelViewState {
    FileTree,
    BfMetadata,
    Tools,
    Search
}

impl SidePanelView {
    pub fn new(bf: BfRef) -> Self {
        Self {
            bigfile: bf.clone(),
            ft_view: FileTreeView::new(bf.clone()),
            bf_view: BigfileMetadataView::new(bf.clone()),
            tl_view: ToolsView::new(bf.clone()),
            sr_view: SearchView::new(bf.clone()),
            state: SidePanelViewState::FileTree
        }
    }

    pub fn should_open_new_tab(&self) -> Option<u32> {
        if let Some(key) = self.ft_view.did_click_file() {
            Some(key)
        } else if let Some(key) = self.sr_view.did_click_file() {
            Some(key)
        } else {
            None
        }
    }
}

impl View for SidePanelView {
    fn set_bigfile(&mut self, bf: crate::ui::BfRef) {
        self.bigfile = bf;
        if let Some(bf) = &self.bigfile {
            self.ft_view.set_bigfile(Some(bf.clone()));
            self.bf_view.set_bigfile(Some(bf.clone()));
            self.tl_view.set_bigfile(Some(bf.clone()));
            self.sr_view.set_bigfile(Some(bf.clone()));
        }
    }

    fn draw(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let None = self.bigfile { return; }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.state, SidePanelViewState::FileTree), "File Tree").clicked() {
                    self.state = SidePanelViewState::FileTree;
                }
                ui.separator();
                if ui.selectable_label(matches!(self.state, SidePanelViewState::Search), "Search").clicked() {
                    self.state = SidePanelViewState::Search;
                }
                ui.separator();
                if ui.selectable_label(matches!(self.state, SidePanelViewState::BfMetadata), "Metadata").clicked() {
                    self.state = SidePanelViewState::BfMetadata;
                }
                ui.separator();
                if ui.selectable_label(matches!(self.state, SidePanelViewState::Tools), "Tools").clicked() {
                    self.state = SidePanelViewState::Tools;
                }
            });
            ui.separator();
            ui.add_space(4.0);

            match self.state {
                SidePanelViewState::FileTree => self.ft_view.draw(ui, ctx),
                SidePanelViewState::BfMetadata => self.bf_view.draw(ui, ctx),
                SidePanelViewState::Tools => self.tl_view.draw(ui, ctx),
                SidePanelViewState::Search => self.sr_view.draw(ui, ctx)
            }
        });

    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.ft_view.settings_menu(ui, ctx);
        self.bf_view.settings_menu(ui, ctx);
        self.tl_view.settings_menu(ui, ctx);
        self.sr_view.settings_menu(ui, ctx);
    }
}