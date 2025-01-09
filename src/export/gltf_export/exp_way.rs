use crate::util::transform_yeti_matrix;

use super::*;
use glam::{Mat4, Vec3};
use gltf_json as json;
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
    for key in &ct.bf.object_table[&ct.key].references {
        if ct.bf.is_key_valid(*key) && ct.bf.file_table[key].object_type.is_gao() {
            let p = ct.bf.object_table[key].archetype.as_game_object().unwrap().position();
            pos.push(p);
            z = f32::min(z, p.z);
        } else {
            return vec![];
        }
    };

    let center_pos = pos.iter().sum::<Vec3>() / (pos.len() as f32);

    for p in &mut pos {
        p.x = center_pos.x - p.x;
        p.y = center_pos.y - p.y;
        p.z = z - center_pos.z;
    }

    // this will probably fail in edge cases but it's good enough for the game i think
    let is_clockwise = {
        let p0 = pos[0];
        let p1 = pos[1];
        
        let d0 = (p0 - center_pos).normalize();
        let d1 = (p1 - center_pos).normalize();

        d0.cross(d1).z >= 0.0
    };

    if !is_clockwise {
        pos = pos.iter().map(|p| *p).rev().collect();
    }

    let points: Vec<Point<f32>> = pos.iter().map(|v| Point::<f32>::new([v.x.into(), v.y.into()])).collect();
    let poly = two_opt_moves(points, &mut rand::thread_rng()).expect("uh oh");

    let mut indices: Vec<u32> = poly.triangulate().flat_map(|f| {
        [f.0.point_id().usize() as u32, f.1.point_id().usize() as u32, f.2.point_id().usize() as u32]
    }).collect();
    
    let mut primitives = Vec::new();

    if ct.options.way_export_strategy.should_export_triangles() {
        primitives.push(write_primitive(ct, GltfPrimitiveBuild {
            pos_pre_transformed: &pos,
            indices: &indices,
            uv0: None,
            uv1: None,
            colors: None
        }));
    }

    if ct.options.way_export_strategy.should_export_extrude() {
        let num_pos = pos.len() as u32;
        let side_pos_start = num_pos;
        indices.append(&mut indices.chunks_exact(3).flat_map(|chk| { // the top of the extruded mesh
            [chk[0], chk[2], chk[1]]
        }).map(|idx| idx + side_pos_start + num_pos).collect());
        indices.append(&mut pos.iter().enumerate().flat_map(|(i, _)| { // the sides of the extruded mesh
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
    
            [i, next1, prev1, i, next2, next1]
        }).collect());
        let mut top_pos: Vec<Vec3> = pos.iter().map(|p| Vec3::new(p.x, p.y, p.z + 5.0)).collect();
        pos.append(&mut pos.clone());
        pos.append(&mut top_pos.clone());
        pos.append(&mut top_pos);

        primitives.push(write_primitive(ct, GltfPrimitiveBuild {
            pos_pre_transformed: &pos,
            indices: &indices,
            uv0: None,
            uv1: None,
            colors: None
        }));
    }
    
    let mesh = ct.root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(format!("{:#010X} {}", ct.key, name)),
        primitives,
        weights: None
    });

    let matrix = transform_yeti_matrix(&Mat4::from_translation(center_pos));

    let node = ct.root.push(json::Node {
        matrix: Some(matrix.to_cols_array()),
        name: Some(name),
        mesh: Some(mesh),
        ..Default::default()
    });

    insert_cache!(ct, &ct.key, node);

    vec![node]
}