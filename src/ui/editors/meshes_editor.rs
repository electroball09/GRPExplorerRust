use log::info;

use super::*;
use crate::objects::ObjectArchetype;
use crate::export::*;
use crate::ui::util::format_bytes_to_hex_wrapped;

pub struct MeshMetadataEditor;

impl EditorImpl for MeshMetadataEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
        let msd = if ectx.bf.object_table.get(&key).unwrap().references.len() > 0 {
            Some(ectx.bf.object_table.get(&key).unwrap().references[0])
        } else { None };

        let do_export = ui.button("Export to .glb").clicked();

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
                    ui.label(format!("  -calc vtx end: {}", sb.vtx_start + sb.vtx_num));
                    ui.label(format!("face_start: {0} {0:#06X}", sb.face_start));
                    ui.label(format!("face_num: {0} {0:#06X}", sb.face_num));
                    ui.label(format!("  -calc face end: {}", sb.face_start + sb.face_num * 3));
                    ui.label(format_bytes_to_hex_wrapped(&sb.unk_dat01));
                    ui.label(format!("unk_vec {}: {}", sb.bone_palette.len(), sb.bone_palette.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")));
                });
            }

            if let Some(msd_key) = msd {
                if ui.button("Test bone palette theory").clicked() {
                    if let ObjectArchetype::MeshData(ref msd) = &ectx.bf.object_table.get(&msd_key).unwrap().archetype {
                        let mut passed = true;
                        for idx in 0..msh.submeshes.len() {
                            let submesh = &msh.submeshes[idx];
                            for vertex_index in submesh.vtx_start..(submesh.vtx_start + submesh.vtx_num) {
                                let weights = &msd.vertex_data.weights[vertex_index as usize];
                                for weight in weights {
                                    if weight.weight > 0.0 {
                                        let valid_bone = (weight.bone as usize) < submesh.bone_palette.len();
                                        if !valid_bone {
                                            info!("submesh {} vertex {} has invalid bone index {} (palette len: {})", idx, vertex_index, weight.bone, submesh.bone_palette.len());
                                            passed = false;
                                        }
                                    }
                                }
                            }
                        }
                        info!("test passed: {}", passed);
                    }
                }
            }
        }

        if do_export {
            ectx.respond(EditorResponse::GltfExport(key));
        }
    }
}

#[derive(Default, Debug)]
pub struct MeshDataEditor {
    show_vertices: bool,
}

impl EditorImpl for MeshDataEditor {
    fn draw(&mut self, key: YKey, ui: &mut egui::Ui, ectx: &mut EditorContext, _tctx: &EditorTabContext) {
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

            ui.checkbox(&mut self.show_vertices, "show vertices");

            if self.show_vertices {
                egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    for v in 0..msd.num_vertices as usize {
                        ui.collapsing(format!("vertex {}", v), |ui| {
                            ui.label(format_bytes_to_hex_wrapped(&msd.vertex_data.bufs[v]));
                            ui.label(format!("pos: {}", msd.vertex_data.pos[v]));
                            ui.label(format!("uv0: {}", msd.vertex_data.uv0[v]));
                            ui.label(format!("uv1: {}", msd.vertex_data.uv1[v]));
                            ui.label(format!("tan: {}", msd.vertex_data.tangents[v]));
                            ui.label(format!("nrm: {}", msd.vertex_data.normals[v]));
                            
                            ui.label("bones: ");
                            let bone = &msd.vertex_data.weights[v];
                            for b in 0..bone.len() {
                                ui.label(format!("  bone{}: {:?}", &b, bone[b]));
                            };
                        });
                    }
                });
            }
        }
    }
}