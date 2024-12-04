use crate::objects::{Light, ObjectArchetype, TextureMetaType};

use super::*;
use glam::Mat4;
use gltf_json as json;
use json::validation::Checked::Valid;

struct LoadedTextures {
    pub base_color: Option<json::Index<json::Texture>>,
    pub normal: Option<json::Index<json::Texture>>,
}

fn load_textures<'a>(ct: &'a mut ExportContext) -> LoadedTextures {
    let mut textures = LoadedTextures {
        base_color: None,
        normal: None
    };

    let shd_key = ct.bf.object_table[&ct.key].references.last().unwrap();

    let mut bkey = None;
    let mut nkey = None;
    for rkey in ct.bf.object_table[&ct.key].references.iter() {
        if ct.bf.is_key_valid(*rkey) {
            if let ObjectArchetype::TextureMetadata(tga) = &ct.bf.object_table[&rkey].archetype {
                let (rkey, meta) = match tga.meta {
                    TextureMetaType::Metadata(meta) => (*rkey, meta),
                    TextureMetaType::Passthrough => {
                        let rkey = ct.bf.object_table[&rkey].references[0];
                        if let ObjectArchetype::TextureMetadata(tga) = &ct.bf.object_table[&rkey].archetype {
                            match tga.meta {
                                TextureMetaType::Metadata(meta) => (rkey, meta),
                                _ => continue
                            }
                        } else {
                            continue
                        }
                    },
                    TextureMetaType::None => continue
                };

                if meta.is_normal_map() {
                    nkey = Some(rkey);
                } else {
                    if bkey == None {
                        bkey = Some(rkey);
                    }
                }
            }
        };
    };

    if let Some(bkey) = bkey {
        ct_with_key!(ct, bkey, {
            let texs = gltf_tga(ct, TextureTransformHint::None);
            if texs.len() != 0 {
                textures.base_color = Some(texs[0]);
            }
        });
    }

    if let Some(nkey) = nkey {
        ct_with_key!(ct, nkey, {
            let texs = gltf_tga(ct, TextureTransformHint::NormalMap);
            if texs.len() != 0 {
                textures.normal = Some(texs[0]);
            }
        });
    }

    textures
}

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

    let old_key = ct.key;
    for kv in map.iter() {
        ct.key = *kv.0;
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
                ..Default::default()
            }));
        }
    }
    ct.key = old_key;

    nodes
}

pub fn gltf_gao<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    gltf_export_init!(ct);

    let gao = match &ct.bf.object_table[&ct.key].archetype {
        ObjectArchetype::GameObject(gao) => gao,
        _ => panic!("wrong object type!")
    };

    let name = ct.bf.file_table[&ct.key].get_name().to_string();
    
    // https://stackoverflow.com/questions/1263072/changing-a-matrix-from-right-handed-to-left-handed-coordinate-system
    let yeti_matrix = gao.matrix;
    let toggle_matrix = Mat4 {
        x_axis: [-1.0, 0.0, 0.0, 0.0].into(),
        y_axis: [0.0, 0.0, 1.0, 0.0].into(),
        z_axis: [0.0, 1.0, 0.0, 0.0].into(),
        w_axis: [0.0, 0.0, 0.0, 1.0].into(),
    };
    let mut blender_matrix = toggle_matrix * yeti_matrix * toggle_matrix; // this switches y and z coords and flips x coord, same as in exp_mesh.rs
    match gao.light {
        Light::Directional(_) | Light::Spot(_) => {
            blender_matrix.z_axis *= -1.0; // directional and spot lights are flipped in gltf
        },
        _ => { }
    };

    let nodes = {
        let old_key = ct.key;
        let mut nodes = Vec::new();
        for key in &ct.bf.object_table[&ct.key].references {
            if ct.bf.is_key_valid(*key) {
                if let ObjectType::got = ct.bf.file_table[key].object_type {
                    ct.key = *key;
                    nodes = gltf_got(ct);
                }
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
                            intensity: point.intensity * 10000.0,
                            name: Some(name.clone()),
                            range: Some(point.range * 1000.0),
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
                            intensity: spot.intensity * 10000.0,
                            name: Some(name.clone()),
                            range: Some(spot.range * 1000.0),
                            spot: Some(json::extensions::scene::khr_lights_punctual::Spot {
                                inner_cone_angle: spot.inner_cone_angle,
                                outer_cone_angle: spot.outer_cone_angle
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
                            intensity: directional.intensity * 10000.0,
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


    if nodes.len() == 0 && light == None { // skip exporting empty/childless/implementationless gaos
        log::warn!("skipping {} due to no data", name);
        return Vec::new();
    }

    if let Some(ref light) = light {
        log::debug!("light idx: {}", light);
    }

    let node = ct.root.push(json::Node {
        matrix: Some(blender_matrix.to_cols_array()),
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