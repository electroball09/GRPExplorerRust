use crate::objects::*;
use super::*;
use glam::Vec3;
use gltf_json as json;

pub fn gltf_msh<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Mesh>> {
    gltf_export_init!(ct);

    let msd_key = ct.bf.object_table[&ct.key].references[0];

    let msh = match &ct.bf.object_table[&ct.key].archetype {
        ObjectArchetype::MeshMetadata(msh) => msh,
        _ => panic!("wrong object type!")
    };
    let msh_name = ct.bf.file_table[&ct.key].get_name().to_string();

    let msd = match &ct.bf.object_table[&msd_key].archetype {
        ObjectArchetype::MeshData(msd) => msd,
        _ => panic!("wrong object type!")
    };
    let _msd_name = ct.bf.file_table[&msd_key].get_name().to_string();

    //let sub_context = std::mem::take(&mut ct.sub_context);
    // let _colors = sub_context.as_ref().and_then(|sc| {
    //     Some(&sc.vertex_colors)
    // });

    let mut prims = Vec::new();
    let mut meshes = Vec::new();

    for idx in 0..msh.submeshes.len() {
        let submesh = &msh.submeshes[idx];

        let vertex_range = (submesh.vtx_start as usize)..(submesh.vtx_start as usize + submesh.vtx_num as usize);

        let vertex_start_u32 = submesh.vtx_start as u32;
        
        let build = GltfPrimitiveBuild {
            //pos_pre_transformed: Box::new(msd.vertex_data.pos.iter().cloned()),
            pos: Box::new(msd.vertex_data.pos[vertex_range.clone()].iter().map(|v| Vec3::new(-v.x, v.z, v.y))),
            indices: Box::new(msd.faces[(submesh.face_start / 3) as usize..(submesh.face_start / 3) as usize + submesh.face_num as usize]
                .iter().flat_map(|face| [face.f0 - vertex_start_u32, face.f1 - vertex_start_u32, face.f2 - vertex_start_u32])),
            uv0: match msd.vertex_data.uv0.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.uv0[vertex_range.clone()].iter().cloned()))
            },
            uv1: match msd.vertex_data.uv1.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.uv1[vertex_range.clone()].iter().cloned()))
            },
            tangents: match msd.vertex_data.tangents.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.tangents[vertex_range.clone()].iter().cloned()))
            },
            normals: match msd.vertex_data.normals.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.normals[vertex_range.clone()].iter().cloned()))
            },
            // never got vertex colors to work
            colors:None, // colors.map(|v| &**v)
            weights: match msd.vertex_data.weights.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.weights[vertex_range.clone()].iter().map(|weights| { 
                    weights.map(|w| {
                        // if the weight is zero we shouldn't try to index into bone palette
                        if submesh.bone_palette.len() > 0 && w.weight > 0.0 {
                            (submesh.bone_palette[w.bone as usize], w.weight)
                        } else { (0, 0.0) }
                    }) 
                })))
            },
            material: Some(submesh.material_index),
        };

        let primitive = write_primitive(ct, build);

        if ct.options.export_submeshes_individually {
            let mesh = ct.root.push(json::Mesh {
                extensions: Default::default(),
                extras: Default::default(),
                name: Some(format!("{:#010X} {} submesh{}", ct.key, msh_name, idx)),
                primitives: vec![primitive],
                weights: None
            });

            meshes.push(mesh);
        } else {
            prims.push(primitive);
        }
    };
    
    if ct.options.export_submeshes_individually {
        for mesh in &meshes {
            insert_cache!(ct, &ct.key, mesh);
        }

        meshes
    } else {
        let mesh = ct.root.push(json::Mesh {
            extensions: Default::default(),
            extras: Default::default(),
            name: Some(format!("{:#010X} {}", ct.key, msh_name)),
            primitives: prims,
            weights: None
        });

        insert_cache!(ct, &ct.key, mesh);

        vec![mesh]
    }
}