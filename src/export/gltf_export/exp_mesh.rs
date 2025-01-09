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
    let colors = std::mem::take(&mut ct.sub_context.vertex_colors);

    for idx in 0..msh.submeshes.len() {
        let submesh = &msh.submeshes[idx];
        
        let primitive = write_primitive(ct, GltfPrimitiveBuild {
            pos_pre_transformed: &msd.vertex_data.pos,
            indices: &msd.faces[(submesh.face_start / 3) as usize..(submesh.face_start / 3) as usize + submesh.face_num as usize].iter().flat_map(|face| [face.f0, face.f1, face.f2]).collect::<Vec<u32>>(),
            uv0: match msd.vertex_data.uv0.len() {
                0 => None,
                _ => Some(&msd.vertex_data.uv0)
            },
            uv1: match msd.vertex_data.uv1.len() {
                0 => None,
                _ => Some(&msd.vertex_data.uv1)
            },
            colors: colors.as_deref()
        });

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