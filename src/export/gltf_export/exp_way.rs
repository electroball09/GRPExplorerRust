use crate::{util::transform_yeti_matrix};

use super::*;
use glam::{Mat4, Vec3};
use rgeometry::{algorithms::polygonization::two_opt_moves, data::Point};
use serde::Deserialize;
use serde_json::json;
use log::*;

#[derive(Deserialize, Debug)]
pub struct WayConfig {
    pub capture_ids: HashMap<String, CaptureWay>,
    pub spawn_zone_ids: HashMap<String, SpawnZoneWay>
}

#[derive(Deserialize, Debug)]
pub struct CaptureWay {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct SpawnZoneWay {
    pub name: String,
    pub team: i32,
}

fn bounding_box(points: &[Vec3]) -> Option<(Vec3, Vec3)> {
    if points.is_empty() {
        return None;
    }
    let mut min = points[0];
    let mut max = points[0];

    for &p in points.iter().skip(1) {
        min = min.min(p);
        max = max.max(p);
    }

    Some((min, max)) // (min corner, max corner)
}

fn center(min: Vec3, max: Vec3) -> Vec3 {
    (min + max) * 0.5
}

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
    let mut way_rot_mat = None;
    for key in &ct.bf.object_table[&ct.key].references {
        if ct.bf.is_key_valid(*key) && ct.bf.file_table[key].object_type.is_gao() {
            let gao = ct.bf.object_table[key].archetype.as_game_object().unwrap();

            if way_rot_mat.is_none() {
                way_rot_mat = Some(Mat4::from_quat(gao.rotation()));
            }

            let p = gao.position();
            pos.push(p);
            z = f32::min(z, p.z);
        } else {
            return vec![];
        }
    };
    let way_rot_mat = way_rot_mat.unwrap_or(Mat4::IDENTITY);

    let bounds = bounding_box(&pos).unwrap();
    let center_pos = center(bounds.0, bounds.1);

    for p in &mut pos {
        p.x = center_pos.x - p.x;
        p.y = center_pos.y - p.y;
        p.z = z - center_pos.z;

        // this rotates the whole thing by the inverse amount specified by the tentpole gao
        // then below, we add the rotation to the base gltf node
        *p = (-way_rot_mat).transform_point3(*p);
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
            tangents: None,
            normals: None,
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
            tangents: None,
            normals: None,
            colors: None
        }));
    }

    let extras = ct.bf.object_table.get(&ct.key)
        .and_then(|obj| obj.references.get(0))                   
        .filter(|&&key| ct.bf.is_key_valid(key))                
        .and_then(|&gao_key| ct.bf.object_table.get(&gao_key)
            .and_then(|gao_obj| gao_obj.references.get(1))      
            .map(|&zc_key| zc_key)
        ).and_then(|zc_key| {
            let hex_key = format!("{:#010X}", zc_key);
            ct.way_config.capture_ids.get(&hex_key)
                .map(|capture| {
                    info!("exporting capture point {}", &hex_key);
                    json!({ 
                        "type": "capture", 
                        "name": capture.name 
                    })
                }).or_else(|| {
                    ct.way_config.spawn_zone_ids.get(&hex_key)
                        .map(|spawn_zone| {
                            info!("exporting spawn zone {}", &hex_key);
                            json!({
                                "type": "spawn_zone",
                                "name": spawn_zone.name,
                                "team": spawn_zone.team
                            })
                        })
                })
    });
    
    let mesh = ct.root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: Some(format!("{:#010X} {}", ct.key, name)),
        primitives,
        weights: None
    });

    let way_mat = Mat4::from_translation(center_pos);
    let matrix = transform_yeti_matrix(&(way_mat * way_rot_mat));

    //dbg!(matrix);

    let node = ct.root.push(json::Node {
        matrix: Some(matrix.to_cols_array()),
        name: Some(name),
        mesh: Some(mesh),
        extras: extras.map(|v| serde_json::value::to_raw_value(&v).unwrap()),
        ..Default::default()
    });

    insert_cache!(ct, &ct.key, node);

    vec![node]
}