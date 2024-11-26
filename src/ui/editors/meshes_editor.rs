use super::*;
use crate::objects::ObjectArchetype;
use crate::export::*;

pub struct MeshMetadataEditor;

impl EditorImpl for MeshMetadataEditor {
    fn draw(&mut self, key: u32, ui: &mut egui::Ui, ectx: &mut EditorContext) {
        let msd = if ui.button("Export to .glb").clicked() {
            let msd_key = ectx.bf.object_table.get(&key).unwrap().references[0];
            ectx.bf.load_file(msd_key).unwrap();
            match &ectx.bf.object_table.get(&msd_key).unwrap().archetype {
                ObjectArchetype::MeshData(ref msd) => Some((msd, msd_key)),
                _ => None
            }
        } else {
            None
        };

        if let ObjectArchetype::MeshMetadata(ref msh) = &ectx.bf.object_table.get(&key).unwrap().archetype {
            ui.label(format!("num submeshes: {}", msh.num_submeshes));
            ui.label(format!("version: {}", msh.version));
            ui.label(format!("unk_dat01: {}", msh.unk_dat01.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")));
            ui.label(format!("unk_dat02: {}", msh.unk_dat02.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")));
            for idx in 0..msh.submeshes.len() {
                let sb = &msh.submeshes[idx];
                ui.collapsing(format!("submesh {}", idx), |ui| {
                    ui.label(format!("vtx_start: {}", sb.vtx_start));
                    ui.label(format!("vtx_num: {}", sb.vtx_num));
                    ui.label(format!("calc vtx end: {}", sb.vtx_start + sb.vtx_num));
                    ui.label(format!("unk_01: {0} {0:#06X}", sb.unk_01));
                    ui.label(format!("unk_02: {0} {0:#06X}", sb.unk_02));
                    ui.label(format!("unk_03: {0} {0:#06X}", sb.unk_03));
                    ui.label(format!("unk_04: {0} {0:#06X}", sb.unk_04));
                    ui.label(format!("unk_05: {0} {0:#06X}", sb.unk_05));
                    ui.label(format!("unk_vec {}: {}", sb.unk_vec.len(), sb.unk_vec.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")));
                });
            }

            if let Some(msd) = msd {
                export_mesh_to_gltf(msh, msd.0);
            }
        }

        if let Some(msd) = msd {
            ectx.bf.unload_file(msd.1).unwrap();
        }
    }
}

pub struct MeshDataEditor;

impl EditorImpl for MeshDataEditor {
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