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

    match ct.key {
        0xA4802C06 | 0xA4801CC1 => transform_alphablend_emissive_shader(&mut material, ct),
        0xA4802C05 | 0x6B800408 | 0xA4801CC0 => transform_alphablend_shader(&mut material, ct),
        _ => {
            match shd_key {
                0xAD00A2AD | 0xAD00F0A4 => transform_invcoloralpha_shader(&mut material, ct),
                0xAD00F62A | 0x0D8151FE | 0xAD027A32 => transform_skybox_shader(&mut material, ct),
                0xAD00E77D | 0xAD00E7B3 | 0x80866577 | 0x8086B1CC | 0x808666DA | 0xAD0149E9 => transform_alphablend_shader(&mut material, ct),
                0xAD00B686 | 0xAD008F52 => transform_alphatest_shader(&mut material, ct),
                _ => transform_standard_shader(&mut material, ct),
            };
        }
    };

    let material = ct.root.push(material);

    insert_cache!(ct, &ct.key, material);

    vec![material]
}

fn transform_standard_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    let mut textures = ct.bf.object_table[&ct.key].references.iter()
        .filter(|key| ct.bf.is_key_valid(**key) && ct.bf.file_table[key].object_type.is_tga())
        .map(|key| *key);

    // this clusterfuck is courtesy of the fact that objects will almost always have a base color, but can have either a normal map or specular map or both or neither
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
            material.pbr_metallic_roughness.base_color_texture = Some(json::texture::Info {
                index: gltf_tga(ct, TextureTransformHint::None)[0],
                tex_coord: 0,
                extensions: Default::default(),
                extras: Default::default()
            });
        });
    }
    if let Some(key) = skey {
        ct_with_key!(ct, key, {
            material.extensions = Some(json::extensions::material::Material {
                specular: Some(json::extensions::material::Specular {
                    specular_color_factor: json::extensions::material::SpecularColorFactor([1.0, 1.0, 1.0]),
                    specular_factor: json::extensions::material::SpecularFactor(1.0),
                    specular_texture: Some(json::texture::Info {
                        index: gltf_tga(ct, TextureTransformHint::ChannelToAlpha(0))[0],
                        tex_coord: 0,
                        extensions: Default::default(),
                        extras: Default::default()
                    }),
                    specular_color_texture: None,
                    extras: Default::default()
                })
            })
        });
    }
    if let Some(key) = nkey {
        ct_with_key!(ct, key, {
            material.normal_texture = Some(json::material::NormalTexture {
                index: gltf_tga(ct, TextureTransformHint::NormalMap)[0],
                tex_coord: 0,
                scale: 1.0,
                extensions: Default::default(),
                extras: Default::default()
            });
        });
    }
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

fn transform_alphatest_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.alpha_mode = Valid(json::material::AlphaMode::Mask);
    material.alpha_cutoff = Some(json::material::AlphaCutoff(0.3));
}

fn transform_skybox_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
    transform_standard_shader(material, ct);

    material.emissive_factor = json::material::EmissiveFactor([1.0, 1.0, 1.0]);
    material.emissive_texture = material.pbr_metallic_roughness.base_color_texture.clone();
}

fn transform_invcoloralpha_shader<'a>(material: &mut json::Material, ct: &'a mut ExportContext) {
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
}