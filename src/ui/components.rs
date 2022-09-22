use egui::Ui;
use id_tree::NodeId;

use crate::bigfile::{*, io::BigfileIO};

pub fn draw_file_tree(bf: &Bigfile, ui: &mut Ui, ctx: &eframe::egui::Context, debug_folders: bool, debug_files: bool) {
    fn draw_folder(bf: &Bigfile, node_id: &NodeId, ctx: &eframe::egui::Context, ui: &mut Ui, debug_folders: bool, debug_files: bool) {
        let folder = bf.node_id_to_folder(node_id);
        let rsp = ui.collapsing(folder.get_name(), |ui| {
            for node_id in bf.tree.get(node_id).unwrap().children().iter() {
                draw_folder(bf, node_id, ctx, ui, debug_folders, debug_files);
            }
            match bf.file_list_map.get(&bf.node_id_to_folder(node_id).idx) {
                Some(v) => {
                    for key in v.iter() {
                        draw_file_button(&bf.file_table[key], ui, ctx, debug_files);
                    }
                },
                None => return
            }
        });
        if debug_folders {
            rsp.header_response.on_hover_ui_at_pointer(|ui| {
                ui.label(format!("idx: {:#06X}\nparent_folder: {:#06X}\n first_child: {:#06X}\n unk03: {:#06X}\n unk05: {:#06X}", 
                                        folder.idx, folder.parent_folder, folder.first_child, folder.unk03, folder.unk05));
            });
        }
    }

    draw_folder(bf, &bf.node_id_map[&0].0, ctx, ui, debug_folders, debug_files);
}

pub fn draw_file_button(file: &crate::FileEntry, ui: &mut Ui, ctx: &eframe::egui::Context, debug: bool) {
    ui.horizontal(|ui| {
        ui.label(format!("{:?} -", file.object_type));
        let btn = ui.button(file.get_name());
        if btn.clicked() {
            println!("clicked file {}", file.get_name_ext());
        };

        if debug {
            btn.on_hover_ui_at_pointer(|ui| {
                ui.label(format!("key: {:#010X}\noffset: {}\n flags: {:#010X}\n zip: {}", 
                                        file.key, file.offset, file.flags, file.zip));
            });
        }
    });
}