use crate::{objects::{Light, ObjectArchetype}, util::transform_yeti_matrix};

use super::*;
use glam::Mat4;
use gltf_json as json;
use json::validation::Checked::Valid;
use log::warn;

pub fn gltf_got<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    let (map, skeleton_key) = {
        let mut map = HashMap::new();
        let mut curr_mesh = None;
        let references = &ct.bf.object_table[&ct.key].references;
        
        // references are arranged, meshes and their corresponding materials are referenced in order
        // the last reference is either a skeleton or 0xFFFFFFFF, only one skeleton per got
        // e.g.
        //  mesh1
        //   mesh1mat1
        //   mesh1mat2
        //  mesh2
        //   mesh2mat1
        //  skeleton

        let mut skeleton_key = None;
        for &key in references {
            if !ct.bf.is_key_valid(key) {
                continue;
            }

            match ct.bf.file_table[&key].object_type {
                ObjectType::msh => {
                    curr_mesh = Some(key);
                    map.insert(key, Vec::new()); 
                },
                ObjectType::mat => {
                    if let Some(mesh_key) = curr_mesh {
                        if let Some(mats) = map.get_mut(&mesh_key) {
                            mats.push(key);
                        }
                    }
                },
                ObjectType::ske => {
                    if let Some(_) = skeleton_key {
                        warn!("multiple skeletons referenced in got {:08X}", ct.key);
                    }

                    skeleton_key = Some(key);
                },
                obj_type => {
                    warn!("weird reference {:?} in got {:08X}", obj_type, ct.key);
                }
            }
        }
        
        (map, skeleton_key)

    };
    
    let skin = skeleton_key.and_then(|key| {
        do_sub_ct!(ct, key, {
            gltf_ske(ct).first().copied()
        })
    });

    let mut nodes = Vec::new();
    for (&mesh_key, mat_keys) in &map {
        do_sub_ct!(ct, mesh_key, {
            let meshes = gltf_msh(ct);
            
            let mats: Vec<_> = mat_keys.iter().filter_map(|&key| {
                do_sub_ct!(ct, key, {
                    gltf_mat(ct).first().copied()
                })
            }).collect();

            let extras_val = ct.sub_context.as_ref()
                .filter(|sc| !sc.capture_visual_for.is_empty())
                .map(|sc| json!({
                    "type": "capture_visual",
                    "for_point": sc.capture_visual_for
                }));

            for mesh in &meshes {
                let mesh_idx = mesh.value();

                if !mats.is_empty() {
                    let max_mat_idx = mats.len() - 1;
                    for (j, prim) in ct.root.meshes[mesh_idx].primitives.iter_mut().enumerate() {
                        let mat_idx = j.min(max_mat_idx); 
                        prim.material = Some(mats[mat_idx]);
                    }
                }

                let extras_raw = extras_val.as_ref()
                    .map(|v| serde_json::value::to_raw_value(v).unwrap());

                let name = ct.root.meshes[mesh_idx].name.clone();
                nodes.push(ct.root.push(json::Node {
                    mesh: Some(*mesh),
                    name,
                    extras: extras_raw,
                    skin,
                    ..Default::default()
                }));
            }
        });
    }

    nodes
}

pub fn gltf_gao<'a>(ct: &'a mut ExportContext, skip_empty_gaos_if_possible: bool) -> Vec<json::Index<json::Node>> {
    gltf_export_init!(ct);

    let gao = match &ct.bf.object_table[&ct.key].archetype {
        ObjectArchetype::GameObject(gao) => gao,
        _ => panic!("wrong object type!")
    };

    let name = ct.bf.file_table[&ct.key].get_name().to_string();
    
    let mut final_matrix = transform_yeti_matrix(&gao.matrix);
    match gao.light { // directional and spot lights are flipped in gltf
        Light::Directional(_) => {
            if ct.options.invert_directional_lights {
                log::debug!("flipping dir. lights");
                final_matrix.z_axis *= -1.0; 
            }
        },
        Light::Spot(_) => {
            if ct.options.invert_spot_lights {
                log::debug!("flipping spot lights");
                final_matrix.z_axis *= -1.0;
            }
        }
        _ => { }
    };

    // gltf validation best practice is to omit transform when transform is identity
    let final_matrix = if final_matrix == Mat4::IDENTITY {
        None
    } else {
        Some(final_matrix)
    };

    let mut script = None;
    for key in &ct.bf.object_table[&ct.key].references {
        if ct.bf.is_key_valid(*key) && ct.bf.file_table[key].object_type.is_zc() {
            script = Some(*key);
            break;
        }
    }

    let mut capture_visual_for = String::new();
    if let Some(key) = script {
        if let Some(data) = ct.export_config.capture_visual_scripts.get(&key.to_string()) {
            capture_visual_for = data.for_point.clone();
        }
    }

    let mut colors = vec!();
    for key in &ct.bf.object_table[&ct.key].references {
        if ct.bf.is_key_valid(*key) && ct.bf.file_table[key].object_type.is_vxc() {
            colors = ct.bf.object_table[key].archetype.as_vertex_colors().unwrap().colors.clone();
            break;
        }
    }

    let nodes = {
        let old_key = ct.key;
        let mut nodes = Vec::new();
        for key in &ct.bf.object_table[&ct.key].references {
            if ct.bf.is_key_valid(*key) {
                do_sub_ct!(ct, *key, {
                    if !colors.is_empty() || !capture_visual_for.is_empty() {
                        ct.sub_context = Some(SubContext {
                            vertex_colors: colors.clone(),
                            capture_visual_for: capture_visual_for.clone()
                        });
                    }
                    match ct.bf.file_table[key].object_type {
                        ObjectType::got => nodes.append(&mut gltf_got(ct)),
                        ObjectType::cot => nodes.append(&mut gltf_cot(ct)),
                        _ => { }
                    };
                });
            }
        }
        ct.key = old_key;
        nodes
    };

    let light = {
        if let Some(ref mut ext) = ct.root.extensions {
            if let Some(ref mut lights) = ext.khr_lights_punctual {
                match &gao.light {
                    Light::Point(point) => {
                        lights.lights.push(json::extensions::scene::khr_lights_punctual::Light {
                            color: [point.color.x, point.color.y, point.color.z],
                            extensions: None,
                            extras: Default::default(),
                            intensity: point.intensity * ct.options.point_light_intensity_multiplier,
                            name: Some(name.clone()),
                            range: Some(point.range * ct.options.point_light_range_multiplier),
                            spot: None,
                            type_: Valid(json::extensions::scene::khr_lights_punctual::Type::Point)
                        });
                        Some(lights.lights.len() as u32 - 1)
                    },
                    Light::Spot(spot) => {
                        lights.lights.push(json::extensions::scene::khr_lights_punctual::Light {
                            color: [spot.color.x, spot.color.y, spot.color.z],
                            extensions: None,
                            extras: Default::default(),
                            intensity: spot.intensity * ct.options.spot_light_intensity_multiplier,
                            name: Some(name.clone()),
                            range: Some(spot.range * ct.options.spot_light_range_multiplier),
                            spot: Some(json::extensions::scene::khr_lights_punctual::Spot {
                                inner_cone_angle: spot.inner_cone_angle / 2.0,
                                outer_cone_angle: spot.outer_cone_angle / 2.0
                            }),
                            type_: Valid(json::extensions::scene::khr_lights_punctual::Type::Spot)
                        });
                        Some(lights.lights.len() as u32 - 1)
                    },
                    Light::Directional(directional) => {
                        lights.lights.push(json::extensions::scene::khr_lights_punctual::Light {
                            color: [directional.color.x, directional.color.y, directional.color.z],
                            extensions: None,
                            extras: Default::default(),
                            intensity: directional.intensity * ct.options.directional_light_intensity_multiplier,
                            name: Some(name.clone()),
                            range: None,
                            spot: None,
                            type_: Valid(json::extensions::scene::khr_lights_punctual::Type::Directional)
                        });
                        Some(lights.lights.len() as u32 - 1)
                    },
                    Light::None => None
                }
            } else {
                log::warn!("no khr_lights_punctual struct!");
                None
            }
        } else {
            log::warn!("no extension struct!");
            None
        }
    };


    if nodes.len() == 0 && light == None && !ct.options.export_empty_gaos && skip_empty_gaos_if_possible { // skip exporting empty/childless/implementationless gaos
        log::debug!("skipping {} due to no data", name);
        return Vec::new();
    }

    let node = ct.root.push(json::Node {
        matrix: final_matrix.and_then(|m| Some(m.to_cols_array())),
        children: Some(nodes),
        name: Some(name),
        extensions: {
            if let Some(light) = light {
                Some(json::extensions::scene::Node {
                    khr_lights_punctual: Some(json::extensions::scene::khr_lights_punctual::KhrLightsPunctual {
                        light: json::Index::new(light)
                    })
                })
            } else {
                Default::default()
            }
        },
        ..Default::default()
    });

    insert_cache!(ct, &ct.key, node);

    vec![node]
}