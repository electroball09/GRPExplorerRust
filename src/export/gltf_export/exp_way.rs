use super::*;
use glam::Vec3;
use gltf_json as json;
use json::validation::Checked::Valid;
use rgeometry::{algorithms::polygonization::two_opt_moves, data::Point};

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

    if ct.options.way_export_strategy.is_none() {
        return vec![];
    }

    let mut pos = Vec::new();
    let mut z = 9999999.0;
    let mut is_snd_way = false;
    for key in &ct.bf.object_table[&ct.key].references {
        if ct.bf.is_key_valid(*key) && ct.bf.file_table[key].object_type.is_gao() {
            let p = ct.bf.object_table[key].archetype.as_game_object().unwrap().position();
            pos.push(p);
            z = f32::min(z, p.z);
        } else {
            is_snd_way = true;
        }
    };
    if is_snd_way {
        return vec![];
    }

    for p in &mut pos {
        p.z = z;
    }

    let points: Vec<Point<f32>> = pos.iter().map(|v| Point::<f32>::new([v.x.into(), v.y.into()])).collect();
    let poly = two_opt_moves(points, &mut rand::thread_rng()).expect("uh oh");

    let mut indices: Vec<u32> = poly.triangulate().flat_map(|f| {
        let (f0, f1, f2) = if ct.options.way_export_strategy.is_extrude() {
            (f.0.point_id().usize() as u32, f.2.point_id().usize() as u32, f.1.point_id().usize() as u32) // swap face 2 and 1 to flip normal
        } else {
            (f.0.point_id().usize() as u32, f.1.point_id().usize() as u32, f.2.point_id().usize() as u32)
        };
        [f0, f1, f2]
    }).collect();

    //just use the triangulated mesh

    if ct.options.way_export_strategy.is_extrude() {
        let num_pos = pos.len() as u32;
        let side_pos_start = 0;
        indices.append(&mut indices.chunks_exact(3).flat_map(|chk| {
            [chk[0], chk[2], chk[1]]
        }).map(|idx| idx + side_pos_start + num_pos).collect()); // the top of the extruded mesh
        indices.append(&mut pos.iter().rev().enumerate().flat_map(|(i, _)| {
            let i = side_pos_start + i as u32;
            let next1 = i + num_pos;
            let prev1 =  match i {
                0 => side_pos_start + num_pos * 2 - 1,
                _ => next1 - 1
            };
            let next2 = if i == num_pos - 1 {
                side_pos_start
            } else {
                i + 1
            };
    
            //[i, prev1, next1, i, next1, next2]
            [i, next1, prev1, i, next2, next1]
        }).collect());
        let mut top_pos: Vec<Vec3> = pos.iter().map(|p| Vec3::new(p.x, p.y, z + 5.0)).collect();
        // pos.append(&mut pos.clone());
        // pos.append(&mut top_pos.clone());
        pos.append(&mut top_pos);
    }

    let vtx_start = ct.cursor.position();

    for p in &pos {
        ct.cursor.write_f32::<ENDIAN>(-p.x).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.z).expect("write error");
        ct.cursor.write_f32::<ENDIAN>(p.y).expect("write error");
    }

    let vtx_len = ct.cursor.position() - vtx_start;
    let idx_start = ct.cursor.position();

    for idx in &indices {
        ct.cursor.write_u32::<ENDIAN>(*idx).expect("write error");
    }

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
        count: USize64::from(indices.len()),
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