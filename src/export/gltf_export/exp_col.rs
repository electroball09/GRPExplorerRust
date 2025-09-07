use crate::objects::ObjectArchetype;

use super::*;
use serde_json::json;
use json::validation::Checked::Valid;

pub fn gltf_cot<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    gltf_export_init!(ct);

    let mut nodes = Vec::new();
    for key in ct.bf.object_table[&ct.key].references.iter() {
        if ct.bf.is_key_valid(*key) {
            if ct.bf.file_table[key].object_type.is_col() {
                ct_with_key!(ct, *key, {
                    nodes.append(&mut gltf_col(ct));
                });
            }
        }
    }

    nodes
}

pub fn gltf_col<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    if !ct.options.export_collision {
        return vec![];
    }

    gltf_export_init!(ct);

    let col_name = ct.bf.file_table[&ct.key].get_name_ext().to_string();
    let col = match &ct.bf.object_table[&ct.key].archetype {
        ObjectArchetype::CollisionObject(col) => col,
        _ => panic!("wrong object type!")
    };

    let vtx_start = ct.cursor.position();
    for pos in &col.positions {
        ct.cursor.write_f32::<ENDIAN>(-pos.x).unwrap(); // invert x and swap y and z for gltf
        ct.cursor.write_f32::<ENDIAN>(pos.z).unwrap();
        ct.cursor.write_f32::<ENDIAN>(pos.y).unwrap();
    }
    let vbuf_len = ct.cursor.position() - vtx_start;

    let ind_start = ct.cursor.position();
    for ind in &col.indices {
        //ct.cursor.write_u16::<ENDIAN>(ind[0]).unwrap(); // no fscking clue what this does
        ct.cursor.write_u16::<ENDIAN>(ind[1]).unwrap();
        ct.cursor.write_u16::<ENDIAN>(ind[2]).unwrap();
        ct.cursor.write_u16::<ENDIAN>(ind[3]).unwrap();
    }
    let ind_len = ct.cursor.position() - ind_start;

    let vtx_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64::from(vbuf_len),
        byte_offset: Some(USize64::from(vtx_start)),
        byte_stride: Some(json::buffer::Stride(12)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer))
    });

    let ind_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64::from(ind_len),
        byte_offset: Some(USize64::from(ind_start)),
        byte_stride: Some(json::buffer::Stride(2)),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer))
    });

    let (min, max) = col.bounding_box();
    let min: &[f32] = &min.to_array();
    let max: &[f32] = &max.to_array();
    let pos_acc = ct.root.push(json::Accessor {
        buffer_view: Some(vtx_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(col.positions.len()),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(min)),
        max: Some(json::Value::from(max)),
        name: None,
        normalized: false,
        sparse: None
    });
    
    let ind_acc = ct.root.push(json::Accessor {
        buffer_view: Some(ind_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(col.indices.len() * 3),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::U16)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None
    });
        
    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), pos_acc);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(ind_acc),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None
    };
    
    let mesh = ct.root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(format!("{:#010X} {}", ct.key, col_name)),
        primitives: vec![primitive],
        weights: None
    });

    let name = format!("{:#010X} {}", ct.key, col_name);

    // TODO there has to be a flag somewhere in the file itself to determine this right??
    let col_type = {
        if name.contains("RT") {
            "complex"
        } else if name.contains("CN") {
            "simple"
        } else {
            ""
        }
    };

    let extras = Some(json!({
        "type": "collision",
        "collision_type": col_type
    }));

    let node = ct.root.push(json::Node {
        mesh: Some(mesh),
        name: Some(name),
        extras: extras.map(|v| serde_json::value::to_raw_value(&v).unwrap()),
        ..Default::default()
    });

    insert_cache!(ct, &ct.key, node);

    vec![node]
}