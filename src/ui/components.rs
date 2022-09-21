use egui::Ui;
use id_tree::NodeId;

use crate::bigfile::{*, io::BigfileIO};

pub fn draw_file_tree(bf: &Bigfile, ui: &mut Ui, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    fn draw_folder(bf: &Bigfile, node_id: &NodeId, ctx: &eframe::egui::Context, ui: &mut Ui) {
        let folder = bf.node_id_to_folder(node_id);
        let rsp = ui.collapsing(folder.get_name(), |ui| {
            for node_id in bf.tree.get(node_id).unwrap().children().iter() {
                draw_folder(bf, node_id, ctx, ui);
            }
            match bf.file_list_map.get(&bf.node_id_to_folder(node_id).idx) {
                Some(v) => {
                    for key in v.iter() {
                        ui.label(bf.file_table[key].get_name());
                    }
                },
                None => return
            }
        }).header_response.on_hover_ui_at_pointer(|ui| {
            draw_folder_debug_label(folder, ui, ctx);
        });
    }

    draw_folder(bf, &bf.node_id_map[&0].0, ctx, ui);
}

pub fn draw_bigfile(bf: &Bigfile, ui: &mut Ui, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    draw_file_tree(bf, ui, ctx, frame);
}

pub fn draw_folder_debug_label(folder: &crate::FolderEntry, ui: &mut Ui, ctx: &eframe::egui::Context) {
    ui.label(format!("idx: {}\nparent_folder: {}\n first_child: {}\n unk03: {}\n unk05: {}", folder.idx, folder.parent_folder, folder.first_child, folder.unk03, folder.unk05));
}