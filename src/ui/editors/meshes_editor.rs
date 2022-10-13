use crate::objects::ObjectArchetype;
use crate::export::*;
use super::EditorResponse;



pub struct MeshDataEditor;

impl super::EditorImpl for MeshDataEditor {
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
                if let Some(path) = pick_exp_path(&obj, ".obj") {
                    exp_msd_as_obj(path, &msd);
                }
            }
        }
        
        EditorResponse::default()
    }
}