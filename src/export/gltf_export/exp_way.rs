use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;
use rgeometry::algorithms::polygonization::two_opt_moves;
use rgeometry::data::{Point, Polygon};

pub fn gltf_wal<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    let mut nodes = Vec::new();
    for key in &ct.bf.object_table[&ct.key].references {
        ct_with_key!(ct, *key, {
            nodes.append(&mut gltf_way(ct));
        });
    };

    nodes
}

pub fn gltf_way<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    let name = ct.bf.file_table[&ct.key].get_name_ext().to_string();

    let mut pos = Vec::new();
    let mut z = 9999999.0;
    for key in &ct.bf.object_table[&ct.key].references {
        if ct.bf.is_key_valid(*key) && ct.bf.file_table[key].object_type.is_gao() {
            let p = ct.bf.object_table[key].archetype.as_game_object().unwrap().position();
            pos.push(p);
            z = f32::min(z, p.z);
        }
    };

    let points: Vec<Point<f32>> = pos.iter().map(|v| Point::<f32>::new([v.x.into(), v.y.into()])).collect();
    let poly = Polygon::<f32>::new(points).expect("polygon error!");

    let vtx_start = ct.cursor.position();

    for p in &pos {
        ct.cursor.write_f32::<ENDIAN>(-p.x).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(z).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.y).expect("write error");
    }

    let vtx_len = ct.cursor.position() - vtx_start;
    let idx_start = ct.cursor.position();

    let mut num_idx = 0;
    for idx in poly.triangulate() {
        ct.cursor.write_u32::<ENDIAN>(idx.0.point_id().usize() as u32).expect("write error");
        ct.cursor.write_u32::<ENDIAN>(idx.1.point_id().usize() as u32).expect("write error");
        ct.cursor.write_u32::<ENDIAN>(idx.2.point_id().usize() as u32).expect("write error");
        num_idx += 3;
    }
    let num_idx = num_idx;

    let idx_len = ct.cursor.position() - idx_start;

    let vtx_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64(vtx_len.into()),
        byte_offset: Some(vtx_start.into()),
        byte_stride: Some(json::buffer::Stride(12)),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        extensions: None,
        extras: None
    });

    let pos_acc = ct.root.push(json::Accessor {
        buffer_view: Some(vtx_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(pos.len()),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::F32)),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None
    });

    let idx_view = ct.root.push(json::buffer::View {
        buffer: *ct.buffer_js,
        byte_length: USize64(idx_len),
        byte_offset: Some(USize64(idx_start)),
        byte_stride: Some(json::buffer::Stride(std::mem::size_of::<u32>())),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        extensions: None,
        extras: None
    });

    let idx_acc = ct.root.push(json::Accessor {
        buffer_view: Some(idx_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(num_idx as usize),
        component_type: Valid(json::accessor::GenericComponentType(json::accessor::ComponentType::U32)),
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
        indices: Some(idx_acc),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None
    };
    
    let mesh = ct.root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(format!("{:#010X} {}", ct.key, name)),
        primitives: vec![primitive],
        weights: None
    });

    let node = ct.root.push(json::Node {
        name: Some(name),
        mesh: Some(mesh),
        ..Default::default()
    });

    insert_cache!(ct, &ct.key, node);

    vec![node]
}