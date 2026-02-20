use crate::bigfile::util::unwrap_tga_key;

use super::*;
use gltf_json as json;
use json::validation::Checked::Valid;

pub fn gltf_mat<'a>(ct: &'a mut ExportContext) -> Vec<json::Index<json::Material>> {
    gltf_export_init!(ct);

    let name = format!("{:#010X} {}", ct.key, ct.bf.file_table[&ct.key].get_name_ext());
    let shd_key = ct.bf.object_table[&ct.key].references.last().unwrap();

    let mut material = json::Material {
        alpha_cutoff: None,
        alpha_mode: Valid(json::material::AlphaMode::Opaque),
        double_sided: false,
        name: Some(name),
        pbr_metallic_roughness: json::material::PbrMetallicRoughness {
            base_color_factor: json::material::PbrBaseColorFactor([1.0; 4]),
            base_color_texture: None,
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
    };

    // we want to set texture modes per material, and if not, per shader
    let mat_key = &ct.key;

    if ct.export_config.material_shader_type_ids.has_standard(mat_key, shd_key) {
        transform_standard_shader(&mut material, ct);
    } else if ct.export_config.material_shader_type_ids.has_alphatest_key(mat_key, shd_key){ 
        transform_alphatest_shader(&mut material, ct, true);
    } else if ct.export_config.material_shader_type_ids.has_alphablend(mat_key, shd_key) {
        transform_alphablend_shader(&mut material, ct);
    } else if ct.export_config.material_shader_type_ids.has_alphablend_emissive_key(mat_key, shd_key) {
        transform_alphablend_emissive_shader(&mut material, ct);
    } else if ct.export_config.material_shader_type_ids.has_coloralpha(mat_key, shd_key) {
        transform_coloralpha_shader(&mut material, ct, [1.0, 0.817, 0.514, 1.0]);
    } else if ct.export_config.material_shader_type_ids.has_invcoloralpha(mat_key, shd_key) {
        transform_invcoloralpha_shader(&mut material, ct, [0.05, 0.05, 0.05, 1.0]);
    } else if ct.export_config.material_shader_type_ids.has_skybox(mat_key, shd_key) {
        transform_skybox_shader(&mut material, ct);
    } else if ct.export_config.material_shader_type_ids.has_submarine(mat_key, shd_key) {
        transform_submarine_material(&mut material, ct);
    } else {
        transform_standard_shader(&mut material, ct);
    }

    let material = ct.root.push(material);

    insert_cache!(ct, &ct.key, material);

    vec![material]
}

fn load_standard_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext, spec_transform_hint: TextureTransformHint) {
    let mut textures = ct.bf.object_table[&ct.key].references.iter()
        .filter(|key| ct.bf.is_key_valid(**key) && ct.bf.file_table[key].object_type.is_tga())
        .map(|key| *key);

    // this clusterfuck is courtesy of the fact that objects will almost always have a base color, but can have either a normal map or specular map or both or neither, in any order
    let (bkey, skey, nkey) = if let Some(key0) = textures.next() {
        if let Some(key) = textures.next() {
            if let Some(key) = unwrap_tga_key(key, ct.bf) {
                if ct.bf.object_table[&key].archetype.as_texture_metadata().unwrap().meta.as_metadata().unwrap().is_normal_map() {
                    (Some(key0), None, Some(key))
                } else {
                    if let Some(key2) = textures.next() {
                        if let Some(key2) = unwrap_tga_key(key2, ct.bf) {
                            if ct.bf.object_table[&key2].archetype.as_texture_metadata().unwrap().meta.as_metadata().unwrap().is_normal_map() {
                                (Some(key0), Some(key), Some(key2))
                            } else {
                                (Some(key0), Some(key), None)
                            }
                        } else {
                            (Some(key0), Some(key), None)
                        }
                    } else {
                        (Some(key0), Some(key), None)
                    }
                }
            } else {
                (Some(key0), None, None)
            }
        } else {
            (Some(key0), None, None)
        }
    } else {
        (None, None, None)
    };

    //log::info!("{} {:?} {:?} {:?}", textures.count(), bkey, skey, nkey);

    if let Some(key) = bkey {
        do_sub_ct!(ct, key, {
            let textures = gltf_tga(ct, TextureTransformHint::None);
            if textures.len() > 0 {
                material.pbr_metallic_roughness.base_color_texture = Some(json::texture::Info {
                    index: textures[0],
                    tex_coord: 0,
                    extensions: Default::default(),
                    extras: Default::default()
                });
            }
        });
    }
    if let Some(key) = skey {
        do_sub_ct!(ct, key, {
            material.extensions = Some(json::extensions::material::Material {
                specular: Some(json::extensions::material::Specular {
                    specular_color_factor: json::extensions::material::SpecularColorFactor([1.0, 1.0, 1.0]),
                    specular_factor: json::extensions::material::SpecularFactor(1.0),
                    specular_texture: {
                        let textures = gltf_tga(ct, spec_transform_hint);
                        if textures.len() > 0 {
                            Some(json::texture::Info {
                                index: textures[0],
                                tex_coord: 0,
                                extensions: Default::default(),
                                extras: Default::default()
                            })
                        } else {
                            None
                        }
                    },
                    specular_color_texture: None,
                    extras: Default::default()
                }),
                emissive_strength: None
            })
        });
    }
    if let Some(key) = nkey {
        do_sub_ct!(ct, key, {
            let textures = gltf_tga(ct, TextureTransformHint::NormalMap);
            if textures.len() > 0 {
                material.normal_texture = Some(json::material::NormalTexture {
                    index: textures[0],
                    tex_coord: 0,
                    scale: 1.0,
                    extensions: Default::default(),
                    extras: Default::default()
                });
            }
        });
    }
}

fn transform_standard_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    load_standard_shader(material, ct, TextureTransformHint::ChannelToAlpha(0));
}

fn transform_alphablend_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.alpha_mode = Valid(json::material::AlphaMode::Blend);
}

fn transform_alphablend_emissive_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.alpha_mode = Valid(json::material::AlphaMode::Blend);

    material.emissive_factor = json::material::EmissiveFactor([1.0, 1.0, 1.0]);
    material.emissive_texture = material.pbr_metallic_roughness.base_color_texture.clone();
}

fn transform_alphatest_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext, two_sided: bool) {
    transform_standard_shader(material, ct);

    material.alpha_mode = Valid(json::material::AlphaMode::Mask);
    material.alpha_cutoff = Some(json::material::AlphaCutoff(0.3));
    material.double_sided = two_sided;
}

fn transform_skybox_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.emissive_factor = json::material::EmissiveFactor([1.0, 1.0, 1.0]);
    material.emissive_texture = material.pbr_metallic_roughness.base_color_texture.clone();
    material.extensions = Some(json::extensions::material::Material {
        specular: None,
        emissive_strength: Some(json::extensions::material::EmissiveStrength {
            emissive_strength: json::extensions::material::EmissiveStrengthFactor(ct.options.skybox_emissive_multiplier)
        })
    });
}

fn transform_invcoloralpha_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext, base_color_factor: [f32; 4]) {
    let txd_key = match ct.bf.object_table[&ct.key].references.iter()
        .filter(|key| ct.bf.is_key_valid(**key))
        .find(|key| ct.bf.file_table[key].object_type.is_tga()) {
            Some(key) => *key,
            None => return
    };

    #[allow(unused_assignments)]
    let texture = {
        let mut texture = None;
        do_sub_ct!(ct, txd_key, {
            texture = Some(gltf_tga(ct, TextureTransformHint::ChannelToAlphaInvertAndClear(0))[0]);
        });
        texture.expect("texture not loaded correctly?")
    };

    material.alpha_mode = Valid(json::material::AlphaMode::Blend);
    material.pbr_metallic_roughness.base_color_texture = Some(json::texture::Info {
        index: texture,
        tex_coord: 0,
        extensions: Default::default(),
        extras: Default::default()
    });
    material.pbr_metallic_roughness.base_color_factor = json::material::PbrBaseColorFactor(base_color_factor);
}

fn transform_coloralpha_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext, base_color_factor: [f32; 4]) {
    let txd_key = match ct.bf.object_table[&ct.key].references.iter()
        .filter(|key| ct.bf.is_key_valid(**key))
        .find(|key| ct.bf.file_table[key].object_type.is_tga()) {
            Some(key) => *key,
            None => return
    };

    #[allow(unused_assignments)]
    let texture = {
        let mut texture = None;
        do_sub_ct!(ct, txd_key, {
            texture = Some(gltf_tga(ct, TextureTransformHint::ChannelToAlphaInvertAndClear(0))[0]);
        });
        texture.expect("texture not loaded correctly?")
    };

    material.alpha_mode = Valid(json::material::AlphaMode::Blend);
    material.pbr_metallic_roughness.base_color_texture = Some(json::texture::Info {
        index: texture,
        tex_coord: 0,
        extensions: Default::default(),
        extras: Default::default()
    });
    material.pbr_metallic_roughness.base_color_factor = json::material::PbrBaseColorFactor(base_color_factor);
}

fn transform_submarine_material<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.pbr_metallic_roughness.base_color_texture.as_mut().unwrap().tex_coord = 1; 
}