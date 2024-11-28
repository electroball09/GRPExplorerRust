use crate::objects::ObjectArchetype;

use super::*;
use glam::Mat4;
use gltf_json as json;
use json::validation::Checked::Valid;

pub fn gltf_mat<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Material>> {
    check_cache!(ct);

    let name = format!("{:#010X} {}", ct.key, ct.bf.file_table[&ct.key].get_name_ext());

    let mut tga_key = None;
    for rkey in ct.bf.object_table[&ct.key].references.iter() {
        if ct.bf.is_key_valid(*rkey) {
            if let ObjectType::tga = ct.bf.file_table[&rkey].object_type {
                tga_key = Some(rkey);
                break;
            }
        }
    }
    let tga_key = match tga_key {
        Some(key) => *key,
        None => { return Vec::new() }
    };
    
    let old_key = ct.key;
    ct.key = tga_key;
    let texture = super::gltf_tga(ct);
    if texture.len() == 0 {
        return Vec::new();
    }
    let texture = texture[0];
    ct.key = old_key;

    let material = ct.root.push(json::Material {
        alpha_cutoff: None,
        alpha_mode: Valid(json::material::AlphaMode::Opaque),
        double_sided: false,
        name: Some(name),
        pbr_metallic_roughness: json::material::PbrMetallicRoughness {
            base_color_factor: json::material::PbrBaseColorFactor([1.0; 4]),
            base_color_texture: Some(json::texture::Info {
                index: texture,
                tex_coord: 0,
                extensions: Default::default(),
                extras: Default::default()
            }),
            metallic_factor: json::material::StrengthFactor(0.0),
            roughness_factor: json::material::StrengthFactor(1.0),
            metallic_roughness_texture: None,
            extensions: Default::default(),
            extras: Default::default()
        },
        normal_texture: None,
        occlusion_texture: None,
        emissive_factor: json::material::EmissiveFactor([0.0; 3]),
        emissive_texture: None,
        extensions: Default::default(),
        extras: Default::default()
    });

    insert_cache!(ct, &ct.key, material);

    vec![material]
}

pub fn gltf_got<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Node>> {
    check_cache!(ct);

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
            if mats.len() > 0 {
                let mat_idx = {
                    if i >= mats.len() {
                        mats.len() - 1
                    } else {
                        i
                    }
                };
                ct.root.meshes[mesh.value()].primitives[0].material = Some(mats[mat_idx]);
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
    check_cache!(ct);

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
    let blender_matrix = toggle_matrix * yeti_matrix * toggle_matrix;
    //let blender_matrix = gao.matrix;

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

    let node = ct.root.push(json::Node {
        matrix: Some(blender_matrix.to_cols_array()),
        children: Some(nodes),
        name: Some(name),
        ..Default::default()
    });

    insert_cache!(ct, &ct.key, node);

    vec![node]
}