use crate::{objects::{Light, ObjectArchetype}, util::transform_yeti_matrix};

use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;

pub fn gltf_got<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    gltf_export_init!(ct);

    let map = {
        let mut map = HashMap::new();
        let mut curr_mesh = None;
        let mut curr_mats = Vec::new();
        let references = &ct.bf.object_table[&ct.key].references;
        for key in references {
            if !ct.bf.is_key_valid(*key) {
                if *key == references[references.len() - 1] {
                    if let Some(mkey) = curr_mesh {
                        map.insert(mkey, curr_mats);
                        break;
                    }
                }
                continue;
            }

            let objtype = ct.bf.file_table[&key].object_type;
            match objtype {
                ObjectType::msh => {
                    if let Some(mkey) = curr_mesh {
                        map.insert(mkey, curr_mats);
                    }
                    curr_mats = Vec::new();
                    curr_mesh = Some(*key);
                },
                ObjectType::mat => {
                    if let Some(_) = curr_mesh {
                        curr_mats.push(*key);
                    }
                },
                ObjectType::ske => {
                    if let Some(mkey) = curr_mesh {
                        map.insert(mkey, curr_mats);
                    }
                    break;
                },
                _ => { }
            }
        };
        map
    };

    let mut nodes = Vec::new();

    for kv in map.iter() {
        ct_with_key!(ct, *kv.0, {
            let meshes = super::gltf_msh(ct);
            let mats = {
                let mut mats = Vec::new();
                for key in kv.1.iter() {
                    ct.key = *key;
                    let midx = gltf_mat(ct);
                    if midx.len() > 0 {
                        mats.push(midx[0]);
                    }
                }
                mats
            };
            for i in 0..meshes.len() {
                let mesh = &meshes[i];
                for j in 0..ct.root.meshes[mesh.value()].primitives.len() {
                    if mats.len() > 0 {
                        let mat_idx = {
                            if j >= mats.len() {
                                mats.len() - 1
                            } else {
                                j
                            }
                        };
                        ct.root.meshes[mesh.value()].primitives[j].material = Some(mats[mat_idx]);
                    }
                }
    
                nodes.push(ct.root.push(json::Node {
                    mesh: Some(*mesh),
                    name: ct.root.meshes[mesh.value()].name.clone(),
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

    let nodes = {
        let old_key = ct.key;
        let mut nodes = Vec::new();
        for key in &ct.bf.object_table[&ct.key].references {
            if ct.bf.is_key_valid(*key) {
                ct_with_key!(ct, *key, {
                    let colors = match ct.bf.object_table[&ct.key].references.iter().find(|key| ct.bf.is_key_valid(**key) && ct.bf.file_table[key].object_type.is_vxc()) {
                        Some(vxc_key) => {
                            let vxc = ct.bf.object_table[&vxc_key].archetype.as_vertex_colors().unwrap();
                            Some(vxc.colors.clone())
                        },
                        None => None
                    };
                    ct.sub_context.vertex_colors = colors;
                    match ct.bf.file_table[key].object_type {
                        ObjectType::got => nodes.append(&mut gltf_got(ct)),
                        ObjectType::cot => nodes.append(&mut gltf_cot(ct)),
                        _ => { }
                    };
                    ct.sub_context.vertex_colors = None;
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
                            intensity: spot.intensity * ct.options.spot_light_intentisy_multiplier,
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


    if nodes.len() == 0 && light == None && !ct.options.export_empty_gaos && !skip_empty_gaos_if_possible { // skip exporting empty/childless/implementationless gaos
        log::debug!("skipping {} due to no data", name);
        return Vec::new();
    }

    let node = ct.root.push(json::Node {
        matrix: Some(final_matrix.to_cols_array()),
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