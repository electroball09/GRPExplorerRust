use crate::objects::*;
use super::*;
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

    let mut prims = Vec::new();

    let sub_context = std::mem::take(&mut ct.sub_context);
    let _colors = sub_context.as_ref().and_then(|sc| {
        Some(&sc.vertex_colors)
    });

    for idx in 0..msh.submeshes.len() {
        let submesh = &msh.submeshes[idx];
        
        let build = GltfPrimitiveBuild {
            pos_pre_transformed: Box::new(msd.vertex_data.pos.iter().cloned()),
            indices: Box::new(msd.faces[(submesh.face_start / 3) as usize..(submesh.face_start / 3) as usize + submesh.face_num as usize].iter().flat_map(|face| [face.f0, face.f1, face.f2])),
            uv0: match msd.vertex_data.uv0.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.uv0.iter().cloned()))
            },
            uv1: match msd.vertex_data.uv1.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.uv1.iter().cloned()))
            },
            tangents: match msd.vertex_data.tangents.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.tangents.iter().cloned()))
            },
            normals: match msd.vertex_data.normals.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.normals.iter().cloned()))
            },
            // never got vertex colors to work
            colors:None, // colors.map(|v| &**v)
            weights: match msd.vertex_data.weights.len() {
                0 => None,
                _ => Some(Box::new(msd.vertex_data.weights.iter().map(|weights| weights.map(|w| (w.bone, w.weight)))))
            }
        };

        let primitive = write_primitive(ct, build);

        prims.push(primitive);
    };
    
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