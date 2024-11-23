use super::*;
use crate::objects::ObjectArchetype;
use crate::export::*;

pub struct MeshDataEditor;

impl super::EditorImpl for MeshDataEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let obj = &ectx.bf.object_table.get(&key).unwrap();
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

            egui::ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                for v in 0..msd.num_vertices {
                    ui.collapsing(format!("vertex {}", v), |ui| {
                        let str: Vec<String> = msd.vertex_data.bufs[v as usize].iter().map(|b| format!("{:#04X}", b)).collect();
                        ui.label(format!("{:?}", str));
                        ui.label(format!("pos: {}", msd.vertex_data.pos[v as usize]));
                        ui.label(format!("uv0: {}", msd.vertex_data.uv0[v as usize]));
                    });
                }
            });
        }
    }
}