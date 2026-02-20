use crate::{objects::{ObjectArchetype}, util::transform_yeti_matrix};

use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;

pub fn gltf_ske<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Skin>> {
    gltf_export_init!(ct);

    let ObjectArchetype::Skeleton(ref skeleton) = ct.bf.object_table[&ct.key].archetype else {
        return vec![];   
    };
    
    // first push each bone to gltf, and track the index assigned for each
    // theoretically they should be in order starting at first assigned index but i can't promise that
    let mut index_map = HashMap::new();
    for (i, bone) in skeleton.bones.iter().enumerate() {
        index_map.insert(i, ct.root.push(json::Node {
            name: Some(bone.get_name().to_string()),
            matrix: Some(transform_yeti_matrix(&bone.bind_matrix).to_cols_array()),
            ..Default::default()
        }));
    };

    // now we set up the children for the gltf nodes
    for (i, bone) in skeleton.bones.iter().enumerate() {
        let node = &mut ct.root.nodes[index_map[&i].value()];
        node.children = Some(bone.children.iter().map(|&child_idx| index_map[&(child_idx as usize)]).collect());
    }

    while ct.cursor.position() % 4 != 0 {
        ct.cursor.write_u8(0).unwrap();
    }

    let matrix_start = ct.cursor.position();
    
    for bone in &skeleton.bones {
        let yeti_matrix = transform_yeti_matrix(&bone.inv_bind_matrix);
        
        for val in yeti_matrix.to_cols_array() {
            ct.cursor.write_f32::<ENDIAN>(val).unwrap();
        }
    }

    let maxtrix_len = ct.cursor.position() - matrix_start;

    let matrix_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64(maxtrix_len.into()),
        byte_offset: Some(matrix_start.into()),
        byte_stride: Some(json::buffer::Stride(64)),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        extensions: None,
        extras: None
    });

    let matrix_acc = ct.root.push(json::Accessor {
        buffer_view: Some(matrix_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(skeleton.bones.len()),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Mat4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None
    });

    let name = ct.bf.object_table[&ct.key].get_name();
    let skin = json::Skin {
        name: Some(name.to_string()),
        skeleton: Some(index_map[&0]),
        joints: index_map.into_values().collect(),
        extensions: None,
        extras: None,
        inverse_bind_matrices: Some(matrix_acc)
    };

    let skin = ct.root.push(skin);

    insert_cache!(ct, &ct.key, skin);

    vec![skin]
}