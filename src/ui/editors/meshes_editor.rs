use std::fs::File;
use std::io::Write;
use crate::objects::ObjectArchetype;

use super::EditorResponse;



pub struct MeshDataEditor;

impl super::Editor for MeshDataEditor {
    fn draw(obj: &mut crate::objects::YetiObject, ui: &mut egui::Ui, ctx: &egui::Context) -> super::EditorResponse {
        if let ObjectArchetype::MeshData(msd) = &obj.archetype {
            ui.label(format!("unk_01: {}", msd.unk_01));
            ui.label(format!("unk_02: {}", msd.unk_02));
            ui.label(format!("num vertices: {}", msd.num_vertices));
            ui.label(format!("num indices: {}", msd.num_indices));
            ui.label(format!("unk_03: {}", msd.unk_03));
            ui.label(format!("data offset: {}", msd.data_offset));
            ui.label(format!("old num submeshes: {}", msd.old_num_submeshes));
            ui.label(format!("old submesh size: {}", msd.old_submesh_size));
            ui.label(format!("num submeshes: {}", msd.num_submeshes));
            ui.label(format!("pivot offset: {}", msd.pivot_offset));
            ui.label(format!("uniform scale: {}", msd.uniform_scale));

            if ui.button("Export to .obj...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let path = format!("{}/{:#010X} {}.obj", path.to_str().unwrap(), obj.get_key(), obj.get_name());
                    let mut file = File::create(path).unwrap();

                    for vert in &msd.vertices {
                        write!(file, "v {} {} {}\n", vert.pos.x, vert.pos.y, vert.pos.z).unwrap(); // swap y and z for coordinate correctness
                    }

                    for face in &msd.faces {
                        write!(file, "f {} {} {}\n", face[0] + 1, face[1] + 1, face[2] + 1).unwrap();
                    }
                }
            }
        }
        
        EditorResponse::default()
    }
}