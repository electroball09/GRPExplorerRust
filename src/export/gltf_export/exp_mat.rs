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
    match ct.key.into() {
        0xDF729407 => transform_alphatest_shader(&mut material, ct, true),
        0xA4802C06 | 0xA4801CC1 | 0xA480258D | 0xA3810A9D | 0xA4801AEE | 0xA4802A74 | 0xA4802AE7 => transform_alphablend_emissive_shader(&mut material, ct),
        0xA4802C05 | 0x6B800408 | 0xA4801CC0 | 0xA480258C | 0xA3810A9C | 0xA4801AEC | 0xA4802A75 | 0xA4802AE6 => transform_alphablend_shader(&mut material, ct),
        0x6F800200 => transform_submarine_material(&mut material, ct),
        0xdf729421 | 0x21041136 | 0x21041137 | 0x6F87E15B => transform_standard_shader(&mut material, ct),
        _ => {
            match (*shd_key).into() {
                0xAD00A2AD => transform_invcoloralpha_shader(&mut material, ct, [0.05, 0.05, 0.05, 1.0]), // decals like wall grime and footprints
                0xAD00F0A4 | 0x11800B53 => transform_coloralpha_shader(&mut material, ct, [1.0, 0.817, 0.514, 1.0], 0.1), // godrays
                0xAD00F62A | 0x0D8151FE | 0xAD027A32 | 0xAD027A1C => transform_skybox_shader(&mut material, ct), // skyboxes
                0xAD00E77D | 0xAD00E7B3 | 0x80866577 | 0x8086B1CC | 0x808666DA | 0xAD0149E9 | 0x61031807 | 0xAD02A99B | 0x0D815201 | 0x0D815CF7 | 0xAD00F51D | 0xAD008934 |
                 0xAD008825 | 0xAD028F84 | 0xAD00892E 
                     => transform_alphablend_shader(&mut material, ct),
                0xAD00B686 | 0xAD008F52  | 0xAD010D7A | 0xAD00961F | 0xAD00994C | 0xAD028FB7 | 0x9203E4C5 | 0xB0800098
                    => transform_alphatest_shader(&mut material, ct, false),
                _ => transform_standard_shader(&mut material, ct),
            };
        }
    };

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
        ct_with_key!(ct, key, {
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
        ct_with_key!(ct, key, {
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
        ct_with_key!(ct, key, {
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

    let mut texture = None;
    ct_with_key!(ct, txd_key, {
        texture = Some(gltf_tga(ct, TextureTransformHint::ChannelToAlphaInvertAndClear(0))[0]);
    });

    material.alpha_mode = Valid(json::material::AlphaMode::Blend);
    material.pbr_metallic_roughness.base_color_texture = Some(json::texture::Info {
        index: texture.unwrap(),
        tex_coord: 0,
        extensions: Default::default(),
        extras: Default::default()
    });
    material.pbr_metallic_roughness.base_color_factor = json::material::PbrBaseColorFactor(base_color_factor);
}

fn transform_coloralpha_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext, base_color_factor: [f32; 4], alpha_scale: f32) {
    let txd_key = match ct.bf.object_table[&ct.key].references.iter()
        .filter(|key| ct.bf.is_key_valid(**key))
        .find(|key| ct.bf.file_table[key].object_type.is_tga()) {
            Some(key) => *key,
            None => return
    };

    let mut texture = None;
    ct_with_key!(ct, txd_key, {
        texture = Some(gltf_tga(ct, TextureTransformHint::ChannelToAlphaAndClear(0, alpha_scale))[0]);
    });

    material.alpha_mode = Valid(json::material::AlphaMode::Blend);
    material.pbr_metallic_roughness.base_color_texture = Some(json::texture::Info {
        index: texture.unwrap(),
        tex_coord: 0,
        extensions: Default::default(),
        extras: Default::default()
    });
    material.pbr_metallic_roughness.base_color_factor = json::material::PbrBaseColorFactor(base_color_factor);
}

fn transform_submarine_material<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.pbr_metallic_roughness.base_color_texture.as_mut().unwrap().tex_coord = 1; // wtf ubisoft
}