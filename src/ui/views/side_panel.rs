use super::search_view::SearchView;
use super::tools::ToolsView;
use super::{View, bf_metadata_view::BigfileMetadataView};
use super::file_tree_view::FileTreeView;
use crate::egui as egui;
use crate::ui::AppContext;

pub struct SidePanelView {
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
    pub fn new() -> Self {
        Self {
            ft_view: FileTreeView::new(),
            bf_view: BigfileMetadataView::new(),
            tl_view: ToolsView::new(),
            sr_view: SearchView::new(),
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
    fn draw<'a>(&mut self, ui: &mut egui::Ui, app: &'a mut AppContext<'a>) {
        if let None = app.bigfile { return; }

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
                SidePanelViewState::FileTree    => self.ft_view.draw(ui, app),
                SidePanelViewState::BfMetadata  => self.bf_view.draw(ui, app),
                SidePanelViewState::Tools       => self.tl_view.draw(ui, app),
                SidePanelViewState::Search      => self.sr_view.draw(ui, app)
            }
        });

    }

    fn settings_menu(&mut self, ui: &mut egui::Ui, app: &mut AppContext) {
        self.ft_view.settings_menu(ui, app);
        self.bf_view.settings_menu(ui, app);
        self.tl_view.settings_menu(ui, app);
        self.sr_view.settings_menu(ui, app);
    }
}